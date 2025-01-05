use crate::TimerAction;
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct Despawn;
impl TimerAction for Despawn {
    fn execute(&self, entity: Entity, world: &mut World) {
        world.despawn(entity);
    }
    fn clone_box(&self) -> Box<dyn TimerAction> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Hide;
impl TimerAction for Hide {
    fn execute(&self, entity: Entity, world: &mut World) {
        if let Some(mut visibility) = world.get_mut::<Visibility>(entity) {
            *visibility = Visibility::Hidden;
        }
    }
    fn clone_box(&self) -> Box<dyn TimerAction> {
        Box::new(self.clone())
    }
}
