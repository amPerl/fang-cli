use binrw::{count, BinRead, FilePtr};

pub mod color;
use self::color::*;

pub mod geometry;
use self::geometry::*;

pub mod mesh;
use self::mesh::*;

#[derive(BinRead, Debug)]
pub struct Ape {
    #[br(count = 16, map = crate::util::vec_to_null_terminated_str)]
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

    #[br(parse_with = FilePtr::with(count(light_count as usize)))]
    pub lights: FilePtr<u32, Vec<FLightInit>>,

    pub skeleton_indices: u32,

    #[br(parse_with = FilePtr::with(count(material_count as usize)))]
    pub materials: FilePtr<u32, Vec<FMeshMaterial>>,

    pub collision_tree: u32,
    pub tex_layer_ids: u32,
    pub platform_specific_mesh: u32,
}

#[derive(BinRead, Debug)]
pub struct FLightInit {
    #[br(count = 16, map = crate::util::vec_to_null_terminated_str)]
    pub name: String,

    #[br(count = 16, map = crate::util::vec_to_null_terminated_str)]
    pub per_pixel_texture_name: String,
    #[br(count = 16, map = crate::util::vec_to_null_terminated_str)]
    pub corona_texture_name: String,

    pub flags: u32,

    pub light_id: u16,
    pub kind: u8,
    pub parent_bone_idx: i8,

    pub intensity: f32,
    pub motif: CFColorMotif,
    pub influence: CFSphere,
    pub orientation: CFMtx43,
    pub spot_inner_radians: f32,
    pub spot_outer_radians: f32,
    pub corona_scale: f32,
}
