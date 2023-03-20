use std::error::Error;
use std::time::Duration;

use bevy_ecs::schedule::Schedule;
use bevy_ecs::system::{BoxedSystem, In, IntoPipeSystem, IntoSystem, NonSendMut};
use bevy_utils::Instant;
pub use builder::*;
pub use config::*;
pub use helper::*;
pub use schedule::*;

use crate::plugin::registry::PluginRegistry;

mod builder;
mod config;
mod registry;
mod helper;
mod schedule;

pub trait Plugin {
    fn build(&self, build: &mut PluginBuilder);
    fn register(self, schedule: &mut Schedule) where Self: Sized {
        let mut builder = PluginBuilder::new();
        self.build(&mut builder);
        schedule.add_systems(builder.systems);
    }
}

pub enum PluginState {
    Idle,
    Loaded,
    Failed(Option<Box<dyn Error>>),
}

pub trait IntoPlugin<M> {
    type Plugin: Plugin;

    fn into_plugin(self) -> Self::Plugin;
}

impl<T> IntoPlugin<()> for T where T: Plugin {
    type Plugin = T;

    fn into_plugin(self) -> Self::Plugin {
        self
    }
}

impl<T, M> IntoPlugin<((), M)> for T where T: IntoSystem<(), PluginState, M> {
    type Plugin = PluginSystem;

    fn into_plugin(self) -> Self::Plugin {
        PluginSystem::new(self)
    }
}

pub struct PluginSystem {
    system: BoxedSystem,
}

impl PluginSystem {
    pub fn new<M>(system: impl IntoSystem<(), PluginState, M>) -> Self {
        Self {
            system: Box::new(IntoSystem::into_system(system).pipe(handle_plugin_result)),
            // system: todo!()
        }
    }
}

impl Plugin for PluginSystem {
    fn build(&self, _app: &mut PluginBuilder) {}
    fn register(self, schedule: &mut Schedule) {
        schedule.add_systems(self.system);
    }
}

fn handle_plugin_result(In(_result): In<PluginState>, _registry: NonSendMut<PluginRegistry>) {
    todo!()
}
