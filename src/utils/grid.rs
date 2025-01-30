pub struct Grid {
    pub dim: usize,
    pub positions: Vec<[usize; 2]>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let dim = (size as f64).sqrt().ceil() as usize;
        Self {
            dim,
            positions: (0..size).map(|i| [i % dim, i / dim]).collect(),
        }
    }

    pub fn get(&self, id: usize) -> [usize; 2] {
        self.positions[id]
    }
}
