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

pub fn build_map() -> HashMap<Vec<Dir>, &'static str> {
    let mut map = HashMap::new();

    map.insert(vec![Dir::R, Dir::U], "e");
    map.insert(vec![Dir::R, Dir::D], "t");
    map.insert(vec![Dir::L, Dir::U], "a");
    map.insert(vec![Dir::L, Dir::D], "o");
    map.insert(vec![Dir::U, Dir::R], "i");
    map.insert(vec![Dir::U, Dir::L], "n");
    map.insert(vec![Dir::D, Dir::R], "s");
    map.insert(vec![Dir::D, Dir::L], "h");
    map.insert(vec![Dir::R, Dir::L], "r");
    map.insert(vec![Dir::L, Dir::R], "d");
    map.insert(vec![Dir::U, Dir::D], "l");
    map.insert(vec![Dir::D, Dir::U], "u");

    map
}

