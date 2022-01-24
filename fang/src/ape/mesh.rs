use binrw::BinRead;

use super::{
    color::CFColorRGB,
    geometry::{CFMtx43A, CFSphere, CFVec3},
};

#[derive(BinRead, Debug)]
pub struct FMeshSegment {
    pub bound_sphere: CFSphere,
    pub bone_mtx_count: u8,
    #[br(pad_after(3))]
    pub bone_mtx_indices: [u8; 4],
}

#[derive(BinRead, Debug)]
pub struct FMeshSkeleton {
    pub parent_bone_index: u8,
    pub child_bone_count: u8,
    pub child_array_start_index: u8,
}

#[derive(BinRead, Debug)]
pub struct FMeshBone {
    #[br(count = 32, map = crate::util::vec_to_null_terminated_str)]
    pub name: String,
    pub at_rest_bone_to_model_mtx: CFMtx43A,
    pub at_rest_model_to_bone_mtx: CFMtx43A,
    pub at_rest_parent_to_bone_mtx: CFMtx43A,
    pub at_rest_bone_to_parent_mtx: CFMtx43A,
    pub segmented_bound_sphere: CFSphere,
    pub skeleton: FMeshSkeleton,
    pub flags: u8,
    #[br(pad_after(11))]
    pub part_id: u8,
}

#[derive(BinRead, Debug)]
pub struct FMeshMaterial {
    pub off_sh_light_registers: u32,
    pub off_sh_surface_registers: u32,
    pub light_shader_idx: u8,
    pub specular_shader_idx: u8,
    pub surface_shader_idx: u16,

    pub part_id_mask: u32,

    pub off_platform_data: u32,

    pub lod_mask: u8,
    pub depth_bias_level: u8,
    pub base_st_sets: u8,
    pub light_map_st_sets: u8,
    pub tex_layer_id_indices: [u8; 4],

    pub affect_angle: f32,
    pub comp_affect_normals: [i8; 3],
    pub affect_bone_id: i8,

    #[br(pad_after(1))]
    pub compressed_radius: u8,

    pub mtl_flags: u16,

    pub draw_key: u32,

    pub material_tint: CFColorRGB,
    pub average_vert_pos: CFVec3,
    pub dl_hash_key: u32,
}
