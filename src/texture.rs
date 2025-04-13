use std::sync::Arc;

use crate::interval::Interval;

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

pub struct SpatialChecker {
    scale_inv: f32,
    even: Arc<dyn Texture + Send + Sync>,
    odd: Arc<dyn Texture + Send + Sync>,
}

impl SpatialChecker {
    #[inline(always)]
    pub fn new(
        scale: f32,
        even: Arc<dyn Texture + Send + Sync>,
        odd: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            scale_inv: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl Texture for SpatialChecker {
    fn value(&self, uv: Vec2, point: Vec3) -> Vec3 {
        let x_int = (point.x * self.scale_inv).floor() as i32;
        let y_int = (point.y * self.scale_inv).floor() as i32;
        let z_int = (point.z * self.scale_inv).floor() as i32;

        return if (x_int + y_int + z_int) % 2 == 0 {
            self.even.value(uv, point)
        } else {
            self.odd.value(uv, point)
        };
    }
}

pub struct ImageTexture {
    image: image::Rgb32FImage,
}

impl ImageTexture {
    #[inline(always)]
    pub fn new(image: image::Rgb32FImage) -> Self {
        Self { image }
    }

    #[inline(always)]
    pub fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        Ok(Self::new(
            image::ImageReader::open(path)?
                .with_guessed_format()?
                .decode()?
                .into_rgb32f(),
        ))
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut uv: Vec2, _point: Vec3) -> Vec3 {
        uv.x = Interval::new(0.0, 0.999).clamp(uv.x);
        uv.y = 1.0 - Interval::new(0.001, 1.0).clamp(uv.y);

        let x = (uv.x * self.image.width() as f32) as u32;
        let y = (uv.y * self.image.height() as f32) as u32;

        let pixel = self.image.get_pixel(x, y);
        Vec3::new(pixel[0], pixel[1], pixel[2])
    }
}
