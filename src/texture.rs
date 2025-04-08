use glam::{Vec2, Vec3};

pub trait Texture {
    fn value(&self, uv: Vec2, point: Vec3) -> Vec3;
}

pub struct SolidColor {
    pub albedo: Vec3,
}

impl SolidColor {
    #[inline(always)]
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    #[inline(always)]
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(Vec3::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: Vec2, _point: Vec3) -> Vec3 {
        self.albedo
    }
}
