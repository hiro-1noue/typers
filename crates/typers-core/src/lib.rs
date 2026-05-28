#[derive(Debug, Clone, Copy, PartialEq)]
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

