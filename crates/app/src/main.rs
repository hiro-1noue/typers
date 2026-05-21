use enigo::{Enigo, Mouse, Settings};

fn main() {
    let enigo = Enigo::new(&Settings::default()).unwrap();
    let (x, y) = enigo.location().unwrap();

    println!("({}, {})", x, y);
}
