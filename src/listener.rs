use crate::input::Input;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use device_query::DeviceEventsHandler;
use crossterm::event::{Event, KeyCode};

pub fn listen() -> Result<Receiver<Input>, String> {
  let (sender, receiver) = mpsc::channel();

  #[cfg(feature = "rdev")]
  {
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
  }

  #[cfg(not(feature = "rdev"))]
  {
    let event_state = DeviceEventsHandler::new(Duration::from_millis(10)).ok_or("failed to setup event loop")?;

    thread::spawn(move || loop {

    });
    
    todo!("listen for key events")
  }

  Ok(receiver)
}
