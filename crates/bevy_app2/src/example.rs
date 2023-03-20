use bevy_ecs::schedule::{common_conditions::*};
use bevy_ecs::system::{Res, ResMut, Resource};
use bevy_ecs::world::World;

use crate::{App, PluginError, PluginHelper};
use crate::main_schedule::Update;
use crate::plugin::{IntoPluginConfigs, Plugin, PluginBuilder, PluginResult};

#[derive(Resource)]
struct SomeResource;

impl SomeResource {
    fn do_something(&self) {}
}

#[derive(Resource)]
struct SomeOtherResource;

struct MyOtherPlugin;

impl Plugin for MyOtherPlugin {
    fn build(&self, _build: &mut PluginBuilder) {}
}


struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, _build: &mut PluginBuilder) {}
}

fn some_system(_resource: Res<SomeResource>) {
    todo!()
}

fn _example() {
    App::new()
        // These all work as usual
        .add_plugin(MyPlugin)
        .add_resource(SomeResource)
        .add_systems(Update, some_system)

        // Plugins can be configured, like systems
        .add_plugin(MyOtherPlugin.before::<MyPlugin>())

        // Anonymous plugins can be created from systems with a PluginResult return
        .add_plugin(|_world: &mut World| -> PluginResult {
            todo!()
        })
        .add_plugin(|some_resource: Option<ResMut<SomeResource>>| -> PluginResult {
            let Some(some_resource) = some_resource else {
                return Err(PluginError::Retry);
            };
            some_resource.do_something();
            Ok(())
        })

        // [App::add_resource] and [App::add_systems] are just wrappers around anonymous plugins
        .add_plugin(PluginHelper::resource(SomeResource))
        .add_plugin(PluginHelper::resource(SomeResource).load_if(resource_exists::<SomeOtherResource>()))
    ;
}
