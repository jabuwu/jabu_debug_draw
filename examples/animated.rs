use std::f32::consts::{FRAC_PI_2, PI};

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

#[derive(Default)]
struct Draw {
    lines_rotation: f32,
    rectangle_rotation: f32,
    circle_rotation: f32,
}

fn draw(mut local: Local<Draw>, mut debug_render: ResMut<DebugDraw>, time: Res<Time>) {
    const LINES_START: Vec2 = Vec2::new(-300., 0.);
    for angle in -10..=10 {
        debug_render.draw(DebugDrawLine {
            start: LINES_START,
            end: LINES_START
                + Vec2::from_angle(local.lines_rotation + angle as f32 * PI * 0.1)
                    .rotate(Vec2::new(100., 0.)),
            thickness: 3.,
            start_color: Color::RED,
            end_color: Color::RED,
            ..Default::default()
        });
    }
    local.lines_rotation -= time.delta_seconds();

    const RECTANGLE_POSITION: Vec2 = Vec2::new(300., 0.);
    let rectangle_size = Vec2::new(
        50. + (local.rectangle_rotation / 4.).sin() * 20.,
        100. + (local.rectangle_rotation / 8.).cos() * 20.,
    );
    debug_render.draw(DebugDrawRectangle {
        position: RECTANGLE_POSITION,
        size: rectangle_size,
        color: Color::WHITE,
        rotation: local.rectangle_rotation,
        ..Default::default()
    });
    for i in 0..4 {
        let (distance, line_size) = if i % 2 == 0 {
            (rectangle_size.x, rectangle_size.y)
        } else {
            (rectangle_size.y, rectangle_size.x)
        };
        let angle = Vec2::from_angle(i as f32 * FRAC_PI_2 + local.rectangle_rotation);
        let offset = RECTANGLE_POSITION + angle * distance * 0.5 + angle * 10.;
        let perp = angle.perp();
        debug_render.draw(DebugDrawLine {
            start: offset + perp * line_size * 0.5,
            end: offset - perp * line_size * 0.5,
            thickness: 2.,
            start_color: Color::WHITE,
            end_color: Color::WHITE,
            ..Default::default()
        });
    }
    local.rectangle_rotation += time.delta_seconds() * 2.;

    debug_render.draw(DebugDrawCircle {
        position: Vec2::new(0., local.circle_rotation.sin() * 50.),
        radius: 100.,
        segments: 3 + (30. + (local.circle_rotation.sin() * 30.)) as u8,
        color: Color::YELLOW,
        rotation: local.circle_rotation,
        ..Default::default()
    });
    local.circle_rotation -= time.delta_seconds();
}