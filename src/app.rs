use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub enum Digit {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
}

impl From<Digit> for u8 {
  fn from(value: Digit) -> Self {
    match value {
      Digit::Zero => 0,
      Digit::One => 1,
      Digit::Two => 2,
      Digit::Three => 3,
      Digit::Four => 4,
      Digit::Five => 5,
      Digit::Six => 6,
      Digit::Seven => 7,
      Digit::Eight => 8,
      Digit::Nine => 9,
    }
  }
}

impl Display for Digit {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    u8::from(self).fmt(f)
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Input {
  Digit(Digit),
  Decimal,
}

impl From<Digit> for Input {
  fn from(value: Digit) -> Self {
    Input::Digit(value)
  }
}

impl Display for Input {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Input::Digit(digit) => digit.fmt(f),
      Input::Decimal => write!(f, "."),
    }
  }
}