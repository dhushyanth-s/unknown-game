use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub collider_type: ActiveCollisionTypes,
    pub active_events: ActiveEvents,
}

impl Default for ColliderBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(10.0, 10.0),
            sensor: Sensor,
            collider_type: ActiveCollisionTypes::all(),
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
