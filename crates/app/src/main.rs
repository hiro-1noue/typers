use rdev::{listen, Event, EventType};

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    if let EventType::MouseMove { x, y } = event.event_type {
        println!("{} {}", x, y);
    }
}
