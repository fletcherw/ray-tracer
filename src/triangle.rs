use crate::v3::V3;

pub struct Triangle {
    p1: V3,
    p2: V3,
    p3: V3,
}

impl Triangle {
    pub fn create(p1: V3, p2: V3, p3: V3) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn normal(&self) -> V3 {
        let v1 = self.p2 - self.p1;
        let v2 = self.p3 - self.p1;
        v1 * v2
    }

    pub fn k(&self) -> f64 {
        -self.normal().dot(&self.p1)
    }
}

#[cfg(test)]
mod tests {
    use crate::triangle::Triangle;
    use crate::v3::V3;

    #[test]
    fn test_normal() {
        let i = V3::create(-1.0, 0.0, 2.0);
        let j = V3::create(1.0, 0.0, 2.0);
        let k = V3::create(0.0, -1.0, 3.0);

        let t = Triangle::create(i, j, k);
        assert_eq!(t.normal(), V3::create(0.0, -2.0, -2.0));
    }

    #[test]
    fn test_k() {
        let i = V3::create(-1.0, 0.0, 2.0);
        let j = V3::create(1.0, 0.0, 2.0);
        let k = V3::create(0.0, -1.0, 3.0);

        let t = Triangle::create(i, j, k);
        assert_eq!(t.k(), 4.0);
    }
}
