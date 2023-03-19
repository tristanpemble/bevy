use bevy_ecs::prelude::{IntoSystemConfigs, Resource, Schedule, Schedules, World};
use bevy_ecs::schedule::{ScheduleLabel, SystemConfig, SystemConfigs};
use bevy_ecs::system::IntoSystem;
use schedule::{PluginInner};

use crate::App;
use crate::prelude::{PendingPlugins, schedule};

pub(crate) mod helpers;
pub(crate) mod schedule;

pub trait Plugin {
    fn config(&self, config: &mut impl PluginConfig);
}

pub trait PluginConfig {
    fn add<M>(&mut self, plugin: impl IntoSystemConfigs<M>) -> &mut Self;

    // fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
    //     self.add(helpers::plugin(plugin))
    // }
}

#[derive(Resource, Default)]
pub struct PluginCounter(pub(crate) usize);

impl PluginConfig for World {
    fn add<M>(&mut self, plugin: impl IntoSystemConfigs<M>) -> &mut Self {
        self.init_resource::<Schedules>();
        self.init_non_send_resource::<PendingPlugins>();
        let mut schedules = self.resource_mut::<Schedules>();
        if let Some(schedule) = schedules.get_mut(&PluginInner) {
            schedule.add_systems(plugin);
        } else {
            let mut new_schedule = Schedule::new();
            new_schedule.add_systems(plugin);
            schedules.insert(PluginInner, new_schedule);
        }
        self
    }
}

impl PluginConfig for App {
    fn add<M>(&mut self, plugin: impl IntoSystemConfigs<M>) -> &mut Self {
        self.world.add(plugin);
        self
    }
}

impl PluginCounter {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}
