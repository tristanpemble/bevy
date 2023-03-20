use bevy_ecs::schedule::{common_conditions::*};
use bevy_ecs::system::{Res, ResMut, Resource};
use bevy_ecs::world::World;

use crate::{add_resource, App};
use crate::main_schedule::Update;
use crate::plugin::{IntoPluginConfigs, Plugin, PluginBuilder, PluginState};

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
    // ...
}

fn _example() {
    App::new()
        // These all work as usual
        .add_resource(SomeResource)
        .add_systems(Update, some_system)
        .add_plugins(MyPlugin)

        // Only, now you can add them in groups
        .add_plugins((MyPlugin, MyOtherPlugin))

        // Plugins can be configured, like systems
        .add_plugins(MyOtherPlugin.when(resource_exists::<SomeResource>()))
        .add_plugins(MyOtherPlugin.optionally_when(resource_exists::<SomeResource>()))
        .add_plugins((
            MyPlugin,
            MyOtherPlugin,
        ).when(resource_exists::<SomeResource>()))

        // Anonymous plugins can be created from any system that returns a PluginState. This is
        // useful for the situations where you used to need access to the App's world.
        .add_plugins(|world: &mut World| -> PluginState {
            PluginState::Loaded
        })
        .add_plugins(|some_resource: Option<ResMut<SomeResource>>| -> PluginState {
            match some_resource {
                Some(some_resource) => {
                    some_resource.do_something();
                    PluginState::Loaded
                }
                _ => PluginState::Idle,
            }
        })

        // [App::add_resource] and [App::add_systems] are just wrappers around anonymous plugins
        .add_plugins(add_resource(SomeResource))
        .add_plugins(add_resource(SomeResource).when(resource_exists::<SomeOtherResource>()))
    ;
}
