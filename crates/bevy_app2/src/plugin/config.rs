use bevy_ecs::prelude::{Condition};
use bevy_ecs::schedule::BoxedCondition;

use crate::plugin::{IntoPlugin, Plugin};

pub struct PluginConfig {
    system: Box<dyn Plugin>,
    conditions: Vec<BoxedCondition>,
}

pub enum PluginConfigs {
    PluginConfig(PluginConfig),
    Configs(Vec<PluginConfigs>),
}

pub trait IntoPluginConfigs<Marker> {
    fn into_plugin_configs(self) -> PluginConfigs;

    fn before<T: Plugin>(self) -> PluginConfigs;
    fn after<T: Plugin>(self) -> PluginConfigs;

    fn load_if<M>(self, condition: impl Condition<M>) -> PluginConfigs;
}

impl IntoPluginConfigs<()> for PluginConfigs {
    fn into_plugin_configs(self) -> PluginConfigs {
        todo!()
    }

    fn before<T: Plugin>(self) -> PluginConfigs {
        todo!()
    }

    fn after<T: Plugin>(self) -> PluginConfigs {
        todo!()
    }

    fn load_if<M>(self, _condition: impl Condition<M>) -> PluginConfigs {
        todo!()
    }
}

impl<Marker, F> IntoPluginConfigs<Marker> for F
    where
        F: IntoPlugin<Marker>
{
    fn into_plugin_configs(self) -> PluginConfigs {
        todo!()
    }

    fn before<R: Plugin>(self) -> PluginConfigs {
        todo!()
    }

    fn after<R: Plugin>(self) -> PluginConfigs {
        todo!()
    }

    fn load_if<M>(self, _condition: impl Condition<M>) -> PluginConfigs {
        todo!()
    }
}