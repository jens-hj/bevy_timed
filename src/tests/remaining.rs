use crate::{timed_system, Timed, TimedPlugin};
use bevy::{ecs::system::RunSystemOnce, prelude::*};
use std::time::Duration;

#[test]
fn test_timer_remaining() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TimedPlugin));

    // Spawn entity with timed hide
    let entity = app
        .world_mut()
        .spawn((Timed::hide_after(1.0), Visibility::Visible))
        .id();

    // Advance time by 0.3 seconds
    app.world_mut()
        .resource_mut::<Time<Real>>()
        .advance_by(Duration::from_secs_f32(0.3));
    assert!(app.world_mut().run_system_once(timed_system).is_ok());

    // Advance time by 0.3 seconds
    app.world_mut()
        .resource_mut::<Time<Real>>()
        .advance_by(Duration::from_secs_f32(0.3));
    assert!(app.world_mut().run_system_once(timed_system).is_ok());

    // Check remaining time
    let remaining = app
        .world_mut()
        .get_entity(entity)
        .unwrap()
        .get::<Timed>()
        .unwrap()
        .remaining;

    // get elapsed time
    let elapsed = app.world_mut().resource::<Time<Real>>().elapsed_secs();
    assert!((elapsed - 0.6).abs() < 0.01);
    assert!(remaining.as_secs_f32() > 0.3 && remaining.as_secs_f32() < 0.6);
}
