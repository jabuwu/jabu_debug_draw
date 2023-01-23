use std::f32::consts::PI;

use bevy::prelude::*;
use jabu_debug_draw::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH: f32 = 1480.;
const WINDOW_HEIGHT: f32 = 820.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugDrawPlugin)
        .add_startup_system(setup)
        .add_system(draw.label("draw"))
        .add_system(traveler_update.label("traveler_update").before("draw"))
        .add_system(traveler_spawn.before("traveler_update"))
        .run();
}

#[derive(Component)]
struct Traveler {
    color: Color,
    velocity: Vec2,
    time_to_live: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw(mut debug_render: ResMut<DebugDraw>, traveler_query: Query<(&Traveler, &Transform)>) {
    for (traveler, traveler_transform) in traveler_query.iter() {
        let bounce = Vec2::new(0., 10.) * (traveler.time_to_live * 6.).cos().abs();
        let rotation = PI * 0.03 * (traveler.time_to_live * 6.).cos();
        debug_render.draw(DebugDrawRectangle {
            position: traveler_transform.translation.truncate() + bounce,
            size: Vec2::splat(100.),
            color: traveler.color,
            rotation,
            depth: -traveler_transform.translation.y,
            ..Default::default()
        });
    }
}

fn traveler_update(
    mut commands: Commands,
    mut traveler_query: Query<(Entity, &mut Traveler, &mut Transform)>,
    time: Res<Time>,
) {
    for (traveler_entity, mut traveler, mut traveler_transform) in traveler_query.iter_mut() {
        traveler_transform.translation += traveler.velocity.extend(0.) * time.delta_seconds();
        traveler.time_to_live -= time.delta_seconds();
        if traveler.time_to_live <= 0. {
            commands.entity(traveler_entity).despawn();
        }
    }
}

#[derive(Default)]
struct TravelerSpawn {
    spawn_time: f32,
}

fn traveler_spawn(mut local: Local<TravelerSpawn>, mut commands: Commands, time: Res<Time>) {
    local.spawn_time += time.delta_seconds();
    if local.spawn_time > 0.1 {
        local.spawn_time -= 0.1;
        let mut rng = thread_rng();
        let (x, velocity_speed) = if rng.gen_bool(0.5) {
            (WINDOW_WIDTH * -0.5, Vec2::new(1., 0.))
        } else {
            (WINDOW_WIDTH * 0.5, Vec2::new(-1., 0.))
        };
        let y = rng.gen_range((WINDOW_HEIGHT * -0.5)..(WINDOW_HEIGHT * 0.5));
        let velocity_angle = rng.gen_range(-0.9..0.9);
        let hue = rng.gen_range(0.0..360.0);
        commands
            .spawn(TransformBundle {
                local: Transform::from_xyz(x, y, 0.),
                ..Default::default()
            })
            .insert(Traveler {
                color: Color::hsl(hue, 0.6, 0.6),
                velocity: Vec2::from_angle(velocity_angle).rotate(velocity_speed * 100.),
                time_to_live: 30.,
            });
    }
}
