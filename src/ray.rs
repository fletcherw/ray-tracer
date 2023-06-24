use crate::v3::V3;

pub struct Ray {
    pub source: V3,
    pub direction: V3,
}

impl Ray {
    pub fn create(source: V3, direction: V3) -> Self {
        Self { source, direction }
    }
}
