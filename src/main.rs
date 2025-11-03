use std::thread::sleep;
use std::time::Duration;
use supermarket_together_pricing_accessibility::keyboard::Keyboard;

fn main() {
  sleep(Duration::from_secs(3));
  let mut keyboard = Keyboard::new().expect("keyboard");
  keyboard.type_on_numpad().expect("type on numpad");
}
