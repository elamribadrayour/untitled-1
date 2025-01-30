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

    pub fn neighbors(&self, id: usize, range: usize) -> Vec<usize> {
        let [x, y] = self.get(id);
        let directions = (0..(range as i32)).map(|i| (i, -i)).collect::<Vec<_>>();
        let neighbors = self
            .positions
            .iter()
            .enumerate()
            .filter(|(i, pos)| {
                let (px, py) = (pos[0] as i32, pos[1] as i32);
                *i != id
                    && directions
                        .iter()
                        .any(|&(dx, dy)| px == (x as i32) + dx && py == (y as i32) + dy)
            })
            .map(|(i, _)| i)
            .collect();
        neighbors
    }
}
