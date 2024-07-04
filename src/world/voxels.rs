use crate::world::materials::VoxelMaterialID;
use block_mesh::{MergeVoxel, Voxel as MeshableVoxel, VoxelVisibility};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Voxel(VoxelMaterialID);

impl Voxel {
    pub const SIZE: f32 = 0.1;
    pub const EMPTY: Self = Self(0);

    pub fn new(material_id: VoxelMaterialID) -> Self {
        Self(material_id)
    }

    pub fn get_material_id(&self) -> VoxelMaterialID {
        self.0
    }
}

impl MeshableVoxel for Voxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.0 == 0 { VoxelVisibility::Empty } else { VoxelVisibility::Opaque }
    }
}

impl MergeVoxel for Voxel {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}
