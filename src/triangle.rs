use crate::ray::Ray;
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

    pub fn intersect(&self, ray: &Ray) -> Option<V3> {
        let lambda = -1.0 * (self.normal().dot(&ray.source) + self.k())
            / (self.normal().dot(&ray.direction));

        // The ray does not intersect the plane defined by the triangle if the plane is behind the source.
        if lambda < 0.0 {
            return None;
        }

        let intersection = ray.source + ray.direction * lambda;

        if !same_side(&self.p1, &self.p2, &self.p3, &intersection) {
            return None;
        }

        if !same_side(&self.p1, &self.p3, &self.p2, &intersection) {
            return None;
        }

        if !same_side(&self.p2, &self.p3, &self.p1, &intersection) {
            return None;
        }

        Some(intersection)
    }
}

fn same_side(divider_start: &V3, divider_end: &V3, point_a: &V3, point_b: &V3) -> bool {
    let divider = divider_start - divider_end;
    let a = divider * (point_a - divider_end);
    let b = divider * (point_b - divider_end);
    a.dot(&b) >= 0.0
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
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

    #[test]
    fn test_intersect() {
        let i = V3::create(-1.0, 0.0, 2.0);
        let j = V3::create(1.0, 0.0, 2.0);
        let k = V3::create(0.0, -1.0, 3.0);

        let t = Triangle::create(i, j, k);

        let origin = V3::create(0.0, 0.0, 0.0);

        assert_eq!(t.intersect(&Ray::create(origin, i)), Some(i));
        assert_eq!(t.intersect(&Ray::create(origin, j)), Some(j));
        assert_eq!(t.intersect(&Ray::create(origin, k)), Some(k));

        assert_eq!(
            t.intersect(&Ray::create(
                V3::create(0.0, -0.5, 0.0),
                V3::create(0.0, 0.0, 1.0)
            )),
            Some(V3::create(0.0, -0.5, 2.5))
        );
    }
}
