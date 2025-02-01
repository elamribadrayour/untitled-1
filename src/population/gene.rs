#[derive(Clone)]
pub struct Gene {
    pub id: usize,
    pub color: usize,
}

impl Gene {
    pub fn new(id: usize, color: usize) -> Self {
        Self { id, color }
    }
}
