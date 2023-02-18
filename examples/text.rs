use bevy::prelude::*;
use jabu_debug_draw::{
    DebugDraw, DebugDrawPlugin, DebugText, DebugTextAlignment, DebugTextVerticalAlignment,
};

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
    hue: f32,
}

fn draw(mut local: Local<Draw>, mut debug_draw: ResMut<DebugDraw>, time: Res<Time>) {
    local.hue = (local.hue + time.delta_seconds() * 100.) % 360.;

    debug_draw.draw(DebugText {
        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna\naliqua. Ut enim ad minim veniam, quis nostrud exercitation\nullamco laboris nisi ut aliquip ex ea commodo consequat.\nDuis aute irure dolor in reprehenderit in voluptate velit\nesse cillum dolore eu fugiat nulla pariatur. Excepteur sint\noccaecat cupidatat non proident, sunt in culpa qui officia\ndeserunt mollit anim id est laborum."
            .to_owned(),
        position: Vec2::ZERO,
        color: Color::hsl(local.hue, 0.9, 0.5),
        scale: 2.,
        depth: 1.,
        alignment: DebugTextAlignment::Center,
        vertical_alignment: DebugTextVerticalAlignment::Center,
        ..Default::default()
    });
}
