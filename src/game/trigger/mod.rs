//! Handles rapier triggers and reactions

use bevy::prelude::*;
use bevy_rapier2d::pipeline::CollisionEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, trigger_react);
}

#[derive(Component)]
pub struct OnTrigger;

#[derive(Clone, Debug, Event)]
pub struct OnTriggerEvent {
    pub trigger: Entity,
    pub other: Entity,
}

pub fn trigger_react(
    mut commands: Commands,
    q_on_trigger: Query<&OnTrigger>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flags) => {
                if let Ok(_on_trigger) = q_on_trigger.get(*e1) {
                    dbg!("sending for e1");
                    commands.trigger(OnTriggerEvent {
                        trigger: *e1,
                        other: *e2,
                    })
                }
                if let Ok(_on_trigger) = q_on_trigger.get(*e2) {
                    dbg!("sending for e2");
                    commands.trigger(OnTriggerEvent {
                        trigger: *e2,
                        other: *e1,
                    });
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
