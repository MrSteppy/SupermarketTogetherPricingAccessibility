use enigo::{Direction, Enigo, Key, Settings};
use std::thread::sleep;
use std::time::Duration;

pub struct Keyboard {
  enigo: Enigo,
}

impl Keyboard {
  pub fn new() -> Result<Self, String> {
    let enigo = Enigo::new(&Settings::default()).map_err(|e| format!("can't create enigo: {e}"))?;
    Ok(Keyboard { enigo })
  }

  pub fn type_on_numpad(&mut self) -> Result<(), String> {
    for key in [Key::Numpad0, Key::Decimal, Key::Numpad4, Key::Numpad2, Key::NumpadEnter] {
      enigo::Keyboard::key(&mut self.enigo, key, Direction::Click).map_err(|e| format!("{e}"))?;
      sleep(Duration::from_millis(100));
    }
    Ok(())
  }
}
