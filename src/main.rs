use std::thread::sleep;
use std::time::Duration;
use rdev::{EventType, Key};
use supermarket_together_pricing_accessibility::keyboard::Keyboard;

fn main() {
  run();

  sleep(Duration::from_secs(3));
  let mut keyboard = Keyboard::new().expect("keyboard");
  keyboard.type_on_numpad().expect("type on numpad");
}

fn run() {
  rdev::listen(|event| {
    if let EventType::KeyPress(key) = event.event_type {
      match key {
        Key::Num1 => {
          println!("User typed 1!");
        }
        _ => {}
      }
    }
  }).expect("failed to key listener");
}
