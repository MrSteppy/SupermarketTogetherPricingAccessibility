use std::error::Error;
use std::fmt::{Display, Formatter};

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
      _ => Err("invalid digit value")?,
    })
  }
}

impl Display for Digit {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    u32::from(*self).fmt(f)
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Price {
  pub value: Vec<Digit>,
  pub decimal_part: Option<DecimalPart>,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct DecimalPart {
  pub first_decimal_digit: Option<FirstDecimalDigit>,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct FirstDecimalDigit {
  pub digit: Digit,
  pub second_decimal_digit: Option<Digit>,
}

impl Price {
  pub fn add<I>(&mut self, input: I)
  where
    I: Into<Input>,
  {
    let _ = self.try_add(input);
  }

  pub fn try_add<I>(&mut self, input: I) -> Result<(), AddInputError>
  where
    I: Into<Input>,
  {
    todo!("add input if possible")
  }

  pub fn as_inputs(&self) -> Vec<Input> {
    self
      .value
      .iter()
      .map(|digit| Input::from(*digit))
      .chain(self.decimal_part.iter().flat_map(|decimal_part| {
        decimal_part
          .first_decimal_digit
          .iter()
          .flat_map(|first_decimal_digit| {
            [Input::Decimal, Input::from(first_decimal_digit.digit)]
              .iter()
              .chain(
                first_decimal_digit
                  .second_decimal_digit
                  .iter()
                  .map(|digit| Input::from(*digit)),
              )
          })
      }))
      .collect()
  }

  pub fn as_cents(&self) -> u32 {
    todo!("convert price to cents")
  }

  pub fn from_cents(cents: u32) -> Self {
    todo!("calculate price from cents")
  }
}

#[derive(Debug)]
pub enum AddInputError {
  DecimalAlreadyPresent,
  MoreThanTwoDecimalPlaces,
}

impl Display for AddInputError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        AddInputError::DecimalAlreadyPresent => "a decimal is already present",
        AddInputError::MoreThanTwoDecimalPlaces => "can't add more than two decimal places",
      }
    )
  }
}

impl Error for AddInputError {}
