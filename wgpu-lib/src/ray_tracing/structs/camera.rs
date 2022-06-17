use bytemuck::{Pod, Zeroable};

use crate::structs::dimensions::Dimensions;

use super::vec3::Vec3;

#[derive(Clone, Copy, Pod, Zeroable, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Camera {
    x: f32,
    y: f32,
    z: f32,
    dir_x: f32,
    dir_y: f32,
    dir_z: f32,
    aspect_ratio: f32,
    vertical_fov_radians: f32,
}

impl Camera {
    pub fn new(pos: Vec3, dir: Vec3, dimensions: &Dimensions, vertical_fov_degrees: f32) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            z: pos.z,
            dir_x: dir.x,
            dir_y: dir.y,
            dir_z: dir.z,
            aspect_ratio: (dimensions.width as f32) / (dimensions.height as f32),
            vertical_fov_radians: vertical_fov_degrees * std::f32::consts::PI / 180.0,
        }
    }
}
