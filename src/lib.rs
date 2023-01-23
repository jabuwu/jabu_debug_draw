use std::mem::take;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};

pub struct DebugDrawPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct DebugDrawStage;

impl Plugin for DebugDrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(CoreStage::Update, DebugDrawStage, SystemStage::parallel())
            .init_resource::<DebugDraw>()
            .add_system_to_stage(DebugDrawStage, debug_renderer);
    }
}

#[derive(Resource, Default)]
pub struct DebugDraw {
    meshes: Vec<DebugDrawMesh>,
}

impl DebugDraw {
    pub fn draw<T: DebugDrawDrawable>(&mut self, mesh: T) {
        self.meshes.push(mesh.to_mesh());
    }
}

pub trait DebugDrawDrawable {
    fn to_mesh(&self) -> DebugDrawMesh;
}

#[derive(Default, Debug, Clone)]
pub struct DebugDrawMesh {
    pub vertices: Vec<DebugDrawVertex>,
    pub indices: Vec<u32>,
}

impl DebugDrawMesh {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DebugDrawVertex {
    pub position: Vec2,
    pub color: Color,
}

#[derive(Component)]
struct DebugDrawObject;

fn debug_renderer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut debug_render: ResMut<DebugDraw>,
    debug_query: Query<Entity, With<DebugDrawObject>>,
) {
    for debug_entity in debug_query.iter() {
        commands.entity(debug_entity).despawn();
    }
    for debug_render_mesh in take(&mut debug_render.meshes).into_iter() {
        let DebugDrawMesh { vertices, indices } = debug_render_mesh;

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut positions: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut uvs: Vec<[f32; 2]> = vec![];
        let mut colors: Vec<[f32; 4]> = vec![];

        for vertex in vertices.iter() {
            positions.push([vertex.position.x, vertex.position.y, 1.]);
            normals.push([0., 0., 0.]);
            uvs.push([0., 0.]);
            colors.push([
                vertex.color.r(),
                vertex.color.g(),
                vertex.color.b(),
                vertex.color.a(),
            ]);
        }

        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

        commands
            .spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(mesh)),
                material: materials.add(ColorMaterial {
                    color: Color::WHITE,
                    texture: None,
                }),
                ..Default::default()
            })
            .insert(DebugDrawObject);
    }
}

mod circle;
mod line;
mod rectangle;

pub use circle::*;
pub use line::*;
pub use rectangle::*;

pub mod prelude;
