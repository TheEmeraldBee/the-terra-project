use glam::Mat4;
use winit::dpi::PhysicalSize;

pub struct Projection {
    pub fovy: f32,

    pub aspect: f32,

    pub near: f32,
    pub far: f32,

    pub proj: Mat4,
}

impl Default for Projection {
    fn default() -> Self {
        Self {
            fovy: 90.0,

            aspect: 0.0,

            near: 0.01,
            far: 5000.0,

            proj: Mat4::IDENTITY,
        }
    }
}

impl Projection {
    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.aspect = size.width as f32 / size.height as f32;

        self.proj = Mat4::perspective_rh(self.fovy, self.aspect, self.near, self.far);
    }
}
