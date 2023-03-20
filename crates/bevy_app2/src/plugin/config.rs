use bevy_ecs::all_tuples;
use bevy_ecs::prelude::Condition;
use bevy_ecs::schedule::BoxedCondition;

use crate::plugin::{IntoPlugin, Plugin};

pub struct PluginConfig {
    pub(crate) plugin: Box<dyn Plugin>,
    pub(crate) conditions: Vec<BoxedCondition>,
}

pub enum PluginConfigs {
    PluginConfig(PluginConfig),
    Configs(Vec<PluginConfigs>),
}

pub trait IntoPluginConfigs<Marker> where Self: Sized {
    fn into_plugin_configs(self) -> PluginConfigs;

    fn when<M>(self, condition: impl Condition<M>) -> PluginConfigs {
        self.into_plugin_configs().when(condition)
    }
    fn optionally_when<M>(self, condition: impl Condition<M>) -> PluginConfigs {
        self.into_plugin_configs().optionally_when(condition)
    }
}

impl IntoPluginConfigs<()> for PluginConfigs {
    fn into_plugin_configs(self) -> PluginConfigs {
        todo!()
    }
}

impl<Marker, F> IntoPluginConfigs<Marker> for F
    where F: IntoPlugin<Marker>
{
    fn into_plugin_configs(self) -> PluginConfigs {
        todo!()
    }
}

pub struct PluginConfigTupleMarker;

macro_rules! impl_plugin_collection {
    ($(($param: ident, $sys: ident)),*) => {
        impl<$($param, $sys),*> IntoPluginConfigs<(PluginConfigTupleMarker, $($param,)*)> for ($($sys,)*)
        where
            $($sys: IntoPluginConfigs<$param>),*
        {
            #[allow(non_snake_case)]
            fn into_plugin_configs(self) -> PluginConfigs {
                let ($($sys,)*) = self;
                PluginConfigs::Configs(vec![$($sys.into_plugin_configs(),)*])
            }
        }
    }
}

all_tuples!(impl_plugin_collection, 1, 20, P, S);