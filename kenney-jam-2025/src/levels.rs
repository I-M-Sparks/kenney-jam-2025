/*
 * Plugin to handle level setup & events
 */
use bevy::prelude::*;

use super::{GameState, despawn_screen};

mod level_1;
mod level_2;

/*
 * Plugin defintion
 */
pub fn levels_plugin(app: &mut App) {
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(GameState::Levels), levels_setup)
        // While in this state, run the `countdown` system
        //.add_systems(Update, countdown.run_if(in_state(GameState::Levels)))
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
 * Read selected Level Ressource and trigger respective level to set up
 */
fn levels_setup() {
    // TODO read level resource and load respective level
}

fn handle_right_mouse_press_event() {
    // TODO return to level selection
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
struct OnLevelsScreen;

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
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum SelectedLevel {
    #[default]
    Level1,
    Level2,
}
/*
 * ================================================================================================================
 * END - States
 * ================================================================================================================
 */
