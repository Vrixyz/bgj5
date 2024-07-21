//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_eventlistener::{
    callbacks::{Listener, ListenerMut},
    event_listener::On,
};
use bevy_rapier2d::geometry::{Collider, Sensor};

use crate::game::{
    assets::{HandleMap, ImageKey},
    trigger::{OnTrigger, OnTriggerEvent},
};

use super::{npc::SpawnNpc, player::SpawnPlayer};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component, Debug)]
pub struct SkinToApply {
    pub key: ImageKey,
}

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    let ground_size = 1000.0;
    let ground_height = 50.0;

    for i in 0..100 {
        commands.spawn((
            TransformBundle::from(Transform::from_xyz(
                i as f32 * ground_size,
                0.5 * -ground_height,
                0.0,
            )),
            Collider::cuboid(ground_size, ground_height),
        ));
    }

    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnNpc {
        image_key: ImageKey::Mockersf,
        position: Vec2::new(700.0, 64.0 + 32.0),
    });
    commands.trigger(SpawnNpc {
        image_key: ImageKey::Joshua,
        position: Vec2::new(820.0, 64.0 + 32.0),
    });
    commands.spawn((
        Collider::ball(320.0),
        Sensor,
        SpatialBundle {
            transform: Transform::from_translation(Vec2::new(1000.0, 64.0 + 32.0).extend(0f32)),
            ..default()
        },
        OnTrigger,
        SkinToApply {
            key: ImageKey::Bavy,
        },
        On::<OnTriggerEvent>::run(change_skin),
    ));

    commands.trigger(SpawnNpc {
        image_key: ImageKey::Job,
        position: Vec2::new(1200.0, 64.0 + 32.0),
    });
    let text_style = TextStyle {
        font_size: 30.0,
        ..default()
    };
    commands.spawn(Text2dBundle {
        text: Text::from_section("Have you heard of Bevy ?", text_style.clone())
            .with_justify(JustifyText::Center),
        transform: Transform::from_translation(Vec3::new(820.0, 64.0 + 128.0 + 32.0, 0.0)),
        ..default()
    });
}

pub fn change_skin(
    mut commands: Commands,
    trigger: Listener<OnTriggerEvent>,
    q: Query<&SkinToApply>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    dbg!(&trigger);
    if let Ok(to_apply) = q.get(trigger.target) {
        commands
            .entity(trigger.other)
            .insert(image_handles[&to_apply.key].clone_weak());
    }
}
