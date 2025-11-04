use crate::input::{Character, Digit, Input};
use rdev::{EventType, Key};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

pub fn listen() -> Result<Receiver<Input>, String> {
  let (sender, receiver) = mpsc::channel();
  thread::spawn(move || rdev::listen(move |event| if let EventType::KeyPress(key) = event.event_type {
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
      Key::Comma => Character::Decimal.into(),
      Key::Dot => Character::Decimal.into(),
      Key::Return => Input::Enter,
      _ => return,
    };
    let _ = sender.send(input);
  }));

  Ok(receiver)
}
