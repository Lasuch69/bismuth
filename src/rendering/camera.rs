use std::f32::consts::PI;

use bevy_ecs::prelude::*;
use bevy_math::prelude::*;

#[derive(Resource)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn get_view_projection_matrix(&self, aspect: f32) -> Mat4 {
        let fovy = self.fovy * (PI / 180.0);

        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(fovy, aspect, self.znear, self.zfar);

        return proj * view;
    }
}
