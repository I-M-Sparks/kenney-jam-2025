/*
 * Plugin to handle level setup & events
 */
use bevy::prelude::*;

use super::{BallDestroyedEvent, GameState, RightMousePressEvent, despawn_screen};

use crate::selection::LevelSelectedEvent;

mod level_1;
mod level_2;

/*
 * Plugin defintion
 */
pub fn levels_plugin(app: &mut App) {
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(GameState::Levels), level_setup)
        // While in this state, run the `countdown` system
        .add_systems(
            Update,
            handle_right_mouse_press_event.run_if(in_state(GameState::Levels)),
        )
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(OnExit(GameState::Levels), despawn_screen::<OnLevelsScreen>);
}

/*
 * ================================================================================================================
 * START - Plugin Systems
 * ================================================================================================================
 */

/*
 * TODO
 */
fn level_setup(
    // Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // Events
    mut level_selected_evr: EventReader<LevelSelectedEvent>,
) {
    debug!("Setting up Level");
    for event in level_selected_evr.read() {
        match event.selected_level {
            SelectedLevel::Level1 => {
                level_1::spawn_level(&mut commands, &asset_server);
            }

            SelectedLevel::Level2 => {
                level_2::spawn_level(&mut commands);
            }
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
}
/*
 * ================================================================================================================
 * END - States
 * ================================================================================================================
 */
