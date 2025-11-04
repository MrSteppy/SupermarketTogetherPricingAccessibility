use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, Mul};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, Default)]
pub enum Digit {
  #[default]
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

impl Digit {
  pub fn as_char(&self) -> char {
    match self {
      Digit::Zero => '0',
      Digit::One => '1',
      Digit::Two => '2',
      Digit::Three => '3',
      Digit::Four => '4',
      Digit::Five => '5',
      Digit::Six => '6',
      Digit::Seven => '7',
      Digit::Eight => '8',
      Digit::Nine => '9',
    }
  }
}

impl Mul<u32> for Digit {
  type Output = u32;

  fn mul(self, rhs: u32) -> Self::Output {
    Self::Output::from(self) * rhs
  }
}

impl AddAssign<Digit> for u32 {
  fn add_assign(&mut self, rhs: Digit) {
    Self::add_assign(self, Self::from(rhs));
  }
}

impl From<Digit> for u32 {
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

impl TryFrom<u32> for Digit {
  type Error = &'static str;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => Digit::Zero,
      1 => Digit::One,
      2 => Digit::Two,
      3 => Digit::Three,
      4 => Digit::Four,
      5 => Digit::Five,
      6 => Digit::Six,
      7 => Digit::Seven,
      8 => Digit::Eight,
      9 => Digit::Nine,
      _ => return Err("invalid digit value"),
    })
  }
}

impl Display for Digit {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_char())
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Character {
  Digit(Digit),
  Decimal,
}

impl From<Digit> for Character {
  fn from(value: Digit) -> Self {
    Character::Digit(value)
  }
}

impl Character {
  pub fn as_char(&self) -> char {
    match self {
      Character::Digit(digit) => digit.as_char(),
      Character::Decimal => '.',
    }
  }
}

impl Display for Character {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_char())
  }
}