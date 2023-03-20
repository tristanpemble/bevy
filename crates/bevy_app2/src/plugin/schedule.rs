use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::world::World;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PluginSchedule;

impl PluginSchedule {
    pub fn run(world: &mut World) {
        // let mut prev = 0;
        // loop {
        //     let mut pending = world.non_send_resource_mut::<PendingPlugins>();
        //     let mut pending = std::mem::take(&mut pending.inner);
        //     for plugin in pending.drain(..) {
        //         plugin(world);
        //     }
        //     world.try_run_schedule_ref(&PluginInner);
        //     prev = match world.resource::<PluginCounter>().0 {
        //         current if prev == current => break,
        //         current => current,
        //     };
        // };
    }
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PluginInner;
