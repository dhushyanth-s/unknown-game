use bevy::prelude::*;

mod player;
use bevy_rapier2d::prelude::*;
use mob::MobPlugin;
use player::PlayerPlugin;

mod mob;
mod collider;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, (PlayerPlugin, MobPlugin)))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct MainCamera;


fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

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
