use bevy::{ecs::world, prelude::*};
use bevy_rapier2d::pipeline::CollisionEvent;

use crate::player::Player;

use super::spawn::Mob;

const MOB_MOVE_SPEED: f32 = 500.0;

pub fn movement(
    mut mobs: Query<&mut Transform, (With<Mob>, Without<Player>)>,
    player_transform: Query<&Transform, (With<Player>, Without<Mob>)>,
    time: ResMut<Time>,
) {
    let player_transform = player_transform.single();
    for mut mob_transform in mobs.iter_mut() {
        let dir = (mob_transform.translation - player_transform.translation).normalize() * -1.0;

        mob_transform.translation += MOB_MOVE_SPEED * dir * time.delta_seconds();
    }
}

pub fn collision_with_player(mut collision_events: EventReader<CollisionEvent>, world: &World) {
    for collision_event in collision_events.read() {
       match collision_event {
				CollisionEvent::Started(a, b, _) => {
					for i in world.inspect_entity(*b) {
						info!("{}", i.name());
					}
				},
        CollisionEvent::Stopped(_, _, _) => {},
			 }
    }
}
