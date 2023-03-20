use bevy_ecs::prelude::{IntoSystemConfigs, Schedule};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::Resource;
use bevy_ecs::world::World;
pub use plugin::*;

use crate::main_schedule::RunSchedule;
use crate::plugin::IntoPluginConfigs;

mod plugin;
mod example;
pub mod main_schedule;

pub struct App {
    world: World,
    plugins: Vec<PluginConfigs>,
    runner: Box<dyn Fn(App) + Send>,
}

// App running
impl App {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            runner: Box::new(run_once),
            plugins: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut app = std::mem::replace(self, App::new());
        let runner = std::mem::replace(&mut app.runner, Box::new(run_once));

        // app.setup();

        (runner)(app);
    }
}

// Plugin building
impl App {
    fn add_plugin<M>(&mut self, plugin: impl IntoPluginConfigs<M>) -> &mut Self {
        self.plugins.push(plugin.into_plugin_configs());
        self
    }

    fn add_resource<T: Resource>(&mut self, value: T) -> &mut Self {
        self.add_plugin(add_resource(value))
    }

    fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.add_plugin(add_systems(schedule, systems))
    }
}

pub fn run_once(mut app: App) {
    let schedule = app.world.resource::<RunSchedule>();
    let schedule = schedule.dyn_clone();
    app.world.run_schedule_ref(&*schedule);
}
