use bevy_ecs::system::Resource;
use bevy_ecs::world::World;


use crate::plugin::{PluginResult, SystemPlugin};

pub struct PluginHelper;

impl PluginHelper {
    pub fn resource<T: Resource>(value: T) -> SystemPlugin {
        let mut cell = Some(value);
        SystemPlugin::new(move |world: &mut World| -> PluginResult {
            let Some(value) = cell.take() else {
                return Ok(());
            };
            world.insert_resource(value);
            Ok(())
        })
    }
}