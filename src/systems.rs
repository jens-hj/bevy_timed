use std::time::Duration;

use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use crate::Timed;

/// Bevy [`Update`] system to handle [`Timed`] components
pub fn timed_system(world: &mut World) {
    let mut query = SystemState::<Query<(Entity, &mut Timed)>>::new(world);
    let delta = world.resource::<Time<Real>>().delta();
    let mut to_execute = Vec::new();

    {
        let mut query = query.get_mut(world);
        for (entity, mut timer) in &mut query {
            if delta > timer.remaining {
                timer.remaining = Duration::ZERO;
                to_execute.push((entity, timer.action.clone()));
            } else {
                timer.remaining -= delta;
            }
        }
    }

    for (entity, action) in to_execute {
        debug!("Executing action {:?} for entity {:?}", action, entity); // Debug action execution
        action.execute(entity, world);
    }
}
