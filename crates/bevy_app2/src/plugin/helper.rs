use bevy_ecs::prelude::{FromWorld, IntoSystemConfigs, Schedule, Schedules};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::{ResMut, Resource};
use bevy_ecs::world::World;

use crate::plugin::{IntoPluginConfigs, PluginState, PluginSystem};

pub fn init_resource<T: Resource + Default>() -> PluginSystem {
    PluginSystem::new(move |world: &mut World| -> PluginState {
        world.init_resource::<T>();
        PluginState::Loaded
    })
}

pub fn init_schedule(label: impl ScheduleLabel) -> PluginSystem {
    let mut cell = Some(label);
    PluginSystem::new(move |mut schedules: ResMut<Schedules>| -> PluginState {
        let Some(label) = cell.take() else {
            return PluginState::Failed(None);
        };
        if let None = schedules.get(&label) {
            return PluginState::Failed(None);
        };
        schedules.insert(label, Schedule::new());
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

pub fn add_schedule(label: impl ScheduleLabel, schedule: Schedule) -> PluginSystem {
    let mut cell = Some((label, schedule));
    PluginSystem::new(move |mut schedules: ResMut<Schedules>| -> PluginState {
        let Some((label, schedule)) = cell.take() else {
            return PluginState::Failed(None);
        };
        if let Some(_) = schedules.get(&label) {
            return PluginState::Failed(None);
        };
        schedules.insert(label, schedule);
        PluginState::Loaded
    })
}

pub fn add_systems<M>(
    schedule: impl ScheduleLabel,
    systems: impl IntoSystemConfigs<M> + Send + Sync + 'static,
) -> PluginSystem {
    let mut cell = Some((schedule, systems));
    PluginSystem::new(move |mut schedules: ResMut<Schedules>| -> PluginState {
        let Some((label, systems)) = cell.take() else {
            return PluginState::Failed(None);
        };
        if let None = schedules.get(&label) {
            return PluginState::Failed(None);
        };
        schedules.get_mut(&label).unwrap().add_systems(systems);
        PluginState::Loaded
    })
}
