use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::World;

use crate::{PluginConfigs, PluginState, PluginStates};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PluginSchedule;

impl PluginSchedule {
    pub fn run(world: &mut World) {
        world.init_resource::<PluginStates>();
        // todo: make not infinite
        while world.resource::<PluginStates>().count(PluginState::Idle) > 0 {
            world.run_schedule_ref(&PluginInner);
        };
        world.remove_resource::<PluginStates>();
    }
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PluginInner;
