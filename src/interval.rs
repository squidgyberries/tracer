#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    #[inline(always)]
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    #[inline(always)]
    pub const fn size(&self) -> f32 {
        self.max - self.min
    }

    #[inline(always)]
    pub const fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    #[inline(always)]
    pub const fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    #[inline(always)]
    pub const fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

impl Default for Interval {
    #[inline(always)]
    fn default() -> Self {
        Self {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }
}
