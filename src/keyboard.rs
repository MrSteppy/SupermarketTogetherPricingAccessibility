use crate::input::{Character, Digit, Input};
use enigo::{Direction, Enigo, Key, Settings};

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
      Input::Char(c) => match c {
        Character::Digit(digit) => match digit {
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
        Character::Decimal => Key::Decimal,
      },
      Input::Enter => Key::NumpadEnter,
    };
    enigo::Keyboard::key(&mut self.enigo, key, Direction::Click)
      .map_err(|e| format!("failed to type {:?}: {e}", input))?;
    Ok(())
  }
}
