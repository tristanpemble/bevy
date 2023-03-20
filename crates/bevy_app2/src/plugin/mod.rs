use bevy_ecs::schedule::{Schedule};
use bevy_ecs::system::{BoxedSystem, In, IntoPipeSystem, IntoSystem, NonSendMut};
pub use builder::*;
pub use config::*;
pub use helper::*;


use crate::plugin::registry::PluginRegistry;

mod builder;
mod config;
mod registry;
mod helper;

pub type PluginResult = Result<(), PluginError>;
pub enum PluginError {
    Retry
}

pub trait Plugin {
    fn build(&self, build: &mut PluginBuilder);
    fn register(self, schedule: &mut Schedule) where Self: Sized {
        let mut builder = PluginBuilder::new();
        self.build(&mut builder);
        schedule.add_systems(builder.systems);
    }
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

impl<T, M> IntoPlugin<((), M)> for T where T: IntoSystem<(), PluginResult, M> {
    type Plugin = SystemPlugin;

    fn into_plugin(self) -> Self::Plugin {
        SystemPlugin::new(self)
    }
}

pub struct SystemPlugin {
    system: BoxedSystem,
}

impl SystemPlugin {
    pub fn new<M>(system: impl IntoSystem<(), PluginResult, M>) -> Self {
        Self {
            system: Box::new(IntoSystem::into_system(system).pipe(handle_plugin_result)),
            // system: todo!()
        }
    }
}

impl Plugin for SystemPlugin {
    fn build(&self, _app: &mut PluginBuilder) {}
    fn register(self, schedule: &mut Schedule) {
        schedule.add_systems(self.system);
    }
}

fn handle_plugin_result(In(_result): In<PluginResult>, _registry: NonSendMut<PluginRegistry>) {
    todo!()
}
