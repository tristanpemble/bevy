use bevy_ecs::prelude::{FromWorld, IntoSystemConfigs};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;

use crate::plugin::{PluginState, PluginSystem};

pub fn init_resource<T: Resource + FromWorld>() -> PluginSystem {
    PluginSystem::new(move |world: &mut World| -> PluginState {
        world.init_resource::<T>();
        PluginState::Loaded
    })
}

pub fn add_resource<T: Resource>(value: T) -> PluginSystem {
    let mut cell = Some(value);
    PluginSystem::new(move |world: &mut World| -> PluginState {
        let Some(value) = cell.take() else {
            return PluginState::Failed(None);
        };
        world.insert_resource(value);
        PluginState::Loaded
    })
}

pub fn add_systems<M>(
    schedule: impl ScheduleLabel,
    systems: impl IntoSystemConfigs<M>,
) -> PluginSystem {
    todo!()
}