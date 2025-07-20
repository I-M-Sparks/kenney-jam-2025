use bevy::prelude::*;

use crate::level_elements::*;

pub fn spawn_level(
    // Globals
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    debug!("spawning level 1");
    spawn_element(
        ElementDurability::Highest,
        ElementShape::Square,
        Vec2::new(0.0, 0.0),
        &mut commands,
        &asset_server,
    );
}
