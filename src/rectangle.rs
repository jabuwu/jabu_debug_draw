use bevy::prelude::*;

use crate::{DebugDrawDrawable, DebugDrawMesh, DebugDrawVertex};

#[derive(Clone, Copy, Debug)]
pub struct DebugDrawRectangle {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub color: Color,
}

impl Default for DebugDrawRectangle {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            rotation: 0.,
            color: Color::BLACK,
        }
    }
}

impl DebugDrawDrawable for DebugDrawRectangle {
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
        }
    }
}
