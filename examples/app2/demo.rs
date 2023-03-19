use bevy::prelude::*;

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
    fn config(&self, config: &mut impl PluginConfig) {
        config.add(plugin(PluginB))
              .add(resource(A(10)));
    }
}

impl Plugin for PluginB {
    fn config(&self, config: &mut impl PluginConfig) {
        config.add(resource(B(20)).run_if(resource_exists::<A>()));
    }
}

impl Plugin for PluginC {
    fn config(&self, config: &mut impl PluginConfig) {
        config.add(resource(C(30)).run_if(resource_exists::<B>()))
              .add(plugin(PluginA));
    }
}

fn main() {
    App::new()
        .add(plugin(MainSchedulePlugin))
        .add(plugin(PluginC))
        .add(systems(PreUpdate, || {
            println!("pre update!");
        }))
        .add(systems(Update, |a: Res<A>, b: Res<B>, c: Res<C>| {
            println!("update! {} {} {}", a.0, b.0, c.0);
        }))
        .add(systems(PostUpdate, || {
            println!("post update!");
        }))
        .run();
}
