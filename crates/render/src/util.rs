use wgpu::Color;

pub fn color(r: f32, g: f32, b: f32) -> Color {
    Color {
        r: r as f64,
        g: g as f64,
        b: b as f64,
        a: 1.0,
    }
}
