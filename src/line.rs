use bevy::prelude::*;

use crate::{DebugDrawDrawable, DebugDrawMesh, DebugDrawVertex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DebugLine {
    pub from: Vec2,
    pub to: Vec2,
    pub color: DebugLineColor,
    pub thickness: f32,
    pub depth: f32,
}

impl Default for DebugLine {
    fn default() -> Self {
        Self {
            from: Vec2::ZERO,
            to: Vec2::ZERO,
            color: DebugLineColor::Solid(Color::BLACK),
            thickness: 1.,
            depth: 0.,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebugLineColor {
    Solid(Color),
    Gradient(Color, Color),
}

impl Default for DebugLineColor {
    fn default() -> Self {
        Self::Solid(Color::BLACK)
    }
}

impl From<Color> for DebugLineColor {
    fn from(value: Color) -> Self {
        DebugLineColor::Solid(value)
    }
}

impl DebugDrawDrawable for DebugLine {
    fn to_mesh(&self) -> DebugDrawMesh {
        if self.from == self.to {
            DebugDrawMesh::new()
        } else {
            let orthogonal = (self.from - self.to).normalize().perp() * self.thickness * 0.5;
            let (from_color, to_color) = match self.color {
                DebugLineColor::Solid(color) => (color, color),
                DebugLineColor::Gradient(from_color, to_color) => (from_color, to_color),
            };
            DebugDrawMesh {
                vertices: vec![
                    DebugDrawVertex {
                        position: self.from - orthogonal,
                        color: from_color,
                    },
                    DebugDrawVertex {
                        position: self.from + orthogonal,
                        color: from_color,
                    },
                    DebugDrawVertex {
                        position: self.to - orthogonal,
                        color: to_color,
                    },
                    DebugDrawVertex {
                        position: self.to + orthogonal,
                        color: to_color,
                    },
                ],
                indices: vec![0, 1, 2, 3, 2, 1],
                depth: self.depth,
            }
        }
    }
}
