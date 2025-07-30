use avian2d::prelude::*;
use bevy::prelude::*;

use super::{AddCollider, ColliderType};

use crate::levels::OnLevelsScreen;

pub fn spawn_element(
    //Parameters
    element_durability: ElementDurability,
    element_shape: ElementShape,
    element_position: Vec2,
    // Globals
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let asset_path = assemble_asset_path(&element_durability, &element_shape);
    let collider_type = get_collider_type(&element_shape);

    if element_durability == ElementDurability::Indestructible {
        commands.spawn(IndestructibleElementBundle {
            marker: IndestructibleElement,
            screen_marker: OnLevelsScreen,
            sprite: Sprite::from_image(asset_server.load(asset_path)),
            transform: Transform::from_xyz(element_position.x, element_position.y, 0.0),
            add_collider: AddCollider {
                collider_scale: 1.0,
                collider_type: collider_type,
            },
            rigid_body: RigidBody::Static,
        });
    } else {
        commands.spawn(DestructibleElementBundle {
            marker: DestructibleElement {
                element_durability: element_durability,
            },
            screen_marker: OnLevelsScreen,
            sprite: Sprite::from_image(asset_server.load(asset_path)),
            transform: Transform::from_xyz(element_position.x, element_position.y, 0.0),
            add_collider: AddCollider {
                collider_scale: 1.0,
                collider_type: collider_type,
            },
            rigid_body: RigidBody::Static,
        });
    }
}

fn get_collider_type(element_shape: &ElementShape) -> ColliderType {
    match element_shape {
        ElementShape::Square => ColliderType::Rectangle,
        ElementShape::Diamond => ColliderType::Diamond,
        ElementShape::Rectangle => ColliderType::Rectangle,
        ElementShape::Pentagon => ColliderType::RegularPolygon,
    }
}

fn assemble_asset_path(
    element_durability: &ElementDurability,
    element_shape: &ElementShape,
) -> String {
    let mut asset_path = String::from("element_");

    match element_durability {
        ElementDurability::Lowest => {
            asset_path.push_str("grey_");
        }
        ElementDurability::Low => {
            asset_path.push_str("blue_");
        }
        ElementDurability::Medium => {
            asset_path.push_str("green_");
        }
        ElementDurability::High => {
            asset_path.push_str("yellow_");
        }
        ElementDurability::Highest => {
            asset_path.push_str("red_");
        }
        ElementDurability::Indestructible => {
            asset_path.push_str("purple_");
        }
    }

    match element_shape {
        ElementShape::Square => {
            asset_path.push_str("square");
        }
        ElementShape::Diamond => {
            asset_path.push_str("diamond");
        }
        ElementShape::Rectangle => {
            asset_path.push_str("rectangle");
        }
        ElementShape::Pentagon => {
            asset_path.push_str("polygon");
        }
    }

    asset_path.push_str(".png");

    trace!("Element uses asset path {}", asset_path);

    asset_path
}

pub enum ElementShape {
    Square,
    Rectangle,
    Diamond,
    Pentagon,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum ElementDurability {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
    Indestructible,
}

#[derive(Component)]
pub struct DestructibleElement {
    pub element_durability: ElementDurability,
}

#[derive(Component)]
struct IndestructibleElement; // ElementDurability is implied here -> no need to store

#[derive(Bundle)]
pub struct DestructibleElementBundle {
    marker: DestructibleElement,
    screen_marker: OnLevelsScreen,
    sprite: Sprite,
    transform: Transform,
    add_collider: AddCollider,
    rigid_body: RigidBody,
}

#[derive(Bundle)]
pub struct IndestructibleElementBundle {
    marker: IndestructibleElement,
    screen_marker: OnLevelsScreen,
    sprite: Sprite,
    transform: Transform,
    add_collider: AddCollider,
    rigid_body: RigidBody,
}
