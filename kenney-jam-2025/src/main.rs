use avian2d::prelude::*;
use bevy::{log::*, prelude::*};
mod levels;
mod selection;
mod start;

fn main() -> AppExit {
    App::new()
        // ========= PLUGINS
        // Default Plugin
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "error,kenney-jam-2025=warn".to_string(),
            level: Level::TRACE,
            ..Default::default()
        }))
        // Add Default Physics
        // length unit 100 => 1m = 1 pixels.
        .add_plugins(PhysicsPlugins::default().with_length_unit(1.0))
        // Debug physics
        .add_plugins(PhysicsDebugPlugin::default())
        // Game plugins
        .add_plugins((
            start::start_plugin,
            selection::selection_plugin,
            levels::levels_plugin,
        ))
        // ========= SYSTEMS
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, handle_raw_input)
        .add_systems(Update, handle_input_events)
        .add_systems(Last, add_colliders)
        // ========= GAME STATE
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        // ========= RUN
        .run()
}

/*
 * ================================================================================================================
 * START - Systems
 * ================================================================================================================
 */

/*
 * Basic Game Setup
 * Spawns everything that is consistent across all game states
 */
fn setup(
    // Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    commands.spawn(PlayerBundle { marker: Player {} });

    commands.spawn(PlayerPaddleBundle {
        marker: PlayerPaddle {},
        add_collider: AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::Capsule,
        },
        sprite: Sprite::from_image(asset_server.load("paddleBlu.png")),
        transform: Transform::from_xyz(0.0, -250.0, 0.0),
        rigid_body: RigidBody::Kinematic,
    });

    commands.spawn(PlayerBallBundle {
        marker: PlayerBall {},
        add_collider: AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::Circle,
        },
        sprite: Sprite::from_image(asset_server.load("ballGrey.png")),
        transform: Transform::from_xyz(0.0, -225.0, 0.0),
        rigid_body: RigidBody::Kinematic,
    });
}

/*
 * Handle Input triggers in FixedUpdate loop
 * Note: should fire events to be handled in Update loop
 */
fn handle_raw_input(
    //Globals
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    // Events
    mut cursor_evr: EventReader<CursorMoved>,
) {
    // Press Left Mouse
    if input.just_pressed(MouseButton::Left) {
        trace!("Left pressed");

        // TODO
    }

    // Release Left Mouse
    if input.just_released(MouseButton::Left) {
        trace!("Left released");

        // TODO
    }

    for event in cursor_evr.read() {
        // TODO
    }
}

/*

* Handle Input events in Update loop
* Since there is only one entity affected by input (the paddle), only one system is needed for this
*/
fn handle_input_events() {}

fn add_colliders(
    // Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    entities_with_add_collider_tag: Query<(Entity, &Transform, &Sprite, &AddCollider)>,
) {
    for (entity, transform, sprite, add_collider) in entities_with_add_collider_tag {
        if asset_server.is_loaded(&sprite.image) {
            trace!("asset loaded, adding collider");

            // make sure to remove Add Collider
            commands.entity(entity).remove::<AddCollider>();

            match add_collider.collider_type {
                ColliderType::Circle => {
                    // add circle collider
                    let collider_size = add_collider.collider_scale
                        * (calculate_sprite_size(&images, &sprite, &transform.scale).x * 0.5);

                    commands
                        .entity(entity)
                        .insert(Collider::circle(collider_size));

                    debug!("Circle Collider created with size {}", collider_size);
                }

                ColliderType::Rectangle => {
                    let collider_size = add_collider.collider_scale
                        * calculate_sprite_size(&images, &sprite, &transform.scale);

                    commands
                        .entity(entity)
                        .insert(Collider::rectangle(collider_size.x, collider_size.y));

                    debug!("Rectangle Collider created with size {}", collider_size);
                }

                ColliderType::Capsule => {
                    // TODO
                    // Note: used fore Paddle
                }

                ColliderType::RegularPolygon => {
                    // TODO
                    // Note: used for the pentagon
                }

                ColliderType::RoundedRectangle => {
                    // TODO
                    // Note: used for the pentagon
                }
            }
        } else {
            trace!("Asset not yet loaded");
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

/*
 * ================================================================================================================
 * END - Systems
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Components - Tags
 * ================================================================================================================
 */

/*
 * Marks the player entity; should only exist once
 */
#[derive(Component)]
struct Player;

/*
 * Marks the player controlled Paddle entity; should only exist once
 */
#[derive(Component)]
struct PlayerPaddle;

/*
 * Marks the player 'controlled' Ball entity; there might be more than one
 */
#[derive(Component)]
struct PlayerBall;

/*
 * Marks a destructible Element entity in a scene
 */
#[derive(Component)]
struct DestructibleElement;

/*
 * Marks a permanent (non-destructible) Element entity in a scene
 */
#[derive(Component)]
struct PermanentElement;

/*
 * ================================================================================================================
 * END - Components - Tags
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Components - Structs
 * ================================================================================================================
 */

#[derive(Component)]
struct AddCollider {
    collider_scale: f32,
    collider_type: ColliderType,
}

/*
 * ================================================================================================================
 * END - Components - Structs
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Bundles
 * ================================================================================================================
 */
#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
}

#[derive(Bundle)]
struct PlayerPaddleBundle {
    marker: PlayerPaddle,
    add_collider: AddCollider,
    sprite: Sprite,
    transform: Transform,
    rigid_body: RigidBody,
}

#[derive(Bundle)]
struct PlayerBallBundle {
    marker: PlayerBall,
    add_collider: AddCollider,
    sprite: Sprite,
    transform: Transform,
    rigid_body: RigidBody,
}

#[derive(Bundle)]
struct DestructribleElementBundle {
    marker: DestructibleElement,
    add_collider: AddCollider,
    sprite: Sprite,
    transform: Transform,
    rigid_body: RigidBody,
}

#[derive(Bundle)]
struct PermanentElementBundle {
    marker: PermanentElement,
    add_collider: AddCollider,
    sprite: Sprite,
    transform: Transform,
    rigid_body: RigidBody,
}

/*
 * ================================================================================================================
 * END - Bundles
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Enumerations
 * ================================================================================================================
 */

/*
 * Used to define how the AddCollider Component will be interpreted
 */
pub enum ColliderType {
    Circle,
    Rectangle,
    Capsule,
    RegularPolygon,
    RoundedRectangle,
}

/*
 * ================================================================================================================
 * END - Enumerations
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Events
 * ================================================================================================================
 */

#[derive(Event)]
struct BallLaunchEvent;

#[derive(Event)]
struct MouseMoveEvent;

#[derive(Event)]
struct BallDestroyedEvent;

#[derive(Event)]
struct ElementDestroyedEvent;

/*
 * ================================================================================================================
 * END - Events
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Functions
 * ================================================================================================================
 */

/*
Calculates sprite size and returns it
 */
fn calculate_sprite_size(images: &Res<Assets<Image>>, sprite: &Sprite, scale: &Vec3) -> Vec2 {
    let mut _sprite_size = if let Some(custom_size) = sprite.custom_size {
        trace!("Using custom sprite size");
        custom_size
    } else if let Some(image) = images.get(sprite.image.id()) {
        trace!("using image size");
        image.size_f32()
    } else {
        warn!("no custom size or sprite size found");
        Vec2::new(1.0, 1.0)
    };

    _sprite_size = _sprite_size * scale.truncate();

    _sprite_size
}

/*
 * ================================================================================================================
 * END - Functions
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - States
 * ================================================================================================================
 */

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Start,
    Selection,
    Levels,
}

/*
 * ================================================================================================================
 * END - States
 * ================================================================================================================
 */
