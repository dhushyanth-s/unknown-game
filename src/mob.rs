use bevy::prelude::*;

use self::spawn::{setup_spawning, spawn};
use self::movement::{collision_with_player, movement};

mod spawn;
mod movement;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_spawning)
            .add_systems(FixedUpdate, (spawn, movement, collision_with_player));
    }
}
