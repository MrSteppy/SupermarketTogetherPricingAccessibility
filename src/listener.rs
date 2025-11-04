use crate::input::{Character, Digit, Input};
use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

pub fn listen() -> Result<Receiver<Input>, String> {
  let (sender, receiver) = mpsc::channel();

  let device_state =
    DeviceEventsHandler::new(Duration::from_millis(0)).unwrap_or(DeviceEventsHandler);

  thread::spawn(move || {
    let _guard = device_state.on_key_down(move |key_code| {
      let input = match key_code {
        Keycode::Key0 => Digit::Zero.into(),
        Keycode::Key1 => Digit::One.into(),
        Keycode::Key2 => Digit::Two.into(),
        Keycode::Key3 => Digit::Three.into(),
        Keycode::Key4 => Digit::Four.into(),
        Keycode::Key5 => Digit::Five.into(),
        Keycode::Key6 => Digit::Six.into(),
        Keycode::Key7 => Digit::Seven.into(),
        Keycode::Key8 => Digit::Eight.into(),
        Keycode::Key9 => Digit::Nine.into(),
        Keycode::Comma => Character::Decimal.into(),
        Keycode::Dot => Character::Decimal.into(),
        Keycode::Enter => Input::Enter,
        _ => return,
      };
      let _ = sender.send(input);
    });
    loop {} //keep guard alive
  });

  Ok(receiver)
}
