use glam::*;
use winit::dpi::PhysicalSize;

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,

    pub fovy: f32,

    pub aspect: f32,

    pub near: f32,
    pub far: f32,

    pub uniform: CameraUniform,
}

impl Default for Camera {
    fn default() -> Self {
        let mut res = Self {
            pos: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,

            fovy: 90.0,

            aspect: const { 1920.0 / 1080.0 },

            near: 0.01,
            far: 5000.0,

            uniform: CameraUniform::default(),
        };
        res.update_view();
        res
    }
}

impl Camera {
    pub fn update_view(&mut self) {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        let view = Mat4::look_to_rh(
            self.pos,
            Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vec3::Y,
        );

        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.near, self.far);

        self.uniform.update(proj * view);
    }

    pub fn uniform(&self) -> CameraUniform {
        self.uniform
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.aspect = size.width as f32 / size.height as f32;
    }

    pub fn forward(&self) -> Vec3 {
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        Vec3::new(yaw_cos, 0.0, yaw_sin).normalize()
    }

    pub fn right(&self) -> Vec3 {
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize()
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}

impl CameraUniform {
    pub fn update(&mut self, proj: Mat4) {
        self.view_proj = proj.to_cols_array_2d()
    }
}
