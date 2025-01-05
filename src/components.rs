use std::time::Duration;

use crate::traits::TimerAction;
use bevy::prelude::*;

/// Generic timer component that includes both duration and action
#[derive(Component)]
pub struct Timed {
    pub remaining: Duration,
    pub action: Box<dyn TimerAction>,
}

impl Clone for Box<dyn TimerAction> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
