use std::thread::sleep;
use std::time::Duration;
use supermarket_together_pricing_accessibility::input::Input;
use supermarket_together_pricing_accessibility::keyboard::Keyboard;
use supermarket_together_pricing_accessibility::listener::listen;
use supermarket_together_pricing_accessibility::price::Price;
use supermarket_together_pricing_accessibility::{double, round_down, FIVE};

const TYPING_DELAY: Duration = Duration::from_millis(100); 

fn main() {
  run();
}

fn run() {
  println!("Okay! Use num-keys 0-9, the comma key and regular enter to type a price.");
  println!("I will double that, round down to the next five cents and enter it on your numpad!");
  println!("Make sure your numpad is activated!");

  let mut price = Price::default();
  let mut keyboard = Keyboard::new().expect("keyboard");
  let input_receiver = listen().expect("failed to setup listener");
  
  while let Ok(input) = input_receiver.recv() {
    match input {
      Input::Char(char) => price += char,
      Input::Enter => {
        let new_price = double_and_round(&price);

        println!("${price} → ${new_price}");

        //type in new price on numpad
        for char in new_price.as_inputs() {
          keyboard.type_on_numpad(char).expect(&format!("type {char}"));
          sleep(TYPING_DELAY);
        }
        keyboard.type_on_numpad(Input::Enter).expect("type enter");

        //reset price
        price = Price::default();
      }
    }
  }
}

fn double_and_round(price: &Price) -> Price {
  let cents = price.as_cents();
  let double = double(cents);
  let rounded = round_down(double, FIVE);
  let price = Price::from(rounded);
  price
}
