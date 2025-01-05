//! A Bevy plugin for handling timed actions.
//!
//! The simplest usage example is despawning an entity after a certain amount of time.
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_timed::{Despawn, Timed};
//!
//! fn spawn_entity(mut commands: Commands) {
//!     commands.spawn((Transform::default(), Timed::from_seconds(Despawn, 1.0)));
//! }
//! ```

mod actions;
mod components;
mod plugins;
mod systems;
#[cfg(test)]
mod tests;
mod traits;

pub use actions::*;
pub use components::*;
pub use plugins::*;
pub use systems::*;
pub use traits::*;
