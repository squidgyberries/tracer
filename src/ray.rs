use glam::Vec3;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    #[inline(always)]
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[inline(always)]
    pub fn at(self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }
}
