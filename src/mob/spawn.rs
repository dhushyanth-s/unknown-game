use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

use bevy_rapier2d::prelude::*;

use crate::collider::ColliderBundle;

#[derive(Resource)]
pub struct MobSpawnConfig {
    timer: Timer,
}

#[derive(Component)]
pub struct Mob;

#[derive(Bundle)]
struct MobBundle {
    sprite: SpriteBundle,
    mob: Mob,
    collider_bundle: ColliderBundle,
}

impl Mob {
    fn new(pos: Vec3) -> MobBundle {
        MobBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: pos,
                    scale: Vec3 {
                        x: 10.0,
                        y: 10.0,
                        z: 0.0,
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.3),
                    ..default()
                },
                ..default()
            },
            mob: Mob {},
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(10.0, 10.0),
                ..default()
            },
        }
    }
}

pub fn setup_spawning(mut commands: Commands) {
    commands.insert_resource(MobSpawnConfig {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}

pub fn spawn(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut config: ResMut<MobSpawnConfig>,
    time: Res<Time>,
    q_camera: Query<&Transform, With<Camera>>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let window = q_window.single();
        let camera_transform = q_camera.single();

        let window_width = window.resolution.width();
        let window_height = window.resolution.height();

        commands.spawn(Mob::new(Vec3 {
            x: window_width / 4.0 + camera_transform.translation.x,
            y: window_height / 4.0 + camera_transform.translation.y,
            z: 0.0,
        }));
    }
}
