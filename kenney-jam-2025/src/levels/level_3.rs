use bevy::prelude::*;

use crate::level_elements::*;

pub fn spawn_level(
    // Globals
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    debug!("spawning level 3");

    let bottom_row_y = -100.0;
    let block_delta = 60.0; // horizontal center-to-center spacing
    let rows_total = 9; // total rows in the diamond (odd number -> symmetric)

    fn durability_for_group(group: i32) -> ElementDurability {
        match group {
            0 => ElementDurability::Lowest,
            1 => ElementDurability::Low,
            2 => ElementDurability::Medium,
            3 => ElementDurability::High,
            _ => ElementDurability::Highest,
        }
    }

    // Build rows 0..rows_total-1; blocks per row grow by 1 until center then shrink.
    // increase by one block per row: 1,2,3,...,center_count,...,3,2,1
    for row in 0..rows_total {
        let center_row = (rows_total as f32 / 2.0).floor() as usize;

        let blocks_in_row = if row < center_row {
            1 + row
        } else if row > center_row {
            rows_total - row
        } else {
            1 + center_row as usize
        };

        let x_offset = if row <= center_row {
            -0.5 * row as f32 * block_delta
        } else {
            -0.5 * (rows_total - 1 - row) as f32 * block_delta
        };

        let y = bottom_row_y + (row as f32) * block_delta * 0.5; // vertical spacing for diamonds
        let group = (row as i32) / 2; // increase durability every 2 rows

        for i in 0..blocks_in_row {
            let x = x_offset + i as f32 * block_delta;
            spawn_element(
                durability_for_group(group),
                ElementShape::Diamond,
                Vec2::new(x, y),
                &mut commands,
                &asset_server,
            );
        }
    }
}
