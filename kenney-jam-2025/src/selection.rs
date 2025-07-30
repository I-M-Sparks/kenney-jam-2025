/*
 * Plugin to handle level selection
 */
use super::{
    AddCollider, BallDestroyedEvent, ColliderType, GameState, Player, PlayerBall, despawn_screen,
};
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::levels::SelectedLevel;

/*
 * Plugin defintion
 */
pub fn selection_plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(GameState::Selection), selection_setup)
        // While in this state, handle level selection (by handling collisions between player ball and level selector)
        .add_systems(
            Update,
            handle_collision_player_ball_and_selection_block.run_if(in_state(GameState::Selection)),
        )
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(
            OnExit(GameState::Selection),
            despawn_screen::<OnSelectionScreen>,
        )
        .add_event::<LevelSelectedEvent>();
}

/*
 * ================================================================================================================
 * START - Plugin Systems
 * ================================================================================================================
 */

fn selection_setup(
    // Singles
    player: Option<Single<&Player>>,
    //Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    trace!("Setting up Selection screen");

    if let Some(player) = player {
        let player = player.into_inner();
        spawn_selection_block(&player, SelectedLevel::Level1, &mut commands, &asset_server);
        spawn_selection_block(&player, SelectedLevel::Level2, &mut commands, &asset_server);
    } else {
        trace!("No player-entity found, using mock player for setup");
        let player = Player {
            highest_selectable_level: SelectedLevel::Level1,
        };
        spawn_selection_block(&player, SelectedLevel::Level1, &mut commands, &asset_server);
        spawn_selection_block(&player, SelectedLevel::Level2, &mut commands, &asset_server);
    }
}

fn handle_collision_player_ball_and_selection_block(
    // Singles
    player_ball: Single<Entity, With<PlayerBall>>,
    // Globals
    mut game_state: ResMut<NextState<GameState>>,
    // Collisions
    collisions: Collisions,
    //Queries
    level_selector_blocks: Query<(Entity, &LevelSelectorBlock), Without<PlayerBall>>,
    level_selector_outlines: Query<&LevelSelectorOutline, Without<PlayerBall>>,
    // Events
    mut ball_destroyed_evw: EventWriter<BallDestroyedEvent>,
    mut level_selected_evw: EventWriter<LevelSelectedEvent>,
) {
    // TODO
    let player_ball = player_ball.into_inner();

    for contact_pair in collisions.iter() {
        // if a collision between the player ball and a level selector occurred
        if (contact_pair.collider1.eq(&player_ball) || contact_pair.collider2.eq(&player_ball))
            && (level_selector_blocks.contains(contact_pair.collider1)
                || level_selector_blocks.contains(contact_pair.collider2))
        {
            let level_selector_block_entity = if contact_pair.collider1.eq(&player_ball) {
                contact_pair.collider2
            } else {
                contact_pair.collider1
            };

            let level_selector_block = level_selector_blocks
                .get(level_selector_block_entity)
                .ok()
                .unwrap()
                .1;

            ball_destroyed_evw.write(BallDestroyedEvent);
            level_selected_evw.write(LevelSelectedEvent {
                selected_level: level_selector_block.selected_level,
            });
            game_state.set(GameState::Levels);
            debug!("Selected {:?}", level_selector_block.selected_level);
            break;
        }
    }

    for contact_pair in collisions.iter() {
        // if a collision between the player ball and a level selector occurred
        if (contact_pair.collider1.eq(&player_ball) || contact_pair.collider2.eq(&player_ball))
            && (level_selector_outlines.contains(contact_pair.collider1)
                || level_selector_outlines.contains(contact_pair.collider2))
        {
            ball_destroyed_evw.write(BallDestroyedEvent);
            debug!("Level could not be selected; not yet unlocked");
            break;
        }
    }

    // TODO read slection data from selection block and write to... Resource? how to determine which level should now run? can I just use a second GameState?
    // TODO trigger GameState change to play
}

/*
 * ================================================================================================================
 * END - Plugin Systems
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Plugin functions
 * ================================================================================================================
 */
fn spawn_selection_block(
    // Parameters
    player: &Player,
    selected_level: SelectedLevel,
    //Globals
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let mut outline_sprite_selectable = Sprite::from_image(asset_server.load("selectorA.png"));
    outline_sprite_selectable.custom_size = Some(Vec2::new(55.0, 55.0));
    let mut outline_sprite_not_selectable = Sprite::from_image(asset_server.load("selectorB.png"));
    outline_sprite_not_selectable.custom_size = Some(Vec2::new(55.0, 55.0));

    let spawn_block = |selected_level,
                       transform: Transform,
                       asset_path,
                       outline_sprite: Sprite,
                       outline_collider_type: ColliderType,
                       commands: &mut Commands,
                       asset_server: &Res<AssetServer>| {
        commands.spawn(LevelSelectorBlockBundle {
            marker: LevelSelectorBlock {
                selected_level: selected_level,
            },
            screen_marker: OnSelectionScreen,
            add_collider: AddCollider {
                collider_scale: 1.0,
                collider_type: if outline_collider_type == ColliderType::None {
                    ColliderType::RegularPolygon
                } else {
                    ColliderType::None
                },
            },
            sprite: Sprite::from_image(asset_server.load(asset_path)),
            transform: transform.clone(),
            rigid_body: RigidBody::Static,
        });

        commands.spawn(LevelSelectorOutlineBundle {
            marker: LevelSelectorOutline,
            screen_marker: OnSelectionScreen,
            add_collider: AddCollider {
                collider_scale: 1.0,
                collider_type: outline_collider_type,
            },
            sprite: outline_sprite.clone(),
            transform: transform.clone(),
            rigid_body: RigidBody::Static,
        });
    };

    match selected_level {
        SelectedLevel::Level1 => {
            let level_1_selector_transform = Transform::from_xyz(-450.0, 100.0, 0.0);
            let asset_path = "element_grey_polygon_glossy.png";

            spawn_block(
                selected_level,
                level_1_selector_transform,
                asset_path,
                outline_sprite_selectable.clone(),
                ColliderType::None,
                commands,
                asset_server,
            );
        }

        SelectedLevel::Level2 => {
            let level_2_selector_transform = Transform::from_xyz(-300.0, 100.0, 0.0);
            let asset_path = "element_blue_polygon_glossy.png";
            let outline_sprite = if player.highest_selectable_level >= SelectedLevel::Level2 {
                outline_sprite_selectable.clone()
            } else {
                outline_sprite_not_selectable.clone()
            };
            let outline_collider_type = if player.highest_selectable_level >= SelectedLevel::Level2
            {
                ColliderType::None
            } else {
                ColliderType::Rectangle
            };

            spawn_block(
                selected_level,
                level_2_selector_transform,
                asset_path,
                outline_sprite,
                outline_collider_type,
                commands,
                asset_server,
            );
        }

        _ => todo!(),
    }
}
/*
 * ================================================================================================================
 * END - Plugin functions
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Plugin Components
 * ================================================================================================================
 */

/*
 * Marks an entity as part of the selection screen
 */
#[derive(Component)]
struct OnSelectionScreen;

/*
 * Markes the **** TODO
 */
#[derive(Component)]
struct LevelSelectorOutline;

/*
 * Markes the **** TODO
 */
#[derive(Component)]
struct SelectorHighlighter;

/*
 * Markes the **** TODO
 */
#[derive(Component)]
struct LevelSelectorBlock {
    selected_level: SelectedLevel,
}

/*
 * ================================================================================================================
 * END - Plugin Components
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Plugin Bundles
 * ================================================================================================================
 */
#[derive(Bundle)]
struct LevelSelectorOutlineBundle {
    marker: LevelSelectorOutline,
    screen_marker: OnSelectionScreen,
    add_collider: AddCollider,
    sprite: Sprite,
    transform: Transform,
    rigid_body: RigidBody,
}

#[derive(Bundle)]
struct LevelSelectorBlockBundle {
    marker: LevelSelectorBlock,
    screen_marker: OnSelectionScreen,
    add_collider: AddCollider,
    sprite: Sprite,
    transform: Transform,
    rigid_body: RigidBody,
}
/*
 * ================================================================================================================
 * END - Plugin Bundles
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * Start - Plugin Events
 * ================================================================================================================
 */
#[derive(Event)]
pub struct LevelSelectedEvent {
    pub selected_level: SelectedLevel,
}
/*
 * ================================================================================================================
 * END - Plugin Events
 * ================================================================================================================
 */
