use avian2d::prelude::*;
use bevy::{log::*, prelude::*};
use std::ops::*;

mod levels;
use crate::levels::SelectedLevel;

mod selection;

mod level_elements;

fn main() -> AppExit {
    App::new()
        // ========= PLUGINS
        // Default Plugin
        .add_plugins(DefaultPlugins.set(LogPlugin {
                filter:
                    //"kenney-jam-2025=trace,bevy_ecs=error,naga=error,wgpu_core=error,avian2d=error"
                    "error,kenney_jam_2025=trace"
                        .to_string(),
                level: bevy::log::Level::TRACE,
                ..Default::default()
            }))
        // Add Default Physics
        // length unit 100 => 1m = 1 pixels.
        .add_plugins(PhysicsPlugins::default().with_length_unit(10.0))
        // Debug physics
        .add_plugins(PhysicsDebugPlugin::default())
        // Game plugins
        .add_plugins((selection::selection_plugin, levels::levels_plugin))
        // ========= SYSTEMS
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, handle_raw_input)
        .add_systems(PreUpdate, handle_collision_player_ball)
        .add_systems(
            Update,
            (
                handle_left_mouse_press_events,
                handle_left_mouse_release_events,
                handle_mouse_move_events,
                handle_ball_destroyed_event,
            ),
        )
        .add_systems(
            PostUpdate,
            (
                player_ball_physics_sanity_check,
                handle_collision_player_ball_and_bottom_collider,
            ),
        )
        .add_systems(Last, add_colliders)
        // ========= EVENTS
        .add_event::<MouseMoveEvent>()
        .add_event::<BallDestroyedEvent>()
        .add_event::<LeftMousePressEvent>()
        .add_event::<LeftMouseReleaseEvent>()
        .add_event::<ElementDestroyedEvent>()
        .add_event::<RightMousePressEvent>()
        // ========= RESOURCE
        .insert_resource(Gravity(Vec2::ZERO))
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
    mut default_restitution: ResMut<DefaultRestitution>,
    mut default_friction: ResMut<DefaultFriction>,
) {
    commands.spawn(Camera2d);

    commands.spawn(PlayerBundle {
        marker: Player {
            highest_selectable_level: SelectedLevel::Level1,
        },
    });

    commands.spawn(PlayerPaddleBundle {
        marker: PlayerPaddle,
        add_collider: AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::Capsule,
        },
        sprite: Sprite::from_image(asset_server.load("paddleBlu.png")),
        transform: Transform::from_xyz(0.0, -300.0, 0.0).with_scale(Vec3::new(1.3, 1.0, 1.0)),
        rigid_body: RigidBody::Static,
    });
    //pre-load red paddle
    let _red_paddle = Sprite::from_image(asset_server.load("paddleRed.png"));

    // left border
    let mut permanent_element_transform = Transform::default();
    permanent_element_transform.translation = Vec3::new(-615.0, 0.0, 0.0);
    use core::f32::consts::FRAC_PI_2; // half a turn = 𝛕/2 = 180°
    permanent_element_transform.rotate_z(FRAC_PI_2);
    permanent_element_transform.scale.x = 3.8;

    commands.spawn(PermanentElementBundle {
        marker: PermanentElement,
        add_collider: AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::RoundedRectangle,
        },
        sprite: Sprite::from_image(asset_server.load("buttonSelected.png")),
        transform: permanent_element_transform,
        rigid_body: RigidBody::Static,
    });

    // right border
    permanent_element_transform.translation = Vec3::new(615.0, 0.0, 0.0);
    use core::f32::consts::PI; // half a turn = 𝛕/2 = 180°
    permanent_element_transform.rotate_z(-PI);

    commands.spawn(PermanentElementBundle {
        marker: PermanentElement,
        add_collider: AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::RoundedRectangle,
        },
        sprite: Sprite::from_image(asset_server.load("buttonSelected.png")),
        transform: permanent_element_transform,
        rigid_body: RigidBody::Static,
    });

    // top border
    permanent_element_transform.translation = Vec3::new(0.0, 335.0, 0.0);
    permanent_element_transform.rotate_z(FRAC_PI_2);
    permanent_element_transform.scale.x = 6.2;

    commands.spawn(PermanentElementBundle {
        marker: PermanentElement,
        add_collider: AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::RoundedRectangle,
        },
        sprite: Sprite::from_image(asset_server.load("buttonSelected.png")),
        transform: permanent_element_transform,
        rigid_body: RigidBody::Static,
    });

    commands.spawn(BottomColliderBundle {
        marker: BottomCollider,
        collider: Collider::rectangle(1280.0, 60.0),
        transform: Transform::from_xyz(0.0, -385.0, 0.0),
        rigid_body: RigidBody::Static,
    });

    default_restitution.coefficient = 1.0;
    default_restitution.combine_rule = CoefficientCombine::Max;

    default_friction.dynamic_coefficient = 0.0;
}

/*
 * Handle Input triggers in FixedUpdate loop
 * Note: should fire events to be handled in Update loop
 */
fn handle_raw_input(
    input: Res<ButtonInput<MouseButton>>,
    // Events
    mut cursor_evr: EventReader<CursorMoved>,
    mut mouse_move_evw: EventWriter<MouseMoveEvent>,
    mut left_mouse_press_evw: EventWriter<LeftMousePressEvent>,
    mut left_mouse_release_evw: EventWriter<LeftMouseReleaseEvent>,
    mut right_mouse_press_evw: EventWriter<RightMousePressEvent>,
) {
    // Press Left Mouse
    if input.just_pressed(MouseButton::Left) {
        trace!("Left pressed");

        left_mouse_press_evw.write(LeftMousePressEvent);
    }

    // Release Left Mouse
    if input.just_released(MouseButton::Left) {
        trace!("Left pressed");

        left_mouse_release_evw.write(LeftMouseReleaseEvent);
    }

    // Mouse move
    for event in cursor_evr.read() {
        trace!("Mouse moved");

        mouse_move_evw.write(MouseMoveEvent {
            position: event.position,
        });
    }

    //Press Right Mouse
    if input.just_pressed(MouseButton::Right) {
        trace!("Right pressed");

        right_mouse_press_evw.write(RightMousePressEvent);
    }
}

/*
* Handle Input events in Update loop
* Since there is only one entity affected by input (the paddle), only one system is needed for this
*/
fn handle_mouse_move_events(
    //Singles
    player_paddle: Single<&mut Transform, With<PlayerPaddle>>,
    player_ball: Option<Single<&mut Transform, (With<PlayerBallInHold>, Without<PlayerPaddle>)>>,
    // Events
    mut mouse_move_evr: EventReader<MouseMoveEvent>,
    // Queries
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let mut paddle_transform = player_paddle.into_inner();

    for event in mouse_move_evr.read() {
        trace!("Move Event");

        if camera.single().is_ok() {
            if let Some((camera, camera_transform)) = camera.single().ok() {
                let cursor_position_in_world_coord = camera.viewport_to_world_2d(
                    camera_transform,
                    Vec2::new(event.position.x, event.position.y),
                );

                if cursor_position_in_world_coord.is_ok() {
                    if let Some(cursor_position_in_world_coord) =
                        cursor_position_in_world_coord.ok()
                    {
                        paddle_transform.translation.x = cursor_position_in_world_coord.x;
                        enforce_paddle_borders(&mut paddle_transform);
                        trace!("paddle translation.x: {}", paddle_transform.translation.x)
                    }
                }
            }
        }
    }

    if let Some(player_ball) = player_ball {
        trace!("Moving player ball");
        let mut player_ball_transform = player_ball.into_inner();

        player_ball_transform.translation.x = paddle_transform.translation.x;
    }
}

fn handle_left_mouse_press_events(
    //Singles
    player_paddle: Single<(&Transform, &mut Sprite), With<PlayerPaddle>>,
    // Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // Events
    mut left_mouse_press_evr: EventReader<LeftMousePressEvent>,
    // Queries
    player_balls: Query<Entity, (With<PlayerBall>, Without<PlayerPaddle>)>,
) {
    let (paddle_transform, mut sprite) = player_paddle.into_inner();

    for _event in left_mouse_press_evr.read() {
        trace!("Mouse press");

        // TODO check that mouse press is on the paddle (inside paddle sprite)
        if player_balls.is_empty() {
            sprite.image = asset_server.load("paddleRed.png");

            commands
                .spawn(PlayerBallBundle {
                    marker: PlayerBall {
                        initial_impulse: Vec2::new(0.0, 130000.0),
                        power_level: PowerLevel::default(),
                    },
                    add_collider: AddCollider {
                        collider_scale: 1.0,
                        collider_type: ColliderType::Circle,
                    },
                    sprite: Sprite::from_image(asset_server.load("ballGrey.png")),
                    transform: Transform::from_xyz(paddle_transform.translation.x, -260.0, 0.0)
                        .with_scale(Vec3::new(1.3, 1.3, 1.0)),
                    rigid_body: RigidBody::Dynamic,
                    max_linear_speed: MaxLinearSpeed(1000.0),
                })
                .insert(PlayerBallInHold);
            debug!("PlayerBall spawned");
        }

        break; // if there is more than one event in queue: ignore it
    }
}

fn handle_left_mouse_release_events(
    //Singles
    player_paddle: Single<&mut Sprite, With<PlayerPaddle>>,
    player_ball: Single<(Entity, &PlayerBall), With<PlayerBallInHold>>,
    //Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // Events
    mut left_mouse_release_evr: EventReader<LeftMouseReleaseEvent>,
) {
    let mut sprite = player_paddle.into_inner();
    let (player_ball_entity, player_ball) = player_ball.into_inner();

    for _event in left_mouse_release_evr.read() {
        sprite.image = asset_server.load("paddleBlu.png");

        commands
            .entity(player_ball_entity)
            .remove::<PlayerBallInHold>();

        commands
            .entity(player_ball_entity)
            .insert(ExternalImpulse::new(player_ball.initial_impulse));

        debug!("PlayerBall launched");

        break; // if there is more than one event in queue: ignore it
    }
}

fn handle_ball_destroyed_event(
    //Singles
    player_ball: Single<Entity, With<PlayerBall>>,
    // Globals
    mut commands: Commands,
    //Events
    mut ball_destroyed_evr: EventReader<BallDestroyedEvent>,
) {
    let player_ball = player_ball.into_inner();
    for _event in ball_destroyed_evr.read() {
        commands.entity(player_ball).despawn();
        break;
    }
}

fn player_ball_physics_sanity_check(
    // Single
    player_ball: Single<&Transform, With<PlayerBall>>,
    // Globals
    mut ball_destroyed_evw: EventWriter<BallDestroyedEvent>,
) {
    let player_ball = player_ball.into_inner();

    if f32::abs(player_ball.translation.x) > 1500.0 || f32::abs(player_ball.translation.y) > 1000.0
    {
        ball_destroyed_evw.write(BallDestroyedEvent);
        debug!("Ball broke the physics; destroyed event sent");
    }
}

fn handle_collision_player_ball(
    // Single
    player_ball: Single<(Entity, &mut PlayerBall, &mut Sprite, &mut LinearVelocity)>,
    // Collisions
    collisions: Collisions,
) {
    let (player_ball_entity, mut player_ball, mut ball_sprite, mut ball_velocity) =
        player_ball.into_inner();

    for contact_pair in collisions.iter() {
        if contact_pair.collider1.eq(&player_ball_entity)
            || contact_pair.collider2.eq(&player_ball_entity)
        {
            trace!(
                "PlayerBall velocity at collision {:?}",
                ball_velocity.length()
            );

            let velocity_addition_factor = 10.0;
            let velocity_addition = ball_velocity
                .clone()
                .normalize()
                .mul(velocity_addition_factor);

            ball_velocity.0 = ball_velocity.0.add(velocity_addition);

            trace!(
                "PlayerBall velocity post collision {:?}",
                ball_velocity.length()
            );

            // Change ball color based on velocity

            // power levels;
            // when the ball reaches another level, color should change
            // white = < [0]
            // blue = [0]
            // green = [1]
            // yellow = [2]
            // red = > [3]
            let power_levels = [200.0, 400.0, 600.0, 800.0];

            let ball_colors = [
                Color::WHITE,                                      // white
                Color::LinearRgba(LinearRgba::rgb(0.5, 0.8, 1.0)), // light blue
                Color::LinearRgba(LinearRgba::rgb(0.0, 1.0, 0.0)), // green
                Color::LinearRgba(LinearRgba::rgb(1.0, 1.0, 0.0)), // yellow
                Color::LinearRgba(LinearRgba::rgb(1.0, 0.2, 0.2)), // red
            ];

            let mut power_level: usize = 0;
            for power_level_index in 0..power_levels.len() {
                if ball_velocity.length() < power_levels[power_level_index] {
                    break;
                }
                power_level += 1;
            }

            match power_level {
                0 => {
                    player_ball.power_level = PowerLevel::Lowest;
                }
                1 => {
                    player_ball.power_level = PowerLevel::Low;
                }
                2 => {
                    player_ball.power_level = PowerLevel::Medium;
                }
                3 => {
                    player_ball.power_level = PowerLevel::High;
                }
                4 => {
                    player_ball.power_level = PowerLevel::Highest;
                }
                _ => {}
            }

            ball_sprite.color = ball_colors[power_level];
            trace!("Setting ball color to {:?}", ball_sprite.color);
        }

        break;
    }
}

fn add_colliders(
    // Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    entities_with_add_collider_tag: Query<(Entity, &Sprite, &AddCollider)>,
) {
    for (entity, sprite, add_collider) in entities_with_add_collider_tag {
        if asset_server.is_loaded(&sprite.image) {
            trace!("asset loaded, adding collider");

            // make sure to remove Add Collider
            commands.entity(entity).remove::<AddCollider>();

            match add_collider.collider_type {
                ColliderType::None => {}

                ColliderType::Circle => {
                    // add circle collider
                    let collider_size = add_collider.collider_scale
                        * (calculate_sprite_size(&images, &sprite).x * 0.5);

                    commands
                        .entity(entity)
                        .insert(Collider::circle(collider_size));

                    debug!("Circle Collider created with size {}", collider_size);
                }

                ColliderType::Rectangle => {
                    let collider_size =
                        add_collider.collider_scale * calculate_sprite_size(&images, &sprite);

                    commands
                        .entity(entity)
                        .insert(Collider::rectangle(collider_size.x, collider_size.y));

                    debug!("Rectangle Collider created with size {}", collider_size);
                }

                ColliderType::Capsule => {
                    let collider_size =
                        add_collider.collider_scale * calculate_sprite_size(&images, &sprite);

                    commands.entity(entity).insert(Collider::capsule_endpoints(
                        collider_size.x * 0.1,
                        Vec2::new(collider_size.x * -0.4, 0.0),
                        Vec2::new(collider_size.x * 0.4, 0.0),
                    ));
                }

                ColliderType::RegularPolygon => {
                    let collider_size =
                        add_collider.collider_scale * calculate_sprite_size(&images, &sprite);

                    // currently only used for a pentagon
                    commands
                        .entity(entity)
                        .insert(Collider::regular_polygon(collider_size.y * 0.55, 5));

                    debug!(
                        "RegularPolygon Collider added with circumradius {} and side count {}",
                        collider_size.y * 0.5,
                        5
                    );
                }

                ColliderType::RoundedRectangle => {
                    let collider_size =
                        add_collider.collider_scale * calculate_sprite_size(&images, &sprite);

                    let border_radius = 1.0;

                    commands.entity(entity).insert(Collider::round_rectangle(
                        collider_size.x,
                        collider_size.y,
                        border_radius,
                    ));

                    debug!(
                        "RoundRectangle Collider created with size {} and border_radius {}",
                        collider_size, border_radius
                    );
                }

                ColliderType::Diamond => {
                    let collider_size =
                        add_collider.collider_scale * calculate_sprite_size(&images, &sprite);

                    let collider_hull_points = Vec::from([
                        Vec2::new(0.0, collider_size.y * 0.5),
                        Vec2::new(collider_size.x * 0.5, 0.0),
                        Vec2::new(0.0, collider_size.y * -0.5),
                        Vec2::new(collider_size.x * -0.5, 0.0),
                    ]);

                    if let Some(diamond_collider) = Collider::convex_hull(collider_hull_points) {
                        commands.entity(entity).insert(diamond_collider);
                    } else {
                        debug!("Creating diamond collider failed!");
                    }

                    // TODO actually create the collider
                }
            }
        } else {
            trace!("Asset not yet loaded");
        }
    }
}

fn handle_collision_player_ball_and_bottom_collider(
    // Singles
    player_ball: Single<Entity, With<PlayerBall>>,
    bottom_collider: Single<Entity, With<BottomCollider>>,
    // Collisions
    collisions: Collisions,
    // Events
    mut ball_destroyed_evw: EventWriter<BallDestroyedEvent>,
) {
    let player_ball = player_ball.into_inner();
    let bottom_collider = bottom_collider.into_inner();

    for contact_pair in collisions.iter() {
        // if one of the colliders is the player ball and one of them is the bottom collider
        if (contact_pair.collider1.eq(&player_ball) || contact_pair.collider2.eq(&player_ball))
            && (contact_pair.collider1.eq(&bottom_collider)
                || contact_pair.collider2.eq(&bottom_collider))
        {
            ball_destroyed_evw.write(BallDestroyedEvent);
            debug!("Ball fell through; sending destruction event");
            break;
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
 * Marks the player controlled Paddle entity; should only exist once
 */
#[derive(Component)]
struct PlayerPaddle;

/*
 * Marks a permanent (non-destructible) Element entity in a scene
 */
#[derive(Component)]
struct PermanentElement;

/*
 * Marks the player ball as still being held (between ball being spawned and player launching it)
 */
#[derive(Component)]
struct PlayerBallInHold;

/*
 * Markes the bottom blocker used to determine when the ball has been "destroyed" (fell through)
 */
#[derive(Component)]
struct BottomCollider;

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

/*
 * Marks the player entity; should only exist once
 */
#[derive(Component)]
struct Player {
    highest_selectable_level: SelectedLevel,
}

#[derive(Component)]
struct AddCollider {
    collider_scale: f32,
    collider_type: ColliderType,
}

/*
 * Marks the player 'controlled' Ball entity; there might be more than one
 */
#[derive(Component)]
struct PlayerBall {
    initial_impulse: Vec2,
    power_level: PowerLevel,
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
    max_linear_speed: MaxLinearSpeed,
}

#[derive(Bundle)]
struct PermanentElementBundle {
    marker: PermanentElement,
    add_collider: AddCollider,
    sprite: Sprite,
    transform: Transform,
    rigid_body: RigidBody,
}

#[derive(Bundle)]
struct BottomColliderBundle {
    marker: BottomCollider,
    collider: Collider,
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
#[derive(Default, Eq, PartialEq)]
pub enum ColliderType {
    #[default]
    None,
    Circle,
    Rectangle,
    Capsule,
    RegularPolygon,
    RoundedRectangle,
    Diamond,
}

#[derive(Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerLevel {
    #[default]
    Lowest,
    Low,
    Medium,
    High,
    Highest,
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
struct LeftMousePressEvent;

#[derive(Event)]
struct LeftMouseReleaseEvent;

#[derive(Event)]
struct RightMousePressEvent;

#[derive(Event)]
struct MouseMoveEvent {
    position: Vec2,
}

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
-* Calculates sprite size and returns it
*/
fn calculate_sprite_size(images: &Res<Assets<Image>>, sprite: &Sprite) -> Vec2 {
    let sprite_size = if let Some(custom_size) = sprite.custom_size {
        trace!("Using custom sprite size {}", custom_size);
        custom_size
    } else if let Some(image) = images.get(sprite.image.id()) {
        trace!("using image size {}", image.size_f32());
        image.size_f32()
    } else {
        warn!("no custom size or sprite size found");
        Vec2::new(1.0, 1.0)
    };

    sprite_size
}

fn enforce_paddle_borders(transform: &mut Transform) {
    const PADDLE_MAX_ABS_TRANSLATION: f32 = 520.0;

    if f32::abs(transform.translation.x) > PADDLE_MAX_ABS_TRANSLATION {
        transform.translation.x = PADDLE_MAX_ABS_TRANSLATION
            * ((transform.translation.x / PADDLE_MAX_ABS_TRANSLATION) as i32) as f32;
    }
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
    Selection,
    Levels,
}

/*
 * ================================================================================================================
 * END - States
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * START - Resource
 * ================================================================================================================
 */

/*
 * ================================================================================================================
 * END - Resource
 * ================================================================================================================
 */
