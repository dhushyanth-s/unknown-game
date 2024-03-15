use bevy::prelude::*;

mod player;
use player::{MyWorldCoords, Player};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<MyWorldCoords>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (Player::player_move, update_camera))
        .add_systems(Update, (Player::player_rotate))
        .run();
}

#[derive(Component)]
pub struct MainCamera;

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

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(Player::new());

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 10.0,
                y: 100.0,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.7),
            ..default()
        },
        ..default()
    });
}
