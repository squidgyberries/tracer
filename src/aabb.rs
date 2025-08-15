use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::{interval::Interval, ray::Ray};

use glam::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub const EMPTY: Aabb = Aabb::new(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY);

    pub const EVERYTHING: Aabb = Aabb::new(
        Interval::EVERYTHING,
        Interval::EVERYTHING,
        Interval::EVERYTHING,
    );

    #[inline(always)]
    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut out = Self { x, y, z };
        out.pad_to_mins();
        out
    }

    #[inline(always)]
    pub fn from_corners(a: Vec3, b: Vec3) -> Self {
        let min = a.min(b);
        let max = a.max(b);

        Self::new(
            Interval::new(min.x, max.x),
            Interval::new(min.y, max.y),
            Interval::new(min.z, max.z),
        )
        .padded_to_mins()
    }

    #[inline(always)]
    pub const fn merged(a: Self, b: Self) -> Self {
        Self::new(
            Interval::enclosing(a.x, b.x),
            Interval::enclosing(a.y, b.y),
            Interval::enclosing(a.z, b.z),
        )
    }

    #[inline(always)]
    pub const fn merge(&mut self, other: Self) {
        self.x = Interval::enclosing(self.x, other.x);
        self.y = Interval::enclosing(self.y, other.y);
        self.z = Interval::enclosing(self.z, other.z);
    }

    #[inline(always)]
    pub const fn pad_to_mins(&mut self) {
        const DELTA: f32 = 0.001;
        if self.x.size() < DELTA {
            self.x.expand(DELTA);
        }
        if self.y.size() < DELTA {
            self.y.expand(DELTA);
        }
        if self.z.size() < DELTA {
            self.z.expand(DELTA);
        }
    }

    #[inline(always)]
    pub const fn padded_to_mins(&self) -> Self {
        const DELTA: f32 = 0.001;
        Self::new(
            if self.x.size() < DELTA {
                self.x.expanded(DELTA)
            } else {
                self.x
            },
            if self.y.size() < DELTA {
                self.y.expanded(DELTA)
            } else {
                self.y
            },
            if self.z.size() < DELTA {
                self.z.expanded(DELTA)
            } else {
                self.z
            },
        )
    }

    pub fn hit(&self, ray: Ray, mut ray_t: Interval) -> bool {
        for axis in 0..=2 {
            let axis_interval = self[axis];
            let adinv = 1.0 / ray.direction[axis];

            let t0 = (axis_interval.min - ray.origin[axis]) * adinv;
            let t1 = (axis_interval.max - ray.origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    #[inline(always)]
    pub fn longest_axis(&self) -> usize {
        return if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        };
    }
}

impl Index<usize> for Aabb {
    type Output = Interval;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Aabb {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl Add<Vec3> for Aabb {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Vec3> for Aabb {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3> for Aabb {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<Vec3> for Aabb {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3> for Aabb {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<Vec3> for Aabb {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

impl Div<Vec3> for Aabb {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl DivAssign<Vec3> for Aabb {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Vec3) {
        *self = *self / rhs;
    }
}
