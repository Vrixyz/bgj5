//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    let ground_size = 1000.0;
    let ground_height = 50.0;

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.5 * -ground_height, 0.0)),
        Collider::cuboid(ground_size, ground_height),
    ));

    commands.trigger(SpawnPlayer);
}
