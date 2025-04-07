#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f32::INFINITY, f32::NEG_INFINITY);
    
    pub const UNIVERSE: Self = Self::new(f32::NEG_INFINITY, f32::INFINITY);

    #[inline(always)]
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    #[inline(always)]
    pub const fn enclosing(a: Self, b: Self) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
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

    #[inline(always)]
    pub const fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }
}
