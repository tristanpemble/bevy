use bevy_ecs::prelude::IntoSystemConfigs;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::{Resource};

use crate::plugin::{IntoPluginConfigs};

mod plugin;
mod example;
mod main_schedule;

pub use plugin::*;

pub struct App;

impl App {
    fn new() -> Self {
        Self {}
    }

    fn add_plugin<M>(&mut self, _plugin: impl IntoPluginConfigs<M>) -> &mut Self {
        todo!()
    }

    fn add_resource<T: Resource>(&mut self, value: T) -> &mut Self {
        self.add_plugin(PluginHelper::resource(value))
    }

    fn add_systems<M>(
        &mut self,
        _schedule: impl ScheduleLabel,
        _systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        todo!()
    }
}
