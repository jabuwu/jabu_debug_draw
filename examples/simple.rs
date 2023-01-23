use bevy::prelude::*;
use jabu_debug_draw::prelude::*;

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

fn draw(mut debug_render: ResMut<DebugDraw>) {
    debug_render.draw(DebugDrawLine {
        start: Vec2::new(-50., -50.),
        end: Vec2::new(50., 50.),
        thickness: 3.,
        start_color: Color::WHITE,
        end_color: Color::WHITE,
        ..Default::default()
    });

    debug_render.draw(DebugDrawRectangle {
        position: Vec2::new(-200., 0.),
        size: Vec2::new(100., 100.),
        color: Color::WHITE,
        ..Default::default()
    });

    debug_render.draw(DebugDrawCircle {
        position: Vec2::new(200., 0.),
        radius: 100.,
        color: Color::WHITE,
        ..Default::default()
    });
}
