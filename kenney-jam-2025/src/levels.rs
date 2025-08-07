/*
 * Plugin to handle level setup & events
 */
use avian2d::prelude::*;
use bevy::prelude::*;

use super::{BallDestroyedEvent, GameState, PlayerBall, RightMousePressEvent, despawn_screen};

use crate::selection::LevelSelectedEvent;

use crate::level_elements::{DestructibleElement, ElementDurability};
use crate::{Player, PowerLevel};

mod level_1;
mod level_2;
mod level_3;
mod level_4;
mod level_5;

/*
 * Plugin defintion
 */
pub fn levels_plugin(app: &mut App) {
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(GameState::Levels), level_setup)
        // While in this state, run the `countdown` system
        .add_systems(
            First,
            handle_event_block_destroyed.run_if(in_state(GameState::Levels)),
        )
        .add_systems(
            Update,
            (
                handle_right_mouse_press_event.run_if(in_state(GameState::Levels)),
                handle_collision_player_ball_with_destructible_element
                    .run_if(in_state(GameState::Levels)),
            ),
        )
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(OnExit(GameState::Levels), despawn_screen::<OnLevelsScreen>)
        // Events
        .add_event::<BlockDestroyedEvent>()
        // Resources
        .insert_resource(LastSelectedLevel {
            selected_level: SelectedLevel::Level1,
        });
}

/*
 * ================================================================================================================
 * START - Plugin Systems
 * ================================================================================================================
 */

fn level_setup(
    // Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut last_selected_level: ResMut<LastSelectedLevel>,
    // Events
    mut level_selected_evr: EventReader<LevelSelectedEvent>,
) {
    debug!("Setting up Level");

    for event in level_selected_evr.read() {
        last_selected_level.selected_level = event.selected_level;

        match event.selected_level {
            SelectedLevel::Level1 => {
                level_1::spawn_level(&mut commands, &asset_server);
            }

            SelectedLevel::Level2 => {
                level_2::spawn_level(&mut commands, &asset_server);
            }

            SelectedLevel::Level3 => {
                level_3::spawn_level(&mut commands, &asset_server);
            }

            SelectedLevel::Level4 => {
                level_4::spawn_level(&mut commands, &asset_server);
            }

            SelectedLevel::Level5 => {
                level_5::spawn_level(&mut commands, &asset_server);
            }

            _ => todo!(),
        }
    }
}

fn handle_collision_player_ball_with_destructible_element(
    // Singles
    player_ball: Single<(Entity, &PlayerBall)>,
    // Globals
    mut commands: Commands,
    // Collisions
    collisions: Collisions,
    // Events
    mut block_destroyed_evw: EventWriter<BlockDestroyedEvent>,
    // Queries
    destructible_elements: Query<&DestructibleElement>,
) {
    let (player_ball_entity, player_ball) = player_ball.into_inner();

    for contact_pair in collisions.iter() {
        // if one of the colliders is the player ball and one of them is the bottom collider
        if (contact_pair.collider1.eq(&player_ball_entity)
            || contact_pair.collider2.eq(&player_ball_entity))
            && (destructible_elements.contains(contact_pair.collider1)
                || destructible_elements.contains(contact_pair.collider2))
        {
            trace!("Ball hit a block");
            let mut element_entity;
            if contact_pair.collider1.eq(&player_ball_entity) {
                element_entity = contact_pair.collider2;
            } else {
                element_entity = contact_pair.collider1;
            }

            if let Some(element) = destructible_elements.get(element_entity).ok() {
                if ball_destroys_element(&player_ball.power_level, &element.element_durability) {
                    commands.entity(element_entity).despawn();
                    block_destroyed_evw.write(BlockDestroyedEvent);
                    debug!("Block destroyed, event fired");
                }
            }
        }
    }
}

fn ball_destroys_element(
    player_ball_power_level: &PowerLevel,
    element_durability: &ElementDurability,
) -> bool {
    match player_ball_power_level {
        PowerLevel::Lowest => {
            return *element_durability == ElementDurability::Lowest;
        }
        PowerLevel::Low => {
            return *element_durability <= ElementDurability::Low;
        }
        PowerLevel::Medium => {
            return *element_durability <= ElementDurability::Medium;
        }
        PowerLevel::High => {
            return *element_durability <= ElementDurability::High;
        }
        PowerLevel::Highest => {
            return *element_durability <= ElementDurability::Highest;
        }
    }
}

fn handle_right_mouse_press_event(
    // Globals
    mut game_state: ResMut<NextState<GameState>>,
    // Events
    mut right_mouse_press_evr: EventReader<RightMousePressEvent>,
    mut ball_destroyed_evw: EventWriter<BallDestroyedEvent>,
) {
    for _event in right_mouse_press_evr.read() {
        debug!("Right Mouse press event received; returning to level selection");
        ball_destroyed_evw.write(BallDestroyedEvent);
        game_state.set(GameState::Selection);
        break;
    }
}

fn handle_event_block_destroyed(
    // Singles
    mut player: Single<&mut Player>,
    // Globals
    last_selected_level: Res<LastSelectedLevel>,
    mut game_state: ResMut<NextState<GameState>>,
    mut ball_destroyed_evw: EventWriter<BallDestroyedEvent>,
    // Events
    mut block_destroyed_evr: EventReader<BlockDestroyedEvent>,
    // Queries
    destructible_elements: Query<&DestructibleElement>,
) {
    let mut player = player.into_inner();
    for event in block_destroyed_evr.read() {
        debug!("Block destroyed event read");
        if destructible_elements.is_empty() {
            debug!("all elements destroyed");
            player.highest_selectable_level = unlock_next_level(
                player.highest_selectable_level,
                last_selected_level.selected_level,
            );
            // destroy ball and return to level selection
            ball_destroyed_evw.write(BallDestroyedEvent);
            game_state.set(GameState::Selection);
            break;
        } else {
            debug!("more elements to be destroyed");
        }
    }
}

/*
 * Return the new highest selectable level
 * Only unlocks the next level if the player has actually cleared the highest selectable level
 */
fn unlock_next_level(
    // Parameters
    current_highest_selectable_level: SelectedLevel,
    current_level: SelectedLevel,
) -> SelectedLevel {
    if current_level == current_highest_selectable_level {
        match current_level {
            SelectedLevel::Level1 => SelectedLevel::Level2,
            SelectedLevel::Level2 => SelectedLevel::Level3,
            SelectedLevel::Level3 => SelectedLevel::Level4,
            SelectedLevel::Level4 => SelectedLevel::Level5,
            _ => todo!(),
        }
    } else {
        current_highest_selectable_level
    }
}

/*
 * ================================================================================================================
 * END - Plugin Systems
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Plugin Components
 * ================================================================================================================
 */

/*
 * Marks an entity as part of the levels screen
 */
#[derive(Component)]
pub struct OnLevelsScreen;

/*
 * ================================================================================================================
 * END - Plugin Components
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - States
 * ================================================================================================================
 */

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, States)]
pub enum SelectedLevel {
    #[default]
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}
/*
 * ================================================================================================================
 * END - States
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Events
 * ================================================================================================================
 */

#[derive(Event)]
struct BlockDestroyedEvent;

/*
 * ================================================================================================================
 * END - Events
 * ================================================================================================================
 */

#[derive(Resource)]
struct LastSelectedLevel {
    selected_level: SelectedLevel,
}
