use crate::{timed_system, Despawn, Timed, TimedPlugin};
use bevy::{ecs::system::RunSystemOnce, prelude::*};
use std::time::Duration;

#[test]
fn test_despawn_timer() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TimedPlugin));

    // Spawn entity with timed despawn
    let entity = app
        .world_mut()
        .spawn((Timed::from_seconds(Despawn, 0.5),))
        .id();

    // Entity should exist initially
    assert!(app.world().get_entity(entity).is_ok());

    // Advance time by 0.3 seconds
    app.world_mut()
        .resource_mut::<Time<Real>>()
        .advance_by(Duration::from_secs_f32(0.3));
    assert!(app.world_mut().run_system_once(timed_system).is_ok());

    // Entity should still exist
    assert!(app.world().get_entity(entity).is_ok());

    // Advance time past the timer
    app.world_mut()
        .resource_mut::<Time<Real>>()
        .advance_by(Duration::from_secs_f32(0.3));
    assert!(app.world_mut().run_system_once(timed_system).is_ok());

    // Entity should be despawned
    assert!(app.world().get_entity(entity).is_err());
}
