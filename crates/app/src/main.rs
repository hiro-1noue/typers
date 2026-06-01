use rdev::{listen, Event, EventType};
use std::sync::{Mutex, LazyLock};
use typers_core::{Dir, classify, accumulate, Decoder};

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::new()));

struct State {
    buf: Vec<(f64, f64)>,
    current_dir: Option<Dir>,
    decoder: Decoder,
}

impl State {
    fn new() -> Self {
        Self {
            buf: Vec::new(),
            current_dir: None,
            decoder: Decoder::new(),
        }
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
            if let Some(ch) = s.decoder.push(dir) {
                println!("decoded: {ch}");
            }

            s.current_dir = Some(dir);
        }
    }}

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
