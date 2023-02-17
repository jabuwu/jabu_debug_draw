use bevy::prelude::*;

use crate::{DebugDrawDrawable, DebugDrawMesh, DebugDrawVertex};

#[derive(Clone, Copy, Debug)]
pub struct DebugRectangle {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub color: Color,
    pub depth: f32,
}

impl Default for DebugRectangle {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            rotation: 0.,
            color: Color::BLACK,
            depth: 0.,
        }
    }
}

impl DebugDrawDrawable for DebugRectangle {
    fn to_mesh(&self) -> DebugDrawMesh {
        let rotation = Vec2::from_angle(self.rotation);
        DebugDrawMesh {
            vertices: vec![
                DebugDrawVertex {
                    position: self.position + rotation.rotate(self.size * Vec2::new(0.5, 0.5)),
                    color: self.color,
                },
                DebugDrawVertex {
                    position: self.position + rotation.rotate(self.size * Vec2::new(-0.5, 0.5)),
                    color: self.color,
                },
                DebugDrawVertex {
                    position: self.position + rotation.rotate(self.size * Vec2::new(0.5, -0.5)),
                    color: self.color,
                },
                DebugDrawVertex {
                    position: self.position + rotation.rotate(self.size * Vec2::new(-0.5, -0.5)),
                    color: self.color,
                },
            ],
            indices: vec![0, 1, 2, 3, 2, 1],
            depth: self.depth,
        }
    }
}
