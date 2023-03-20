use bevy::prelude::*;
use bevy_internal::app::PluginBuilder;

#[derive(Resource)]
struct A(i32);
#[derive(Resource)]
struct B(i32);
#[derive(Resource)]
struct C(i32);

struct PluginA;
struct PluginB;
struct PluginC;

impl Plugin for PluginA {
    fn build(&self, build: &mut PluginBuilder) {
        todo!()
    }
}

impl Plugin for PluginB {
    fn build(&self, build: &mut PluginBuilder) {
        todo!()
    }
}

impl Plugin for PluginC {
    fn build(&self, build: &mut PluginBuilder) {
        todo!()
    }
}

fn main() {
    App::new()
        .add_plugins(MainSchedulePlugin)
        .add_plugins(PluginC)
        .add_systems(PreUpdate, || {
            println!("pre update!");
        })
        .add_systems(Update, |a: Res<A>, b: Res<B>, c: Res<C>| {
            println!("update! {} {} {}", a.0, b.0, c.0);
        })
        .add_systems(PostUpdate, || {
            println!("post update!");
        })
        .run();
}
