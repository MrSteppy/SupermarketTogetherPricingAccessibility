use rdev::{EventType, Key};
use std::thread::sleep;
use std::time::Duration;
use supermarket_together_pricing_accessibility::input::{Digit, Character};
use supermarket_together_pricing_accessibility::keyboard::Keyboard;
use supermarket_together_pricing_accessibility::price::Price;
use supermarket_together_pricing_accessibility::{double, round_down, FIVE};

fn main() {
  run();
}

fn run() {
  println!("Okay! Use num-keys 0-9, the comma key and regular enter to type a price.");
  println!("I will double that, round down to the next five cents and enter it on your numpad!");
  println!("Make sure your numpad is activated!");

  let mut price = Price::default();
  let mut keyboard = Keyboard::new().expect("keyboard");
  rdev::listen(move |event| {
    if let EventType::KeyPress(key) = event.event_type {
      let input = match key {
        Key::Num1 => Digit::One.into(),
        Key::Num2 => Digit::Two.into(),
        Key::Num3 => Digit::Three.into(),
        Key::Num4 => Digit::Four.into(),
        Key::Num5 => Digit::Five.into(),
        Key::Num6 => Digit::Six.into(),
        Key::Num7 => Digit::Seven.into(),
        Key::Num8 => Digit::Eight.into(),
        Key::Num9 => Digit::Nine.into(),
        Key::Num0 => Digit::Zero.into(),
        Key::Comma | Key::Dot => Character::Decimal,
        Key::Return => {
          let cents = price.as_cents();
          let new_cents = round_down(double(cents), FIVE);
          let new_price = Price::from(new_cents);

          println!("${price} → ${new_price}");

          for input in new_price.as_inputs() {
            keyboard.type_on_numpad(input).expect(&format!("type {input}"));
            sleep(Duration::from_millis(100));
          }
          //enter new price
          keyboard.type_numpad_enter().expect("type enter");

          //reset price
          price = Price::default();
          return
        },
        _ => return,
      };
      price += input;
    }
  })
  .expect("failed to setup key listener");
}
