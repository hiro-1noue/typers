use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir { R, L, U, D }

pub fn classify(dx: f64, dy: f64) -> Dir {
    if dx.abs() >= dy.abs() {
        if dx > 0.0 { Dir::R } else { Dir::L }
    } else {
        if dy > 0.0 { Dir::D } else { Dir::U }
    }
}

pub fn accumulate(pts: &[(f64, f64)]) -> (f64, f64) {
    pts.windows(2).fold((0.0, 0.0), |(ax, ay), w| {
        (ax + w[1].0 - w[0].0, ay + w[1].1 - w[0].1)
    })
}

pub fn build_map() -> HashMap<(Dir, Dir), &'static str> {
    let mut map = HashMap::new();

    map.insert((Dir::R, Dir::U), "e");
    map.insert((Dir::R, Dir::D), "t");
    map.insert((Dir::L, Dir::U), "a");
    map.insert((Dir::L, Dir::D), "o");
    map.insert((Dir::U, Dir::R), "i");
    map.insert((Dir::U, Dir::L), "n");
    map.insert((Dir::D, Dir::R), "s");
    map.insert((Dir::D, Dir::L), "h");
    map.insert((Dir::R, Dir::L), "r");
    map.insert((Dir::L, Dir::R), "d");
    map.insert((Dir::U, Dir::D), "l");
    map.insert((Dir::D, Dir::U), "u");

    map
}

pub struct Decoder {
    prev_dir: Option<Dir>,
    map: HashMap<(Dir, Dir), &'static str>,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            prev_dir: None,
            map: build_map(),
        }
    }

    pub fn push(&mut self, dir: Dir) -> Option<&'static str> {
        let result = if let Some(prev) = self.prev_dir {
            self.map.get(&(prev, dir)).copied()
        } else {
            None
        };

        self.prev_dir = Some(dir);
        result
    }
}


