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

use super::{
    npc::{DespawnId, SpawnNpc},
    player::SpawnPlayer,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(change_skin);
    app.observe(trigger_react_despawn);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component, Debug)]
pub struct SkinToApply {
    pub key: ImageKey,
}

/// Applied to [`OnTrigger`], this will despawn entities with [`DespawnId`] with that string.
#[derive(Component, Debug)]
pub struct Despawner(pub String);

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

    let text_style = TextStyle {
        font_size: 30.0,
        ..default()
    };

    ///
    /// bevy introduction
    ///
    let position = 700f32;
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnNpc {
        image_key: ImageKey::Mockersf,
        position: Vec2::new(position, 64.0 + 32.0),
        despawn_id: Some("Bevy fans".to_string()),
        ..default()
    });
    commands.trigger(SpawnNpc {
        image_key: ImageKey::Joshua,
        position: Vec2::new(position + 120.0, 64.0 + 32.0),
        despawn_id: Some("Bevy fans".to_string()),
        ..default()
    });
    commands.spawn((
        Name::new("Trigger fans"),
        Collider::ball(320.0),
        Sensor,
        SpatialBundle {
            transform: Transform::from_translation(
                Vec2::new(position + 300.0, 64.0 + 32.0).extend(0f32),
            ),
            ..default()
        },
        OnTrigger,
        Despawner("Bevy fans".to_string()),
        SkinToApply {
            key: ImageKey::Bavy,
        },
        DespawnId("Bevy fans".to_string()),
    ));
    commands.spawn((
        DespawnId("Bevy fans".to_string()),
        Text2dBundle {
            text: Text::from_section("Have you heard of Bevy ?", text_style.clone())
                .with_justify(JustifyText::Center),
            transform: Transform::from_translation(Vec3::new(
                position + 120.0,
                64.0 + 128.0 + 32.0,
                0.0,
            )),
            ..default()
        },
    ));

    ///
    /// Job
    ///
    let position = 1400f32;
    commands.trigger(SpawnNpc {
        image_key: ImageKey::Job,
        position: Vec2::new(position, 64.0 + 32.0),
        despawn_id: Some("Bevy job".to_string()),
        ..default()
    });
    commands.spawn((
        DespawnId("Bevy job".to_string()),
        Text2dBundle {
            text: Text::from_section(
                "Hey you look capable! What about getting a job?",
                text_style.clone(),
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_translation(Vec3::new(position, 64.0 + 128.0 + 32.0, 0.0)),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("Trigger job"),
        Collider::ball(320.0),
        Sensor,
        SpatialBundle {
            transform: Transform::from_translation(
                Vec2::new(position + 300.0, 64.0 + 32.0).extend(0f32),
            ),
            ..default()
        },
        OnTrigger,
        Despawner("Bevy job".to_string()),
        SkinToApply { key: ImageKey::Job },
        DespawnId("Bevy job".to_string()),
    ));

    ///
    /// bevy dev
    ///
    let position = 2300f32;
    commands.trigger(SpawnNpc {
        image_key: ImageKey::Joshua,
        position: Vec2::new(position, 64.0 + 32.0),
        despawn_id: Some("Bevy dev".to_string()),
        ..default()
    });
    commands.spawn((
        DespawnId("Bevy dev".to_string()),
        Text2dBundle {
            text: Text::from_section(
                "A bevy user is a bevy developer who doesn't know it yet.",
                text_style.clone(),
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_translation(Vec3::new(position, 64.0 + 128.0 + 32.0, 0.0)),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("Trigger dev"),
        Collider::ball(320.0),
        Sensor,
        SpatialBundle {
            transform: Transform::from_translation(
                Vec2::new(position + 300.0, 64.0 + 32.0).extend(0f32),
            ),
            ..default()
        },
        OnTrigger,
        Despawner("Bevy dev".to_string()),
        SkinToApply { key: ImageKey::Job },
        DespawnId("Bevy dev".to_string()),
    ));
}

pub fn change_skin(
    trigger: Trigger<OnTriggerEvent>,
    mut commands: Commands,
    q: Query<&SkinToApply>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    dbg!("change_skin");
    if let Ok(to_apply) = q.get(trigger.event().trigger) {
        commands
            .entity(trigger.event().other)
            .insert(image_handles[&to_apply.key].clone_weak());
    }
}
pub fn trigger_react_despawn(
    trigger: Trigger<OnTriggerEvent>,
    q: Query<&Despawner>,
    mut commands: Commands,
    q_ids: Query<(Entity, &DespawnId)>,
) {
    dbg!("trigger_react_despawn");
    if let Ok(to_despawn) = q.get(trigger.event().trigger) {
        for (e, d) in q_ids.iter() {
            if d.0 == to_despawn.0 {
                commands.entity(e).despawn();
            }
        }
    }
}
