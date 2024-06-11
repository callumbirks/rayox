use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Vec3<T>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Copy,
{
    pub fn new_uniform(a: T) -> Self {
        Vec3 { x: a, y: a, z: a }
    }
}

impl<T> Vec3<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    pub fn dot_product(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn sqr_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Vec3<f32> {
    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalized(self) -> Self {
        let sqr_normal = self.sqr_magnitude();
        if sqr_normal > 0.0 {
            let inv_normal = 1.0 / sqr_normal.sqrt();
            Self {
                x: self.x * inv_normal,
                y: self.y * inv_normal,
                z: self.z * inv_normal,
            }
        } else {
            self
        }
    }
}

impl Vec3<f64> {
    pub fn magnitude(&self) -> f64 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalized(self) -> Self {
        let sqr_normal = self.sqr_magnitude();
        if sqr_normal > 0.0 {
            let inv_normal = 1.0 / sqr_normal.sqrt();
            Self {
                x: self.x * inv_normal,
                y: self.y * inv_normal,
                z: self.z * inv_normal,
            }
        } else {
            self
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul for Vec3<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div for Vec3<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Vec3<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: Copy + Add<Output = T>,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: Copy + Sub<Output = T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign for Vec3<T>
where
    T: Copy + Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T> DivAssign for Vec3<T>
where
    T: Copy + Div<Output = T>,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T> Neg for Vec3<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Default + Copy> Default for Vec3<T> {
    fn default() -> Self {
        Vec3 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}
