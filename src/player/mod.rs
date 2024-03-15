use bevy::{prelude::*, window::PrimaryWindow};

use crate::MainCamera;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite: SpriteBundle,
    player: Player,
}

/// We will add this to each camera we want to compute cursor position for.
/// Add the component to the camera that renders to each window.
#[derive(Resource, Default)]
pub struct MyWorldCoords(Vec2);

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyWorldCoords>()
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, (Player::player_move, update_camera))
            .add_systems(Update, (Player::player_rotate));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Player::new());
}


fn update_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    (mut cam_query, player_query): (
        Query<&mut Transform, (With<Camera>, Without<Player>)>,
        Query<&Transform, (With<Player>, Without<Camera>)>,
    ),
    time: Res<Time>,
) {
    let mut camera = cam_query.single_mut();
    let player = player_query.single();

    camera.translation = player.translation;
}

impl Player {
    pub fn new() -> PlayerBundle {
        PlayerBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 50.0, 0.0),
                    scale: Vec3::new(20.0, 20.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.3, 0.3, 0.7),
                    ..default()
                },
                ..default()
            },
            player: Player,
        }
    }

    pub fn player_move(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut query: Query<&mut Transform, With<Player>>,
        time: Res<Time>,
    ) {
        let mut player_transform = query.single_mut();
        let mut direction = Vec2::new(0.0, 0.0);
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction += Vec2::new(-1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec2::new(1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec2::new(0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction += Vec2::new(0.0, -1.0);
        }
        if direction.length() != 0.0 {
            direction = direction.normalize();
        }

        player_transform.translation += direction.extend(0.0) * 500.0 * time.delta_seconds();
    }

    pub fn player_rotate(
        mut q_player: Query<&mut Transform, With<Player>>,
        mut mycoords: ResMut<MyWorldCoords>,
        // query to get the window (so we can read the current cursor position)
        q_window: Query<&Window, With<PrimaryWindow>>,
        // query to get camera transform
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so Query::single() is OK
        let (camera, camera_transform) = q_camera.single();
        let mut player_transform = q_player.single_mut();

        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            mycoords.0 = world_position;
            let player_translation = player_transform.translation.truncate();
            let diff = (world_position - player_translation);
            let angle = diff.y.atan2(diff.x);
            player_transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
        }
    }
}
