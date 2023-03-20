use bevy_ecs::prelude::{IntoSystemConfigs, Schedule, Schedules};
use bevy_ecs::schedule::{ExecutorKind, ScheduleLabel};
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
pub use plugin::*;

use crate::main_schedule::AppSchedule;
use crate::plugin::IntoPluginConfigs;

mod plugin;
mod example;
pub mod main_schedule;

pub mod prelude {
    pub use crate::{App, main_schedule::*, Plugin};
}

pub struct App {
    world: World,
    builder: PluginBuilder,
    runner: Box<dyn Fn(World) + Send>,
}

// App running
impl App {
    pub fn new() -> Self {
        let mut builder = PluginBuilder::new();
        builder.init_resource::<AppSchedule>();

        Self {
            world: World::new(),
            runner: Box::new(run_once),
            builder,
        }
    }

    pub fn run(&mut self) {
        let mut app = std::mem::replace(self, App::new());

        let mut schedule = Schedule::new();
        schedule.set_executor_kind(ExecutorKind::MultiThreaded);
        schedule.run(&mut app.world);

        // (app.runner)(app.world);
    }
}

// Plugin building
impl App {
    pub fn init_schedule<M>(
        &mut self,
        label: impl ScheduleLabel,
    ) -> &mut Self {
        self.add_plugins(init_schedule(label))
    }

    pub fn add_plugins<M>(&mut self, plugin: impl IntoPluginConfigs<M>) -> &mut Self {
        self.builder.add_plugins(plugin);
        self
    }

    pub fn add_resource<T: Resource>(&mut self, value: T) -> &mut Self {
        self.builder.add_resource(value);
        self
    }

    pub fn add_schedule<M>(
        &mut self,
        label: impl ScheduleLabel,
        schedule: Schedule,
    ) -> &mut Self {
        self.builder.add_schedule(label, schedule);
        self
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M> + Send + Sync + 'static,
    ) -> &mut Self {
        self.builder.add_systems(schedule, systems);
        self
    }
}

pub fn run_once(mut world: World) {
    let schedule = world.resource::<AppSchedule>();
    let schedule = schedule.dyn_clone();
    world.run_schedule_ref(&*schedule);
}
