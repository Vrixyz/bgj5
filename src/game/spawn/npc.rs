//! Spawn the player.

use bevy::{color::palettes, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
    },
    screen::Screen,
    ui::palette,
    MainCamera,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_npc);
    app.register_type::<SpawnNpc>();
}

#[derive(Event, Debug, Reflect)]
pub struct SpawnNpc {
    pub image_key: ImageKey,
    pub position: Vec2,
}

fn spawn_npc(
    _trigger: Trigger<SpawnNpc>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(128), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("NPC"),
        SpriteBundle {
            texture: image_handles[&_trigger.event().image_key].clone_weak(),
            transform: Transform::from_translation(_trigger.event().position.extend(1.0)),
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        player_animation,
        StateScoped(Screen::Playing),
    ));
}
