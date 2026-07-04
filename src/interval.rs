use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f32::INFINITY, f32::NEG_INFINITY);

    pub const EVERYTHING: Self = Self::new(f32::NEG_INFINITY, f32::INFINITY);

    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub const fn enclosing(a: Self, b: Self) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
    }

    pub const fn size(&self) -> f32 {
        self.max - self.min
    }

    pub const fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub const fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub const fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub const fn expand(&mut self, delta: f32) {
        let padding = delta * 0.5;
        self.min -= padding;
        self.max += padding;
    }

    pub const fn expanded(&self, delta: f32) -> Self {
        let padding = delta * 0.5;
        Self::new(self.min - padding, self.max + padding)
    }
}

impl Add<f32> for Interval {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.min + rhs, self.max + rhs)
    }
}

impl AddAssign<f32> for Interval {
    fn add_assign(&mut self, rhs: f32) {
        self.min += rhs;
        self.max += rhs;
    }
}

impl Sub<f32> for Interval {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.min - rhs, self.max - rhs)
    }
}

impl SubAssign<f32> for Interval {
    fn sub_assign(&mut self, rhs: f32) {
        self.min -= rhs;
        self.max -= rhs;
    }
}

impl Mul<f32> for Interval {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        if rhs >= 0.0 {
            Self::new(self.min * rhs, self.max * rhs)
        } else {
            Self::new(self.max * rhs, self.min * rhs)
        }
    }
}

impl MulAssign<f32> for Interval {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Interval {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs >= 0.0 {
            Self::new(self.min / rhs, self.max / rhs)
        } else {
            Self::new(self.max / rhs, self.min / rhs)
        }
    }
}

impl DivAssign<f32> for Interval {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
