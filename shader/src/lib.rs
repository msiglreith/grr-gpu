#![cfg_attr(target_arch = "spirv", no_std)]
#![feature(lang_items)]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std::glam::{Vec2, Vec4};
use spirv_std::storage_class::{Input, Output, UniformConstant};
use spirv_std::num_traits::Float;

// Based upon https://www.shadertoy.com/view/XlSGzz
fn sdf_torus(p: Vec2) -> f32 {
    1.0 / (8.0 * (2.0 * p.length() - 1.0).abs())
}

#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_pos: Input<Vec2>,
    mut output: Output<Vec4>,
    viewport: UniformConstant<Vec2>,
) {
    let viewport = viewport.load();
    let pos = frag_pos.load();
    let p = (2.0 * pos - viewport) / viewport.y;
    let coverage = sdf_torus(p).powf(2.2);

    output.store(Vec4::new(coverage, coverage, coverage, 1.0));
}

#[allow(unused_attributes)]
#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_id)] vert_id: Input<i32>,
    #[spirv(position)] mut out_pos: Output<Vec4>,
) {
    let vert_id = vert_id.load();
    out_pos.store(Vec4::new(
        (((vert_id & 1) << 2) - 1) as f32,
        (((vert_id & 2) << 1) - 1) as f32,
        0.0,
        1.0,
    ));
}
