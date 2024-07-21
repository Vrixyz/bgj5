//! Handles rapier triggers and reactions

use bevy::prelude::*;
use bevy_eventlistener::{event_listener::EntityEvent, EventListenerPlugin};
use bevy_eventlistener_derive::EntityEvent;
use bevy_rapier2d::pipeline::CollisionEvent;

use super::assets::ImageKey;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<OnTriggerEvent>();
    app.add_plugins(EventListenerPlugin::<OnTriggerEvent>::default());
    app.add_systems(Update, trigger_react);
}

#[derive(Component)]
pub struct OnTrigger;

#[derive(Clone, Debug, Event, EntityEvent)]
pub struct OnTriggerEvent {
    #[target] // Marks the field of the event that specifies the target entity
    pub target: Entity,
    pub other: Entity,
}

pub fn trigger_react(
    q_on_trigger: Query<&OnTrigger>,
    mut collision_events: EventReader<CollisionEvent>,
    mut event_trigger: EventWriter<OnTriggerEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, flags) => {
                if let Ok(_on_trigger) = q_on_trigger.get(*e1) {
                    dbg!("sending for e1");
                    event_trigger.send(OnTriggerEvent {
                        target: *e1,
                        other: *e2,
                    });
                }
                if let Ok(on_trigger) = q_on_trigger.get(*e2) {
                    dbg!("sending for e2");
                    event_trigger.send(OnTriggerEvent {
                        target: *e2,
                        other: *e1,
                    });
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
