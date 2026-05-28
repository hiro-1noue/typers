use rdev::{listen, Event, EventType};
use std::sync::Mutex;
use typers_core::{Dir, classify, accumulate};

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
