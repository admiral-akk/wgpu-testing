use bytemuck::{Pod, Zeroable};

use super::vec3::Vec3;

#[derive(Clone, Copy, Pod, Zeroable, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Sphere {
    pos: Vec3,
    r: f32,
}

impl Sphere {
    pub fn new(pos: Vec3, r: f32) -> Self {
        Self { pos, r }
    }
}
