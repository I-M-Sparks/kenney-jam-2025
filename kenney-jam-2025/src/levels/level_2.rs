use bevy::prelude::*;

use crate::level_elements::*;

pub fn spawn_level(
    // Globals
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    debug!("spawning level 2");

    // spawn 3 columns
    for index in 0..3 {
        let x_coordinate_left_column = -400.0;
        let horizontal_offset = 400.0; // in pixels
        let y_position_bottom_block = -100.0;
        let vertical_offset = 75.0; // in pixels

        spawn_square_block_column(
            x_coordinate_left_column + horizontal_offset * index as f32,
            y_position_bottom_block,
            vertical_offset,
            &mut commands,
            &asset_server,
        );

        // spawn two X between the columns

        // LEFT cross
        let lower_left_pos = Vec2::new(-300.0, -66.5);
        let cross_dimensions = Vec2::new(200.0, 225.0);

        // bottom row of the cross
        spawn_element(
            ElementDurability::Lowest,
            ElementShape::Diamond,
            lower_left_pos.clone(),
            &mut commands,
            &asset_server,
        );
        spawn_element(
            ElementDurability::Lowest,
            ElementShape::Diamond,
            lower_left_pos
                .clone()
                .with_x(lower_left_pos.x + cross_dimensions.x),
            &mut commands,
            &asset_server,
        );

        // center of the cross
        spawn_element(
            ElementDurability::Medium,
            ElementShape::Diamond,
            lower_left_pos.clone() + (cross_dimensions.clone() * 0.5),
            &mut commands,
            &asset_server,
        );

        // top row of the cross
        spawn_element(
            ElementDurability::High,
            ElementShape::Diamond,
            lower_left_pos
                .clone()
                .with_y(lower_left_pos.y + cross_dimensions.y),
            &mut commands,
            &asset_server,
        );
        spawn_element(
            ElementDurability::High,
            ElementShape::Diamond,
            lower_left_pos.clone() + cross_dimensions.clone(),
            &mut commands,
            &asset_server,
        );

        let lower_left_pos = Vec2::new(100.0, -66.5);
        let cross_dimensions = Vec2::new(200.0, 225.0);

        // bottom row of the cross
        spawn_element(
            ElementDurability::Lowest,
            ElementShape::Diamond,
            lower_left_pos.clone(),
            &mut commands,
            &asset_server,
        );
        spawn_element(
            ElementDurability::Lowest,
            ElementShape::Diamond,
            lower_left_pos
                .clone()
                .with_x(lower_left_pos.x + cross_dimensions.x),
            &mut commands,
            &asset_server,
        );

        // center of the cross
        spawn_element(
            ElementDurability::Medium,
            ElementShape::Diamond,
            lower_left_pos.clone() + (cross_dimensions.clone() * 0.5),
            &mut commands,
            &asset_server,
        );

        // top row of the cross
        spawn_element(
            ElementDurability::High,
            ElementShape::Diamond,
            lower_left_pos
                .clone()
                .with_y(lower_left_pos.y + cross_dimensions.y),
            &mut commands,
            &asset_server,
        );
        spawn_element(
            ElementDurability::High,
            ElementShape::Diamond,
            lower_left_pos.clone() + cross_dimensions.clone(),
            &mut commands,
            &asset_server,
        );
    }
}
