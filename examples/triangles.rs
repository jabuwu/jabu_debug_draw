use std::f32::consts::TAU;

use bevy::prelude::*;
use jabu_debug_draw::prelude::*;
use rand::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugDrawPlugin)
        .add_startup_system(setup)
        .add_system(draw)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Default)]
struct Draw {
    count: f32,
}

fn draw(mut local: Local<Draw>, mut debug_draw: ResMut<DebugDraw>, time: Res<Time>) {
    let mut rng = StdRng::from_seed(Default::default());
    local.count += time.delta_seconds() * 30.;
    for index in 0..(local.count as usize) {
        let x = rng.gen_range(-800.0..800.0);
        let y = rng.gen_range(-600.0..600.0);
        let origin = Vec2::new(x, y);
        let angle1 = rng.gen_range(0.0..TAU);
        let angle2 = rng.gen_range(0.0..TAU);
        let angle3 = rng.gen_range(0.0..TAU);
        let distance1 = rng.gen_range(10.0..300.);
        let distance2 = rng.gen_range(10.0..300.);
        let distance3 = rng.gen_range(10.0..300.);
        let offset1 = Vec2::from_angle(angle1).rotate(Vec2::X * distance1);
        let offset2 = Vec2::from_angle(angle2).rotate(Vec2::X * distance2);
        let offset3 = Vec2::from_angle(angle3).rotate(Vec2::X * distance3);
        let hue = rng.gen_range(0.0..360.0);
        let intensity = 1. - ((local.count - index as f32) / 200.).clamp(0., 1.);
        debug_draw.draw(DebugTriangle {
            points: [origin + offset1, origin + offset2, origin + offset3],
            color: Color::hsl(hue, 0.8 * intensity, 0.6 * intensity),
            ..Default::default()
        });
    }
}
