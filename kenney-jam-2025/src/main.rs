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
        .add_plugins((
            start::start_plugin,
            selection::selection_plugin,
            levels::levels_plugin,
        ))
        // ========= SYSTEMS
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, handle_raw_input)
        .add_systems(
            Update,
            (
                handle_left_mouse_press_events,
                handle_left_mouse_release_events,
                handle_mouse_move_events,
            ),
        )
        .add_systems(Last, add_colliders)
        // ========= EVENTS
        .add_event::<MouseMoveEvent>()
        .add_event::<BallDestroyedEvent>()
        .add_event::<LeftMousePressEvent>()
        .add_event::<LeftMouseReleaseEvent>()
        .add_event::<ElementDestroyedEvent>()
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
) {
    commands.spawn(Camera2d);

    commands.spawn(PlayerBundle { marker: Player {} });

    commands.spawn(PlayerPaddleBundle {
        marker: PlayerPaddle { paddle_speed: 1.0 },
        add_collider: AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::Capsule,
        },
        sprite: Sprite::from_image(asset_server.load("paddleBlu.png")),
        transform: Transform::from_xyz(0.0, -320.0, 0.0),
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

    // TODO bottom border

    default_restitution.coefficient = 1.075;
    default_restitution.combine_rule = CoefficientCombine::Max;
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

        if let Some(delta) = event.delta {
            mouse_move_evw.write(MouseMoveEvent { delta: delta });
        }
    }
}

/*
* Handle Input events in Update loop
* Since there is only one entity affected by input (the paddle), only one system is needed for this
*/
fn handle_mouse_move_events(
    //Singles
    player_paddle: Single<(&mut Transform, &PlayerPaddle)>,
    player_ball: Option<Single<&mut Transform, (With<PlayerBallInHold>, Without<PlayerPaddle>)>>,
    // Events
    mut mouse_move_evr: EventReader<MouseMoveEvent>,
) {
    let (mut paddle_transform, player_paddle) = player_paddle.into_inner();

    for event in mouse_move_evr.read() {
        trace!("Move Event");

        paddle_transform.translation.x =
            paddle_transform.translation.x + (event.delta.x * player_paddle.paddle_speed);

        enforce_paddle_borders(&mut paddle_transform);
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
    // TODO handle mouse press
    for event in left_mouse_press_evr.read() {
        trace!("Mouse press");

        // TODO check that mouse press is on the paddle (inside paddle sprite)
        if player_balls.is_empty() {
            sprite.image = asset_server.load("paddleRed.png");

            commands
                .spawn(PlayerBallBundle {
                    marker: PlayerBall {
                        initial_impulse: Vec2::new(0.0, 200000.0),
                    },
                    add_collider: AddCollider {
                        collider_scale: 1.0,
                        collider_type: ColliderType::Circle,
                    },
                    sprite: Sprite::from_image(asset_server.load("ballGrey.png")),
                    transform: Transform::from_xyz(paddle_transform.translation.x, -290.0, 0.0),
                    rigid_body: RigidBody::Dynamic,
                })
                .insert(PlayerBallInHold);
        }

        break; // if there is more than one event in queue: ignore it
    }
}

fn handle_left_mouse_release_events(
    //Singles
    player_paddle: Single<&mut Sprite, With<PlayerPaddle>>,
    mut player_ball: Single<(Entity, &PlayerBall), With<PlayerBallInHold>>,
    //Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // Events
    mut left_mouse_release_evr: EventReader<LeftMouseReleaseEvent>,
) {
    let mut sprite = player_paddle.into_inner();
    let (player_ball_entity, player_ball) = player_ball.into_inner();

    // TODO handle mouse release
    for event in left_mouse_release_evr.read() {
        trace!("Mouse press");
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
    mut player_ball: Single<Entity, With<PlayerBall>>,
    //Events
    mut ball_destroyed_evr: EventReader<BallDestroyedEvent>,
) {
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
                    // TODO
                    let collider_size =
                        add_collider.collider_scale * calculate_sprite_size(&images, &sprite);

                    commands.entity(entity).insert(Collider::capsule_endpoints(
                        collider_size.x * 0.1,
                        Vec2::new(collider_size.x * -0.4, 0.0),
                        Vec2::new(collider_size.x * 0.4, 0.0),
                    ));
                }

                ColliderType::RegularPolygon => {
                    // TODO
                    // Note: used for the pentagon
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
 * Marks the player ball as still being held (between ball being spawned and player launching it)
 */
#[derive(Component)]
struct PlayerBallInHold;

/*
 * Markes the bottom blocker used to determine when the ball has been "destroyed" (fell through)
 */
#[derive(Component)]
struct BottomBlocker;

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
 * Marks the player controlled Paddle entity; should only exist once
 */
#[derive(Component)]
struct PlayerPaddle {
    paddle_speed: f32, // TBD unit? pixels per second maybe?
}

/*
 * Marks the player 'controlled' Ball entity; there might be more than one
 */
#[derive(Component)]
struct PlayerBall {
    initial_impulse: Vec2,
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

#[derive(Bundle)]
struct BottomBlockerBundle {
    marker: BottomBlocker,
    add_collider: Collider,
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
struct LeftMousePressEvent;

#[derive(Event)]
struct LeftMouseReleaseEvent;

#[derive(Event)]
struct MouseMoveEvent {
    delta: Vec2,
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
Calculates sprite size and returns it
 */
fn calculate_sprite_size(images: &Res<Assets<Image>>, sprite: &Sprite) -> Vec2 {
    let mut sprite_size = if let Some(custom_size) = sprite.custom_size {
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
    const PADDLE_MAX_ABS_TRANSLATION: f32 = 539.0;

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
    Start,
    Selection,
    Levels,
}

/*
 * ================================================================================================================
 * END - States
 * ================================================================================================================
 */
