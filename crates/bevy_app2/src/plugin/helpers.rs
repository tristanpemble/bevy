use std::any::type_name;

use once_cell::sync::{Lazy, OnceCell};

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::{IntoSystemConfigs, Resource, Schedule, Schedules, World};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::NonSend;

use crate::plugin::{Plugin, PluginCounter};
use crate::plugin::schedule::{PluginInner};

#[derive(Default)]
pub struct PendingPlugins {
    pub(crate) inner: Vec<Box<dyn FnOnce(&mut World)>>,
}

fn register_plugin<T: Plugin>(plugin: T) -> impl FnOnce(&mut World) {
    let mut cell = Some(plugin);
    move |world: &mut World| {
        let Some(plugin) = cell.take() else {
            return;
        };
        plugin.config(world);
    }
}

pub fn plugin<T: Plugin + 'static>(plugin: T) -> impl FnMut(&mut World) {
    let mut cell = Some(plugin);
    move |world: &mut World| {
        let Some(plugin) = cell.take() else {
            return;
        };
        println!("plugin: {:?}", type_name::<T>());
        world.resource_mut::<PluginCounter>().incr();
        world.non_send_resource_mut::<PendingPlugins>().inner.push(Box::new(register_plugin(plugin)));
    }
}

pub fn resource<R: Resource>(resource: R) -> impl FnMut(&mut World) {
    let mut cell = Some(resource);

    move |world: &mut World| {
        let Some(resource) = cell.take() else {
            return;
        };
        println!("resource: {:?}", type_name::<R>());
        world.resource_mut::<PluginCounter>().incr();
        world.insert_resource(resource);
    }
}

pub fn schedule(
    label: impl ScheduleLabel,
    schedule: Schedule,
) -> impl FnMut(&mut World) {
    let mut label = Some(label);
    let mut schedule = Some(schedule);
    move |world: &mut World| {
        let (Some(label), Some(schedule)) = (label.take(), schedule.take()) else {
            return;
        };
        println!("schedule: {:?}", label.dyn_clone());
        world.resource_mut::<PluginCounter>().incr();
        let mut schedules = world.resource_mut::<Schedules>();
        if let None = schedules.get_mut(&label) {
            schedules.insert(label, schedule);
        }
    }
}

pub fn systems<M>(
    schedule: impl ScheduleLabel,
    systems: impl IntoSystemConfigs<M>,
) -> impl FnMut(&mut World) {
    let mut label = Some(schedule);
    let mut systems = Some(systems);
    move |world: &mut World| {
        let (Some(label), Some(systems)) = (label.take(), systems.take()) else {
            return;
        };
        println!("systems: {:?}", label);
        world.resource_mut::<PluginCounter>().incr();
        let mut schedules = world.resource_mut::<Schedules>();
        if let Some(schedule) = schedules.get_mut(&label) {
            schedule.add_systems(systems);
        } else {
            let mut schedule = Schedule::new();
            schedule.add_systems(systems);
            schedules.insert(label, schedule);
        }
    }
}

