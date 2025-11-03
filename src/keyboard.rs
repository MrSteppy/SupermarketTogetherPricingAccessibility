use crate::input::{Digit, Input};
use enigo::{Direction, Enigo, Key, Settings};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Keyboard {
  enigo: Enigo,
}

impl Keyboard {
  pub fn new() -> Result<Self, String> {
    let enigo = Enigo::new(&Settings::default()).map_err(|e| format!("can't create enigo: {e}"))?;
    Ok(Keyboard { enigo })
  }

  pub fn type_on_numpad<I>(&mut self, input: I) -> Result<(), String>
  where
    I: Into<Input>,
  {
    let input = input.into();
    let key = match input {
      Input::Digit(digit) => match digit {
        Digit::Zero => Key::Numpad0,
        Digit::One => Key::Numpad1,
        Digit::Two => Key::Numpad2,
        Digit::Three => Key::Numpad3,
        Digit::Four => Key::Numpad4,
        Digit::Five => Key::Numpad5,
        Digit::Six => Key::Numpad6,
        Digit::Seven => Key::Numpad7,
        Digit::Eight => Key::Numpad8,
        Digit::Nine => Key::Numpad9,
      },
      Input::Decimal => Key::Decimal,
    };
    enigo::Keyboard::key(&mut self.enigo, key, Direction::Click)
      .map_err(|e| format!("failed to type {input}: {e}"))?;
    Ok(())
  }

  pub fn type_0_42_on_numpad(&mut self) -> Result<(), String> {
    for key in [
      Key::Numpad0,
      Key::Decimal,
      Key::Numpad4,
      Key::Numpad2,
      Key::NumpadEnter,
    ] {
      enigo::Keyboard::key(&mut self.enigo, key, Direction::Click).map_err(|e| format!("{e}"))?;
      sleep(Duration::from_millis(100));
    }
    Ok(())
  }
}
