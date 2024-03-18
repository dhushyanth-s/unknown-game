use bevy::prelude::*;

use super::{MyWorldCoords, Player};

#[derive(Bundle)]
pub struct BulletBundle {
    sprite: SpriteBundle
}

#[derive(Component)]
pub struct Bullet {
    target_dir: Vec2,
    timer: Timer
}

impl Bullet {
    pub fn new(player_pos: Vec3, mouse_pos: Vec2) -> (BulletBundle, Bullet) {
        (BulletBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: player_pos,
                    scale: Vec3::new(5.0, 5.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 0.0),
                    ..default()
                },
                ..default()
            }
        }, Bullet {
            target_dir: (mouse_pos - player_pos.truncate()).normalize(),
            timer: Timer::from_seconds(1.0, TimerMode::Once)
        })
    }

    pub fn update(mut commands: Commands, mut q: Query<(Entity, &mut Transform, &mut Bullet)>, time: Res<Time>) {
        for (entity, mut bullet_transform, mut bullet ) in q.iter_mut() {
            bullet.timer.tick(time.delta());

            let dir = bullet.target_dir.extend(0.0);

            bullet_transform.translation += dir * 500.0 * time.delta_seconds();

            if bullet.timer.finished() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn shoot(mut commands: Commands, mouse_input: Res<ButtonInput<MouseButton>>, mouse_coords: Res<MyWorldCoords>, player_q: Query<&Transform, With<Player>>) {
    let player_transform = player_q.single();
    if mouse_input.just_pressed(MouseButton::Left) {
        commands.spawn(Bullet::new(player_transform.translation, mouse_coords.0));
    }
}