use std::f32::consts::TAU;

use bevy::prelude::*;
use jabu_debug_draw::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH: f32 = 1380.;
const WINDOW_HEIGHT: f32 = 820.;

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

#[derive(Debug, Clone, Copy, Default)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
}

struct Draw {
    particles: Vec<Particle>,
}

impl Default for Draw {
    fn default() -> Self {
        let mut rng = thread_rng();
        let mut particles = vec![];
        for _ in 0..300 {
            let x = rng.gen_range((WINDOW_WIDTH * -0.5)..(WINDOW_WIDTH * 0.5));
            let y = rng.gen_range((WINDOW_HEIGHT * -0.5)..(WINDOW_HEIGHT * 0.5));
            let angle = rng.gen_range(0.0..TAU);
            let speed = rng.gen_range(1.0..=20.0);
            particles.push(Particle {
                position: Vec2::new(x, y),
                velocity: Vec2::from_angle(angle).rotate(Vec2::X * speed),
            });
        }
        Self { particles }
    }
}

fn draw(mut local: Local<Draw>, mut debug_render: ResMut<DebugDraw>, time: Res<Time>) {
    for particle in local.particles.iter_mut() {
        particle.position += particle.velocity * time.delta_seconds();
        if particle.position.x < WINDOW_WIDTH * -0.5 {
            particle.position.x = WINDOW_WIDTH * 0.5;
        }
        if particle.position.x > WINDOW_WIDTH * 0.5 {
            particle.position.x = WINDOW_WIDTH * -0.5;
        }
        if particle.position.y < WINDOW_HEIGHT * -0.5 {
            particle.position.y = WINDOW_HEIGHT * 0.5;
        }
        if particle.position.y > WINDOW_HEIGHT * 0.5 {
            particle.position.y = WINDOW_HEIGHT * -0.5;
        }
    }
    for particle in local.particles.iter() {
        debug_render.draw(DebugDrawCircle {
            position: particle.position,
            radius: 2.,
            segments: 8,
            color: Color::WHITE,
            ..Default::default()
        });
        const MAX_DISTANCE: f32 = 100.;
        for other_particle in local.particles.iter() {
            let distance = Vec2::distance(particle.position, other_particle.position);
            let mut strength = (MAX_DISTANCE - distance) / MAX_DISTANCE;
            if strength > 0. {
                strength = strength.powf(4.);
                let color = Color::rgba(1., 1., 1., strength);
                debug_render.draw(DebugDrawLine {
                    from: particle.position,
                    to: other_particle.position,
                    color: color.into(),
                    ..Default::default()
                });
            }
        }
    }
}
