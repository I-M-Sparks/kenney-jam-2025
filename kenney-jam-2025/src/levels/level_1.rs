use bevy::prelude::*;

use crate::level_elements::*;

pub fn spawn_level(
    // Globals
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    debug!("spawning level 1");

    for index in 0..6 {
        let x_coordinate_left_column = -350.0;
        let horizontal_offset = 150.0; // in pixels
        let y_position_bottom_block = -100.0;
        let vertical_offset = 75.0; // in pixels

        spawn_square_block_column(
            x_coordinate_left_column + horizontal_offset * index as f32,
            y_position_bottom_block,
            vertical_offset,
            &mut commands,
            &asset_server,
        );
    }
}
