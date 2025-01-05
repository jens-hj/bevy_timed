use crate::{timed_system, Timed, TimedPlugin};
use bevy::{ecs::system::RunSystemOnce, prelude::*};
use std::time::Duration;

#[test]
fn test_hide_timer() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TimedPlugin));

    let entity = app
        .world_mut()
        .spawn((Timed::hide_after(0.5), Visibility::Visible))
        .id();

    // Advance time past the timer
    app.world_mut()
        .resource_mut::<Time<Real>>()
        .advance_by(Duration::from_secs_f32(0.3));
    assert!(app.world_mut().run_system_once(timed_system).is_ok());

    // Entity should be visible initially
    assert_eq!(
        app.world()
            .get_entity(entity)
            .unwrap()
            .get::<Visibility>()
            .unwrap(),
        &Visibility::Visible
    );

    // Advance time past the timer
    app.world_mut()
        .resource_mut::<Time<Real>>()
        .advance_by(Duration::from_secs_f32(0.3));
    assert!(app.world_mut().run_system_once(timed_system).is_ok());

    // Entity should be hidden
    assert_eq!(
        app.world()
            .get_entity(entity)
            .unwrap()
            .get::<Visibility>()
            .unwrap(),
        &Visibility::Hidden
    );
}
