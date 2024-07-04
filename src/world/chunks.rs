use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::VertexFormat;

pub const CHUNK_SIZE: f32 = 32.;

#[derive(Component, Clone, Default, ExtractComponent)]
/// A marker component for voxel meshes.
pub struct VoxelTerrainMesh;

impl VoxelTerrainMesh {
    pub const ATTRIBUTE_DATA: MeshVertexAttribute = MeshVertexAttribute::new("Vertex_Data", 0x127497, VertexFormat::Uint32);
}