use crate::utils::Assets;

#[derive(Clone)]
pub struct Gene {
    pub id: usize,
    pub color: usize,
}

impl Gene {
    pub fn new(id: usize, assets: &Assets) -> Self {
        Self {
            id,
            color: assets.rand(),
        }
    }
}
