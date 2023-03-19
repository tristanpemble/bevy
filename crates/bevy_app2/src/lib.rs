use std::process::exit;
use std::thread::current;

use bevy_ecs::component::Component;
use bevy_ecs::schedule::{BoxedScheduleLabel, IntoSystemConfigs, Schedule, ScheduleLabel, Schedules};
use bevy_ecs::system::{ResMut, Resource};
use bevy_ecs::world::World;
use main_schedule::MainSchedule;
use plugin::PluginCounter;
use plugin::schedule::{PluginInner, PluginOuter};

use crate::main_schedule::Main;
use crate::plugin::{Plugin, PluginConfig};
use crate::prelude::{PendingPlugins, plugin};

mod main_schedule;
mod plugin;

pub mod prelude {
    pub use crate::{App, main_schedule::*, plugin::{Plugin, PluginConfig}, plugin::helpers::*};
}

pub struct App {
    world: World,
    runner: Box<dyn Fn(App) + Send>,
}

impl App {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            runner: Box::new(run_once),
        }
    }

    fn setup(&mut self) {
        // Initialize plugin schedule
        self.world.init_resource::<Schedules>();
        self.world.init_resource::<PluginCounter>();

        // Add the outer-plugin schedule
        let mut outer = Schedule::new();
        outer.add_systems(|world: &mut World| {
            let mut prev = 0;
            loop {
                let mut pending = world.non_send_resource_mut::<PendingPlugins>();
                let mut pending = std::mem::take(&mut pending.inner);
                for plugin in pending.drain(..) {
                    plugin(world);
                }
                world.try_run_schedule_ref(&PluginInner);
                prev = match world.resource::<PluginCounter>().0 {
                    current if prev == current => break,
                    current => current,
                };
            };
        });

        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.insert(PluginOuter, outer);

        // Bootstrap the plugins
        self.world.run_schedule_ref(&PluginOuter);

        // Remove the plugin schedule
        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.remove(&PluginInner);
        schedules.remove(&PluginOuter);
        self.world.remove_resource::<PluginCounter>();
        self.world.remove_non_send_resource::<PendingPlugins>();
    }

    pub fn run(&mut self) {
        let mut app = std::mem::replace(self, App::new());
        let runner = std::mem::replace(&mut app.runner, Box::new(run_once));

        app.setup();

        (runner)(app);
    }
}

fn run_once(mut app: App) {
    let schedule = app.world.resource::<MainSchedule>();
    let schedule = schedule.0.dyn_clone();

    println!("{:?}", schedule.dyn_clone());

    app.world.run_schedule_ref(&*schedule);
}
