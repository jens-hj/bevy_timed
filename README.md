# bevy_timed

## Bevy Compatibility

| Bevy | Bevy Timed |
| ---- | ---------- |
| 0.15 | 0.2.0      |

## Usage

Add the plugin to your Bevy app:

```rust
use bevy::prelude::*;
use bevy_timed::TimedPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TimedPlugin))
        .run();
}
```

Add a timed component to an entity in order to perform an action after a certain amount of time.

```rust
use bevy::prelude::*;
use bevy_timed::Timed;

fn spawn_timed_entity(mut commands: Commands) {
    // Use the explicit `Timed::from_seconds` constructor specifying the action
    commands.spawn((Transform::default(), Timed::from_seconds(Despawn, 0.5)));

    // Use the more semantic `Timed::despawn_after` constructor which is
    // equivalent to the above.
    commands.spawn((Transform::default(), Timed::despawn_after(0.5)));
}
```

## License

Dual licensed under MIT or Apache 2.0.

- [LICENSE-MIT](http://opensource.org/licenses/MIT)
- [LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0)
