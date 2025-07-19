/*
 * Plugin to handle level selection
 */
use bevy::prelude::*;

use super::{GameState, despawn_screen};

/*
 * Plugin defintion
 */
pub fn selection_plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(GameState::Start), selection_setup)
        // While in this state, run the `countdown` system
        //.add_systems(Update, countdown.run_if(in_state(GameState::Selection)))
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(
            OnExit(GameState::Start),
            despawn_screen::<OnSelectionScreen>,
        );
}

/*
 * ================================================================================================================
 * START - Plugin Systems
 * ================================================================================================================
 */

fn selection_setup() {}

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
 * Marks an entity as part of the selection screen
 */
#[derive(Component)]
struct OnSelectionScreen;

/*
 * ================================================================================================================
 * END - Plugin Components
 * ================================================================================================================
 */
