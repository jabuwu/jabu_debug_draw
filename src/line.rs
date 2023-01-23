use bevy::prelude::*;

use crate::{DebugDrawDrawable, DebugDrawMesh, DebugDrawVertex};

#[derive(Debug, Clone, Copy)]
pub struct DebugDrawLine {
    pub start: Vec2,
    pub end: Vec2,
    pub start_color: Color,
    pub end_color: Color,
    pub thickness: f32,
    pub depth: f32,
}

impl Default for DebugDrawLine {
    fn default() -> Self {
        Self {
            start: Vec2::ZERO,
            end: Vec2::ZERO,
            start_color: Color::BLACK,
            end_color: Color::BLACK,
            thickness: 1.,
            depth: 0.,
        }
    }
}

impl DebugDrawDrawable for DebugDrawLine {
    fn to_mesh(&self) -> DebugDrawMesh {
        if self.start == self.end {
            DebugDrawMesh::new()
        } else {
            let orthogonal = (self.start - self.end).normalize().perp() * self.thickness * 0.5;
            DebugDrawMesh {
                vertices: vec![
                    DebugDrawVertex {
                        position: self.start - orthogonal,
                        color: self.start_color,
                    },
                    DebugDrawVertex {
                        position: self.start + orthogonal,
                        color: self.start_color,
                    },
                    DebugDrawVertex {
                        position: self.end - orthogonal,
                        color: self.end_color,
                    },
                    DebugDrawVertex {
                        position: self.end + orthogonal,
                        color: self.end_color,
                    },
                ],
                indices: vec![0, 1, 2, 3, 2, 1],
                depth: self.depth,
            }
        }
    }
}
