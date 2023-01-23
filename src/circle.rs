use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{DebugDrawDrawable, DebugDrawMesh, DebugDrawVertex};

#[derive(Clone, Copy, Debug)]
pub struct DebugDrawCircle {
    pub position: Vec2,
    pub radius: f32,
    pub segments: u8,
    pub rotation: f32,
    pub color: Color,
}

impl Default for DebugDrawCircle {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            radius: 0.,
            segments: 64,
            rotation: 0.,
            color: Color::BLACK,
        }
    }
}

impl DebugDrawDrawable for DebugDrawCircle {
    fn to_mesh(&self) -> DebugDrawMesh {
        let mut vertices = vec![];
        let mut indices = vec![];
        for segment in 0..self.segments {
            let angle = self.rotation + segment as f32 / self.segments as f32 * TAU;
            vertices.push(DebugDrawVertex {
                position: self.position + Vec2::from_angle(angle) * self.radius * 0.5,
                color: self.color,
            });
            indices.push(self.segments as u32);
            indices.push(segment as u32);
            indices.push((segment as u32 + 1) % self.segments as u32);
        }
        vertices.push(DebugDrawVertex {
            position: self.position,
            color: self.color,
        });
        DebugDrawMesh { vertices, indices }
    }
}
