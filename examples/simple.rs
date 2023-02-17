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

fn draw(mut debug_draw: ResMut<DebugDraw>) {
    debug_draw.draw(DebugLine {
        from: Vec2::new(-50., -50.),
        to: Vec2::new(50., 50.),
        thickness: 3.,
        color: Color::WHITE.into(),
        ..Default::default()
    });

    debug_draw.draw(DebugRectangle {
        position: Vec2::new(-200., 0.),
        size: Vec2::new(100., 100.),
        color: Color::WHITE,
        ..Default::default()
    });

    debug_draw.draw(DebugCircle {
        position: Vec2::new(200., 0.),
        radius: 100.,
        color: Color::WHITE,
        ..Default::default()
    });
}
