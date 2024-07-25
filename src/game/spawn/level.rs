//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_rapier2d::geometry::{Collider, Sensor};

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        trigger::{OnTrigger, OnTriggerEvent},
    },
    screen::Screen,
};

use super::{
    npc::{DespawnId, SpawnNpc},
    player::SpawnPlayer,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(change_skin);
    app.observe(trigger_react_despawn);
    app.observe(trigger_game_over);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component, Debug)]
pub struct TriggerGameOver;

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
    let mut position = 0.0;
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

    commands.spawn((Text2dBundle {
        text: Text::from_section("use arrow keys ->", text_style.clone())
            .with_justify(JustifyText::Center),
        transform: Transform::from_translation(Vec3::new(
            position - 120.0,
            64.0 + 128.0 + 32.0 + 40.0,
            0.0,
        )),
        ..default()
    },));

    //
    // bevy introduction
    //
    position += 700.0;
    let despawn_id = "bevy fans".to_string();
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnNpc {
        image_key: ImageKey::Mockersf,
        position: Vec2::new(position, 64.0 + 32.0),
        despawn_id: Some(despawn_id.clone()),
    });
    commands.trigger(SpawnNpc {
        image_key: ImageKey::Joshua,
        position: Vec2::new(position + 120.0, 64.0 + 32.0),
        despawn_id: Some(despawn_id.clone()),
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
        Despawner(despawn_id.clone()),
        SkinToApply {
            key: ImageKey::Bavy,
        },
        DespawnId(despawn_id.clone()),
    ));
    commands.spawn((Text2dBundle {
        text: Text::from_section("Have you heard of Bevy ?", text_style.clone())
            .with_justify(JustifyText::Center),
        transform: Transform::from_translation(Vec3::new(position, 64.0 + 128.0 + 32.0, 0.0)),
        ..default()
    },));

    for i in 0..3 {
        //
        // Job
        //

        let despawn_id = format!("bevy job {}", i);
        position += 1300.0 + 400.0 * i as f32;
        commands.trigger(SpawnNpc {
            image_key: ImageKey::Job,
            position: Vec2::new(position, 64.0 + 32.0),
            despawn_id: Some(despawn_id.clone()),
        });
        commands.spawn((Text2dBundle {
            text: Text::from_section(
                [
                    "Hey you look capable! What about getting a job?",
                    "Wow your contributions are great! Let's make business together!",
                    "Please help our company scale with Bevy!",
                ][i.min(2)],
                text_style.clone(),
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_translation(Vec3::new(
                position - 100.0,
                64.0 + 128.0 + 32.0,
                0.0,
            )),
            ..default()
        },));
        commands.spawn((
            Name::new(despawn_id.clone()),
            Collider::ball(320.0),
            Sensor,
            SpatialBundle {
                transform: Transform::from_translation(
                    Vec2::new(position + 300.0, 64.0 + 32.0).extend(0f32),
                ),
                ..default()
            },
            OnTrigger,
            Despawner(despawn_id.clone()),
            SkinToApply { key: ImageKey::Job },
            DespawnId(despawn_id.clone()),
        ));

        //
        // bevy dev
        //
        let despawn_id = format!("bevy dev {}", i);
        position += 1400.0 + 600.0 * i as f32;
        commands.trigger(SpawnNpc {
            image_key: ImageKey::Dev,
            position: Vec2::new(position, 64.0 + 32.0),
            despawn_id: Some(despawn_id.clone()),
        });
        commands.spawn((Text2dBundle {
            text: Text::from_section(
                [
                    "A bevy user is a bevy developer who doesn't know it yet.",
                    "There's so many areas to bevy, let's make it even better!",
                    "SME is for Subject Matter Experts, working with them is great!",
                ][i.min(2)],
                text_style.clone(),
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_translation(Vec3::new(
                position - 120.0,
                64.0 + 128.0 + 32.0,
                0.0,
            )),
            ..default()
        },));
        commands.spawn((
            Name::new(despawn_id.clone()),
            Collider::ball(320.0),
            Sensor,
            SpatialBundle {
                transform: Transform::from_translation(
                    Vec2::new(position + 300.0, 64.0 + 32.0).extend(0f32),
                ),
                ..default()
            },
            OnTrigger,
            Despawner(despawn_id.clone()),
            SkinToApply { key: ImageKey::Dev },
            DespawnId(despawn_id.clone()),
        ));
    }
    position += 2000.0;
    commands.spawn((
        Name::new("Trigger super"),
        Collider::ball(320.0),
        Sensor,
        SpatialBundle {
            transform: Transform::from_translation(
                Vec2::new(position + 300.0, 64.0 + 32.0).extend(0f32),
            ),
            ..default()
        },
        OnTrigger,
        Despawner("superbevy".to_string()),
        SkinToApply {
            key: ImageKey::SuperBevy,
        },
        DespawnId("superbevy".to_string()),
    ));
    commands.spawn((Text2dBundle {
        text: Text::from_section("Yeah that was the bevy cyle", text_style.clone())
            .with_justify(JustifyText::Center),
        transform: Transform::from_translation(Vec3::new(
            position - 120.0,
            64.0 + 128.0 + 32.0,
            0.0,
        )),
        ..default()
    },));

    position += 2300.0;
    commands.spawn((Text2dBundle {
        text: Text::from_section("Thanks for 'playing'", text_style.clone())
            .with_justify(JustifyText::Center),
        transform: Transform::from_translation(Vec3::new(
            position - 120.0,
            64.0 + 128.0 + 32.0,
            0.0,
        )),
        ..default()
    },));
    position += 2600.0;
    commands.spawn((Text2dBundle {
        text: Text::from_section("When is editor ?", text_style.clone())
            .with_justify(JustifyText::Center),
        transform: Transform::from_translation(Vec3::new(
            position - 120.0,
            64.0 + 128.0 + 32.0,
            0.0,
        )),
        ..default()
    },));
    position += 3000.0;
    commands.spawn((Text2dBundle {
        text: Text::from_section("Stop now it's over!", text_style.clone())
            .with_justify(JustifyText::Center),
        transform: Transform::from_translation(Vec3::new(
            position - 120.0,
            64.0 + 128.0 + 32.0,
            0.0,
        )),
        ..default()
    },));

    position += 3500.0;
    commands.spawn((
        Name::new("Trigger gameover"),
        Collider::ball(320.0),
        Sensor,
        SpatialBundle {
            transform: Transform::from_translation(
                Vec2::new(position + 800.0, 64.0 + 32.0).extend(0f32),
            ),
            ..default()
        },
        OnTrigger,
        TriggerGameOver,
    ));
    commands.spawn((Text2dBundle {
        text: Text::from_section(
            "Hire me next year? Comment 'I hire you'!",
            text_style.clone(),
        )
        .with_justify(JustifyText::Center),
        transform: Transform::from_translation(Vec3::new(
            position - 120.0,
            64.0 + 128.0 + 32.0,
            0.0,
        )),
        ..default()
    },));
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

pub fn trigger_game_over(
    trigger: Trigger<OnTriggerEvent>,
    q: Query<&TriggerGameOver>,
    mut next_gamestate: ResMut<NextState<Screen>>,
) {
    dbg!("change_skin");
    if let Ok(_to_apply) = q.get(trigger.event().trigger) {
        next_gamestate.set(Screen::Loading);
    }
}
