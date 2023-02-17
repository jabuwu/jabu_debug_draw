use bevy::prelude::*;

use crate::{DebugDrawDrawable, DebugDrawMesh, DebugDrawVertex};

#[derive(Clone, Copy, Debug)]
pub struct DebugTriangle {
    pub points: [Vec2; 3],
    pub color: Color,
    pub depth: f32,
}

impl Default for DebugTriangle {
    fn default() -> Self {
        Self {
            points: [Vec2::ZERO, Vec2::ZERO, Vec2::ZERO],
            color: Color::BLACK,
            depth: 0.,
        }
    }
}

impl DebugDrawDrawable for DebugTriangle {
    fn to_mesh(&self) -> DebugDrawMesh {
        let a = self.points[0];
        let b = self.points[1];
        let c = self.points[2];
        let clockwise = b.x * a.y + c.x * b.y + a.x * c.y > a.x * b.y + b.x * c.y + c.x * a.y;
        let vertices = vec![
            DebugDrawVertex {
                position: a,
                color: self.color,
            },
            DebugDrawVertex {
                position: b,
                color: self.color,
            },
            DebugDrawVertex {
                position: c,
                color: self.color,
            },
        ];
        let indices = if clockwise {
            vec![0, 2, 1]
        } else {
            vec![0, 1, 2]
        };
        DebugDrawMesh {
            vertices,
            indices,
            depth: self.depth,
        }
    }
}
