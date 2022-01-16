use super::{fang_types::*, util::*};
use binrw::{count, BinRead, FilePtr};

#[derive(BinRead, Debug)]
pub struct Ape {
    #[br(count = 16, map = vec_to_null_terminated_str)]
    pub name: String,

    pub bound_sphere: CFSphere,
    pub bound_box_min: CFVec3,
    pub bound_box_max: CFVec3,

    pub flags: u16,
    pub mesh_coll_mask: u16,

    pub used_bone_count: u8,
    pub root_bone_index: u8,
    pub bone_count: u8,
    pub segment_count: u8,
    pub tex_layer_id_count: u8,
    pub tex_layer_id_count_st: u8,
    pub tex_layer_id_count_flip: u8,
    pub light_count: u8,
    pub material_count: u8,
    pub coll_tree_count: u8,
    pub lod_count: u8,
    pub shadow_lod_bias: u8,

    pub load_distances: [f32; 8],

    #[br(parse_with = FilePtr::with(count(segment_count as usize)))]
    pub segments: FilePtr<u32, Vec<FMeshSegment>>,

    #[br(parse_with = FilePtr::with(count(bone_count as usize)))]
    pub bones: FilePtr<u32, Vec<FMeshBone>>,

    pub lights: u32,
    pub skeleton_indices: u32,

    #[br(parse_with = FilePtr::with(count(material_count as usize)))]
    pub materials: FilePtr<u32, Vec<FMeshMaterial>>,

    pub collision_tree: u32,
    pub tex_layer_ids: u32,
    pub platform_specific_mesh: u32,
}

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
    #[br(count = 32, map = vec_to_null_terminated_str)]
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
