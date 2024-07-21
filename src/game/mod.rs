//! Game mechanics and content.

use bevy::prelude::*;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

mod animation;
pub mod assets;
pub mod audio;
mod movement;
pub mod spawn;
pub mod trigger;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        RapierDebugRenderPlugin::default(),
        trigger::plugin,
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
    ));
}
