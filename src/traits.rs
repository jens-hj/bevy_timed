use bevy::prelude::*;
use std::time::Duration;

use crate::Timed;

/// Trait for actions that can be performed when a timer completes
pub trait TimerAction: Send + Sync + std::fmt::Debug + 'static {
    fn execute(&self, entity: Entity, world: &mut World);
    fn clone_box(&self) -> Box<dyn TimerAction>;
}

impl Timed {
    pub fn new(action: impl TimerAction + 'static, after: Duration) -> Self {
        Self {
            remaining: after,
            action: Box::new(action),
        }
    }

    pub fn from_seconds(action: impl TimerAction + 'static, seconds: f32) -> Self {
        Self::new(action, Duration::from_secs_f32(seconds))
    }
}
