use bevy::prelude::*;

use crate::systems::*;

pub struct TimedPlugin;

impl Plugin for TimedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, timed_system);
    }
}
