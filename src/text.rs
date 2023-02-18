use std::{collections::HashMap, mem::take, sync::RwLock};

use bevy::prelude::*;
use lazy_static::lazy_static;
use ttf2mesh_triangulation::Triangulator;
use ttf_parser::{Face, GlyphId};

use crate::{DebugDrawDrawable, DebugDrawMesh, DebugDrawVertex};

lazy_static! {
    static ref TTF_FACE: Face<'static> =
        Face::parse(include_bytes!("./FiraSans-Bold.ttf"), 0).unwrap();
    static ref GLYPH_CACHE: RwLock<HashMap<GlyphId, Glyph>> = RwLock::new(HashMap::new());
}
const BASE_SCALE: f32 = 0.02;
const LINE_HEIGHT: f32 = 1200.;
const RESOLUTION: usize = 3;
const INVERSE_RESOLUTION: f32 = 1. / (RESOLUTION as f32);

#[derive(Clone)]
struct Glyph {
    triangles: Vec<[Vec2; 3]>,
    horizontal_advance: u16,
}

#[derive(Clone, Debug)]
pub struct DebugText {
    pub text: String,
    pub position: Vec2,
    pub scale: f32,
    pub color: Color,
    pub alignment: DebugTextAlignment,
    pub vertical_alignment: DebugTextVerticalAlignment,
    pub depth: f32,
}

impl Default for DebugText {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
            position: Vec2::ZERO,
            scale: 1.,
            color: Color::BLACK,
            alignment: DebugTextAlignment::Left,
            vertical_alignment: DebugTextVerticalAlignment::Top,
            depth: 0.,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DebugTextAlignment {
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DebugTextVerticalAlignment {
    Top,
    Center,
    Bottom,
}

enum Character {
    Glyph(Glyph),
    Newline,
}

impl DebugDrawDrawable for DebugText {
    fn to_mesh(&self) -> DebugDrawMesh {
        let mut characters = vec![];
        for char in self.text.chars() {
            if char == '\n' {
                characters.push(Character::Newline);
            } else if let Some(glyph_id) = TTF_FACE.glyph_index(char) {
                characters.push(Character::Glyph(
                    if let Some(glyph) = {
                        let glyph_cache = GLYPH_CACHE.read().expect("failed to lock mesh cache");
                        glyph_cache.get(&glyph_id).cloned()
                    } {
                        glyph
                    } else {
                        let mut triangulator_builder = TriangulatorBuilder::default();
                        TTF_FACE.outline_glyph(glyph_id, &mut triangulator_builder);
                        let triangles = if triangulator_builder.has_contours {
                            if let Ok(triangles) = triangulator_builder.triangulator.triangulate() {
                                triangles
                            } else {
                                vec![]
                            }
                        } else {
                            vec![]
                        };
                        let glyph = Glyph {
                            triangles,
                            horizontal_advance: TTF_FACE.glyph_hor_advance(glyph_id).unwrap_or(0),
                        };
                        {
                            let mut glyph_cache =
                                GLYPH_CACHE.write().expect("failed to lock mesh cache");
                            glyph_cache.insert(glyph_id, glyph.clone());
                        }
                        glyph
                    },
                ));
            }
        }

        let scale = self.scale * BASE_SCALE;
        let mut line_widths = vec![];
        let mut current_width = 0.;
        let mut lines = 0;
        for character in characters.iter() {
            match character {
                Character::Glyph(glyph) => {
                    current_width += glyph.horizontal_advance as f32 * scale;
                }
                Character::Newline => {
                    line_widths.push(current_width);
                    current_width = 0.;
                    lines += 1;
                }
            }
        }
        line_widths.push(current_width);

        let mut vertices = vec![];
        let mut indices = vec![];
        let mut line = 0;
        let mut position = self.position
            + Vec2::new(
                match self.alignment {
                    DebugTextAlignment::Left => 0.,
                    DebugTextAlignment::Center => line_widths[line] * -0.5,
                    DebugTextAlignment::Right => line_widths[line] * -1.,
                },
                match self.vertical_alignment {
                    DebugTextVerticalAlignment::Top => 0.,
                    DebugTextVerticalAlignment::Center => {
                        (lines as f32 * LINE_HEIGHT * scale) * 0.5
                    }
                    DebugTextVerticalAlignment::Bottom => lines as f32 * LINE_HEIGHT * scale,
                },
            );
        for character in characters.iter() {
            match character {
                Character::Glyph(glyph) => {
                    vertices.reserve(glyph.triangles.len() * 3);
                    indices.reserve(glyph.triangles.len() * 3);
                    for triangle in glyph.triangles.iter() {
                        indices.push(vertices.len() as u32);
                        vertices.push(DebugDrawVertex {
                            position: position + triangle[0] * scale,
                            color: self.color,
                        });
                        indices.push(vertices.len() as u32);
                        vertices.push(DebugDrawVertex {
                            position: position + triangle[1] * scale,
                            color: self.color,
                        });
                        indices.push(vertices.len() as u32);
                        vertices.push(DebugDrawVertex {
                            position: position + triangle[2] * scale,
                            color: self.color,
                        });
                    }
                    position.x += glyph.horizontal_advance as f32 * scale;
                }
                Character::Newline => {
                    line += 1;
                    //position.x = self.position.x + line_widths[line] * -0.5;
                    position.x = match self.alignment {
                        DebugTextAlignment::Left => self.position.x,
                        DebugTextAlignment::Center => self.position.x - line_widths[line] * 0.5,
                        DebugTextAlignment::Right => self.position.x - line_widths[line],
                    };
                    position.y -= LINE_HEIGHT * scale;
                }
            }
        }
        DebugDrawMesh {
            vertices,
            indices,
            depth: self.depth,
        }
    }
}

#[derive(Default)]
struct TriangulatorBuilder {
    contour: Vec<Vec2>,
    triangulator: Triangulator,
    has_contours: bool,
}

impl ttf_parser::OutlineBuilder for TriangulatorBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        if self.contour.len() > 0 {
            self.triangulator.add_contour(0, take(&mut self.contour));
            self.contour = Vec::new();
            self.has_contours = true;
        }
        self.contour.push(Vec2::new(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.contour.push(Vec2::new(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        if let Some(last) = self.contour.last() {
            let mut p0 = *last;
            let mut p1 = Vec2::new(x1, y1);
            let p2 = Vec2::new(x, y);
            let d01 = (p1 - p0) * INVERSE_RESOLUTION;
            let d12 = (p2 - p1) * INVERSE_RESOLUTION;
            for i in 0..(RESOLUTION - 1) {
                p0 += d01;
                p1 += d12;
                let cp = p0 + (p1 - p0) * (i as f32) * INVERSE_RESOLUTION;
                self.contour.push(cp);
            }
            self.contour.push(Vec2::new(x, y));
        } else {
            self.contour.push(Vec2::new(x, y));
        }
    }

    fn curve_to(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, x: f32, y: f32) {
        // TODO
        self.contour.push(Vec2::new(x, y));
    }

    fn close(&mut self) {
        if self.contour.len() > 0 {
            self.triangulator.add_contour(0, take(&mut self.contour));
            self.contour = Vec::new();
            self.has_contours = true;
        }
    }
}
