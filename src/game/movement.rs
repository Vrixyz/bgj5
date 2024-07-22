//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.register_type::<JumpEvent>();
    app.register_type::<IsGrounded>();
    app.register_type::<JumpDelay>();
    app.register_type::<CoyoteTime>();
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(Startup, set_gravity);
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );
    app.add_event::<JumpEvent>();
    // Apply movement based on controls.
    app.register_type::<Movement>();
    app.add_systems(
        Update,
        (
            (
                (compute_is_grounded, compute_can_jump).chain(),
                apply_movement,
            )
                .chain(),
            //wrap_within_window,
        )
            .chain()
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct IsGrounded(pub bool);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CanJump(pub bool);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct JumpDelay(pub Timer);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CoyoteTime(pub Timer);

pub fn set_gravity(mut conf: ResMut<RapierConfiguration>) {
    conf.gravity = Vec2::new(0f32, -981f32 * 2f32);
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.0 = intent;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    /// Note that physics engines may use different unit/pixel ratios.
    pub speed: f32,
}

#[derive(Event, Reflect)]
pub struct JumpEvent(pub Entity);

fn compute_is_grounded(
    rapier_context: Res<RapierContext>,
    mut query: Query<(Entity, &GlobalTransform, &mut IsGrounded)>,
) {
    for (entity, global_transform, mut is_grounded) in query.iter_mut() {
        let options = ShapeCastOptions {
            max_time_of_impact: 10.0,
            target_distance: 0.0,
            stop_at_penetration: true,
            compute_impact_geometry_on_penetration: true,
        };
        let filter = QueryFilter {
            exclude_rigid_body: Some(entity),
            flags: QueryFilterFlags::EXCLUDE_SENSORS,
            ..QueryFilter::default()
        };

        if let Some((_entity, _hit)) = rapier_context.cast_shape(
            global_transform.translation().xy(),
            Rot::default(),
            -Vec2::Y,
            &Collider::ball(64f32),
            options,
            filter,
        ) {
            is_grounded.0 = true;
        } else {
            is_grounded.0 = false;
        }
    }
}

fn compute_can_jump(
    time: Res<Time>,
    mut query: Query<(&mut JumpDelay, &mut CoyoteTime, &IsGrounded, &mut CanJump)>,
) {
    for (mut jump_delay, mut coyote_time, is_grounded, mut can_jump) in query.iter_mut() {
        jump_delay.0.tick(time.delta());
        if is_grounded.0 {
            if jump_delay.0.finished() {
                coyote_time.0.reset();
            }
        } else {
            coyote_time.0.tick(time.delta());
        }
        let new_can_jump = !coyote_time.0.finished() && jump_delay.0.finished();
        if can_jump.0 != new_can_jump {
            can_jump.0 = new_can_jump;
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(
        Entity,
        &MovementController,
        &Movement,
        &CanJump,
        &mut Velocity,
        &mut JumpDelay,
        &mut CoyoteTime,
        &mut GravityScale,
        &mut Transform,
    )>,
    mut jump_event: EventWriter<JumpEvent>,
) {
    for (
        entity,
        controller,
        movement,
        can_jump,
        mut velocity,
        mut jump_delay,
        mut coyote_time,
        mut gravity,
        mut transform,
    ) in &mut movement_query
    {
        let wanted_velocity = movement.speed * controller.0.x;
        transform.translation.x += wanted_velocity * time.delta_seconds();
        if controller.0.y > 0.01f32 {
            if can_jump.0 {
                jump_delay.0.reset();
                let dur = coyote_time.0.remaining();
                coyote_time.0.tick(dur);
                velocity.linvel.y = 600.0;
                jump_event.send(JumpEvent(entity));
            }
            if velocity.linvel.y > 0.1f32 {
                gravity.0 = 0.5f32;
            } else {
                gravity.0 = 1f32;
            }
        } else if controller.0.y < 0.01f32 {
            gravity.0 = 1.5f32;
        }
    }
}
