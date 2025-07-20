/*
 * Plugin to handle level selection
 */
use super::{AddCollider, ColliderType, GameState, Player, despawn_screen};
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
        // While in this state, run the `countdown` system
        //.add_systems(Update, countdown.run_if(in_state(GameState::Selection)))
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(
            OnExit(GameState::Selection),
            despawn_screen::<OnSelectionScreen>,
        );
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

fn handle_collision_player_ball_and_selection_block() {
    // TODO
    // TODO detect collision
    // TODO read data from selection block and write to... Resource?
    // TODO trigger GameState change
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
                collider_type: ColliderType::RegularPolygon,
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
