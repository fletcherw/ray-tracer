use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct V3 {
    x: f64,
    y: f64,
    z: f64,
}

impl V3 {
    pub fn create(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &V3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}
impl Add<&V3> for V3 {
    type Output = V3;

    fn add(self, rhs: &V3) -> V3 {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        V3::create(x, y, z)
    }
}

impl Add<V3> for V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> V3 {
        self + &rhs
    }
}

impl Sub<&V3> for &V3 {
    type Output = V3;

    fn sub(self, rhs: &V3) -> V3 {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        V3::create(x, y, z)
    }
}

impl Sub<Self> for V3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        &self - &rhs
    }
}
impl Mul<&V3> for &V3 {
    type Output = V3;

    fn mul(self, rhs: &V3) -> V3 {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        V3::create(x, y, z)
    }
}

impl Mul<&V3> for V3 {
    type Output = V3;

    fn mul(self, rhs: &V3) -> V3 {
        &self * rhs
    }
}

impl Mul<V3> for &V3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 {
        self * &rhs
    }
}

impl Mul<V3> for V3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 {
        &self * &rhs
    }
}

impl Mul<f64> for &V3 {
    type Output = V3;

    fn mul(self, rhs: f64) -> V3 {
        V3::create(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f64> for V3 {
    type Output = V3;

    fn mul(self, rhs: f64) -> V3 {
        &self * rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::V3;

    #[test]
    fn cross_unit_vectors() {
        let i = V3::create(1.0, 0.0, 0.0);
        let j = V3::create(0.0, 1.0, 0.0);
        let k = V3::create(0.0, 0.0, 1.0);

        assert_eq!(i * j, k);
    }

    #[test]
    fn scale_vectors() {
        let v = V3::create(1.0, 2.0, -3.0);

        assert_eq!(v * 3.0, V3::create(3.0, 6.0, -9.0));
    }

    #[test]
    fn dot_unit_vectors() {
        let i = V3::create(1.0, 0.0, 0.0);
        let j = V3::create(0.0, 1.0, 0.0);
        let k = V3::create(0.0, 0.0, 1.0);

        assert_eq!(i.dot(&j), 0.0);
        assert_eq!(i.dot(&k), 0.0);
        assert_eq!(j.dot(&k), 0.0);
    }

    #[test]
    fn dot_arbitrary_vectors() {
        let v1 = V3::create(3.0, 1.0, 7.0);
        let v2 = V3::create(-2.5, 1.1, 0.0);

        assert_eq!(v1.dot(&v2), -6.4);
    }
}
