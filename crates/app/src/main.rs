use rdev::{listen, Event, EventType};
use std::sync::Mutex;

static STATE: Mutex<State> = Mutex::new(State::new());

struct State {
    buf: Vec<(f64, f64)>,
    current_dir: Option<Dir>,
}

impl State {
    const fn new() -> Self {
        Self { buf: Vec::new(), current_dir: None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir { R, L, U, D }

fn classify(dx: f64, dy: f64) -> Dir {
    if dx.abs() >= dy.abs() {
        if dx > 0.0 { Dir::R } else { Dir::L }
    } else {
        if dy > 0.0 { Dir::D } else { Dir::U }
    }
}

fn accumulate(pts: &[(f64, f64)]) -> (f64, f64) {
    pts.windows(2).fold((0.0, 0.0), |(ax, ay), w| {
        (ax + w[1].0 - w[0].0, ay + w[1].1 - w[0].1)
    })
}

const WINDOW: usize = 7;

fn callback(event: Event) {
    if let EventType::MouseMove { x, y } = event.event_type {
        let mut s = STATE.lock().unwrap();
        s.buf.push((x, y));
        let len = s.buf.len();

        if len > WINDOW {
            s.buf.drain(..len - WINDOW);
        }

        if s.buf.len() < WINDOW { return; }

        let recent = &s.buf[s.buf.len() - WINDOW..];
        let (dx, dy) = accumulate(recent);

        // 移動量が小さい場合は無視（静止・ノイズ）
        if (dx * dx + dy * dy).sqrt() < 10.0 { return; }

        let dir = classify(dx, dy);

        if s.current_dir != Some(dir) {
            if let Some(prev) = s.current_dir {
                println!("{:?} → {:?}", prev, dir);
            }
            s.current_dir = Some(dir);
        }
    }}

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
