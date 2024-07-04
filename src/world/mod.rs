use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG, UnitQuadBuffer, UnorientedQuad, UnorientedUnitQuad, visible_block_faces};
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use crate::{constants, GameStatus};
use crate::world::chunks::{CHUNK_SIZE, VoxelTerrainMesh};
use crate::world::voxels::Voxel;

pub mod chunks;
pub mod voxels;
mod materials;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStatus::InGame), create_dummy_voxel_circle);
    }
}

type ChunkShape = ConstShape3u32<34, 34, 34>;

fn into_domain(array_dim: u32, [x, y, z]: [u32; 3]) -> Vec3A {
    (2.0 / array_dim as f32) * Vec3A::new(x as f32, y as f32, z as f32) - 1.0
}

fn sphere(radius: f32, p: Vec3A) -> Voxel {
    let m = p.length() < radius;
    Voxel::new(if m { 1 } else { 0 })
}

fn create_dummy_voxel_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let mut render_mesh = Mesh::new(constants::graphic_settings::PRIMITIVE_TOPOLOGY, RenderAssetUsages::RENDER_WORLD);

    let mut voxels = [Voxel::EMPTY; ChunkShape::SIZE as usize];
    for i in 0..ChunkShape::SIZE {
        let p = into_domain(32, ChunkShape::delinearize(i));
        voxels[i as usize] = sphere(0.5, p);
    }

    let mut mesh_buffer = UnitQuadBuffer::new();
    visible_block_faces(
        &voxels,
        &ChunkShape {},
        [0; 3],
        [33; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut mesh_buffer
    );

    let num_indices = mesh_buffer.num_quads() * 6;
    let num_vertices = mesh_buffer.num_quads() * 4;

    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);

    for (group, face) in mesh_buffer.groups.into_iter().zip(RIGHT_HANDED_Y_UP_CONFIG.faces.into_iter()) {
        for quad in group.iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(&(*quad).into(), 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());
        }
    }

    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions),
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices])
    );
    render_mesh.insert_indices(Indices::U32(indices));

    dbg!((CHUNK_SIZE * Voxel::SIZE) / 2.,);

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(render_mesh),
            transform: Transform::from_xyz(
                (CHUNK_SIZE * Voxel::SIZE) / 2.,
                (CHUNK_SIZE * Voxel::SIZE) / 2.,
                (CHUNK_SIZE * Voxel::SIZE) / 2.
            ),
            ..default()
        }
    );
}