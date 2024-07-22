//! Spawn the player.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{CanJump, CoyoteTime, IsGrounded, JumpDelay, Movement, MovementController},
    },
    screen::Screen,
    MainCamera,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
    app.register_type::<UpdateCanJumpLabel>();
    app.add_systems(Update, update_can_jump_label);
    app.add_systems(
        Update,
        follow_camera
            .after(PhysicsSet::Writeback)
            .before(TransformSystem::TransformPropagate),
    );
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct UpdateCanJumpLabel;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(130), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: image_handles[&ImageKey::Ducky].clone_weak(),
            transform: Transform::from_scale(Vec2::new(1f32, 1f32).extend(1.0))
                .with_translation(Vec3::new(0f32, 128f32, 0f32)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        MovementController::default(),
        Movement { speed: 420.0 },
        player_animation,
        StateScoped(Screen::Playing),
        (
            RigidBody::Dynamic,
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            Collider::ball(64f32),
            GravityScale(1f32),
            JumpDelay(Timer::from_seconds(0.25f32, TimerMode::Once)),
            CoyoteTime(Timer::from_seconds(0.25f32, TimerMode::Once)),
            CanJump(false),
            IsGrounded(false),
            ExternalImpulse::default(),
        ),
    ));
    /*
    .with_children(|child_builder| {
        let text_style = TextStyle {
            font_size: 20.0,
            ..default()
        };
        child_builder
            .spawn((
                Text2dBundle {
                    text: Text::from_section("can_jump", text_style.clone())
                        .with_justify(JustifyText::Center),
                    ..default()
                },
                UpdateCanJumpLabel,
            ))
            .insert(Transform::from_translation(Vec3::new(
                0f32,
                16f32 + 10f32,
                0f32,
            )));
    });*/
}

fn update_can_jump_label(
    q_can_jump: Query<&CanJump>,
    mut q_label: Query<(&mut Text, &Parent), With<UpdateCanJumpLabel>>,
) {
    for (mut text, parent) in q_label.iter_mut() {
        text.sections[0] = if q_can_jump.get(parent.get()).unwrap().0 {
            "can_jump"
        } else {
            "no jump"
        }
        .into();
    }
}

fn follow_camera(
    time: Res<Time>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    q_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut camera = q_camera.single_mut();
    if let Ok(player) = q_player.get_single() {
        let mut target = player.translation;
        target.y = 256.0;
        camera.translation = camera
            .translation
            .lerp(target, time.delta_seconds() * 10f32);
    }
}
