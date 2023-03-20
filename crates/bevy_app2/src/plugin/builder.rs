use bevy_ecs::prelude::{IntoSystemConfigs, Resource, Schedule};
use bevy_ecs::schedule::{ScheduleLabel, SystemConfigs};

use crate::{add_resource, add_schedule, add_systems, init_resource, init_schedule, PluginConfigs};
use crate::plugin::IntoPluginConfigs;

#[derive(Default)]
pub struct PluginBuilder {
    plugins: Vec<PluginConfigs>,
}

impl PluginBuilder {
    pub fn new() -> Self {
        Self {
            plugins: vec![],
        }
    }

    pub fn add_plugins<M>(&mut self, plugin: impl IntoPluginConfigs<M>) -> &mut Self {
        self.plugins.push(plugin.into_plugin_configs());
        self
    }

    pub fn add_resource<T: Resource>(&mut self, value: T) -> &mut Self {
        self.add_plugins(add_resource(value))
    }

    pub fn add_schedule(
        &mut self,
        label: impl ScheduleLabel,
        schedule: Schedule,
    ) -> &mut Self {
        self.add_plugins(add_schedule(label, schedule))
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M> + Send + Sync + 'static,
    ) -> &mut Self {
        self.add_plugins(add_systems(schedule, systems))
    }

    pub fn init_resource<T: Resource + Default>(&mut self) -> &mut Self {
        self.add_plugins(init_resource::<T>())
    }

    pub fn init_schedule(&mut self, label: impl ScheduleLabel) -> &mut Self {
        self.add_plugins(init_schedule(label))
    }
}

pub struct PluginBuilderMarker;

impl<M> IntoPluginConfigs<(PluginBuilderMarker, M)> for PluginBuilder {
    fn into_plugin_configs(self) -> PluginConfigs {
        PluginConfigs::Configs(self.plugins)
    }
}
