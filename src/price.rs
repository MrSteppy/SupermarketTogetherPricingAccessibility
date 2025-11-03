use crate::input::{Digit, Input};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Price {
  pub value: Vec<Digit>,
  pub decimal_part: Option<DecimalPart>,
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
    let input = input.into();
    match input {
      Input::Digit(digit) => {
        if let Some(decimal_part) = &mut self.decimal_part {
          if let Some(first_decimal_digit) = &mut decimal_part.first_decimal_digit {
            if first_decimal_digit.second_decimal_digit.is_some() {
              return Err(AddInputError::MoreThanTwoDecimalPlaces);
            } else {
              first_decimal_digit.second_decimal_digit = Some(digit);
            }
          } else {
            decimal_part.first_decimal_digit = Some(FirstDecimalDigit::from(digit))
          }
        } else {
          self.value.push(digit);
        }
      }
      Input::Decimal => {
        if self.decimal_part.is_some() {
          return Err(AddInputError::DecimalAlreadyPresent);
        } else {
          self.decimal_part = Some(DecimalPart::default());
        }
      }
    }
    Ok(())
  }

  pub fn as_inputs(&self) -> Vec<Input> {
    let mut inputs = vec![];
    if self.value.is_empty() {
      inputs.push(Input::from(Digit::Zero));
    } else {
      inputs.append(&mut self.value.iter().map(|digit| Input::from(*digit)).collect());
    }

    if let Some(decimal_part) = &self.decimal_part {
      if let Some(first_decimal_digit) = &decimal_part.first_decimal_digit {
        inputs.push(Input::Decimal);
        inputs.push(Input::from(first_decimal_digit.digit));
        if let Some(second_decimal_digit) = first_decimal_digit.second_decimal_digit {
          inputs.push(Input::from(second_decimal_digit));
        }
      }
    }

    inputs
  }

  pub fn as_cents(&self) -> u32 {
    let mut value = self
      .value
      .iter()
      .fold(0, |cents, &digit| cents * 10 + digit * 100);
    if let Some(decimal_part) = &self.decimal_part {
      if let Some(first_decimal_digit) = &decimal_part.first_decimal_digit {
        value += first_decimal_digit.digit * 10;
        if let Some(second_decimal_digit) = first_decimal_digit.second_decimal_digit {
          value += second_decimal_digit;
        }
      }
    }
    value
  }

  pub fn from_cents(mut cents: u32) -> Self {
    let mut inverse_inputs = vec![];
    while cents > 0 {
      inverse_inputs.push(Digit::try_from(cents % 10).expect("valid digit").into());
      cents /= 10;

      if inverse_inputs.len() == 2 {
        inverse_inputs.push(Input::Decimal);
      }
    }

    let mut price = Price::default();
    for input in inverse_inputs.into_iter().rev() {
      price += input;
    }

    price
  }
}

impl<I> Add<I> for Price
where
  I: Into<Input>,
{
  type Output = Self;

  fn add(mut self, rhs: I) -> Self::Output {
    Price::add(&mut self, rhs);
    self
  }
}

impl<I> AddAssign<I> for Price
where
  I: Into<Input>,
{
  fn add_assign(&mut self, rhs: I) {
    Price::add(self, rhs);
  }
}

impl From<Price> for u32 {
  fn from(value: Price) -> Self {
    value.as_cents()
  }
}

impl From<u32> for Price {
  fn from(value: u32) -> Self {
    Self::from_cents(value)
  }
}

impl Display for Price {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      self
        .as_inputs()
        .into_iter()
        .map(|input| input.as_char())
        .collect::<String>()
    )
  }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct DecimalPart {
  pub first_decimal_digit: Option<FirstDecimalDigit>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct FirstDecimalDigit {
  pub digit: Digit,
  pub second_decimal_digit: Option<Digit>,
}

impl From<Digit> for FirstDecimalDigit {
  fn from(digit: Digit) -> Self {
    Self {
      digit,
      second_decimal_digit: None,
    }
  }
}

#[cfg(test)]
mod test_price {
  use crate::input::{Digit, Input};
  use crate::price::{AddInputError, DecimalPart, FirstDecimalDigit, Price};

  macro_rules! assert_no_price_change {
    ($price: ident, $operation: expr) => {{
      let old_price = $price.clone();
      $operation;
      assert_eq!(old_price, $price);
    }};
  }

  #[test]
  fn test_try_add() {
    let mut price = Price::default();

    macro_rules! assert_invariant_decimal_addition {
      () => {
        assert_no_price_change!(
          price,
          assert_eq!(
            Err(AddInputError::DecimalAlreadyPresent),
            price.try_add(Input::Decimal)
          )
        );
      };
    }

    price.try_add(Digit::Zero).expect("add zero");
    assert_eq!(
      Price {
        value: vec![Digit::Zero],
        decimal_part: None,
      },
      price
    );
    price.try_add(Input::Decimal).expect("add decimal");
    assert_eq!(
      Price {
        value: vec![Digit::Zero],
        decimal_part: Some(DecimalPart::default()),
      },
      price
    );
    assert_invariant_decimal_addition!();
    price.try_add(Digit::Four).expect("add four");
    assert_eq!(
      Price {
        value: vec![Digit::Zero],
        decimal_part: Some(DecimalPart {
          first_decimal_digit: Some(FirstDecimalDigit {
            digit: Digit::Four,
            second_decimal_digit: None
          }),
        }),
      },
      price
    );
    assert_invariant_decimal_addition!();
    price.try_add(Digit::Two).expect("add two");
    assert_eq!(
      Price {
        value: vec![Digit::Zero],
        decimal_part: Some(DecimalPart {
          first_decimal_digit: Some(FirstDecimalDigit {
            digit: Digit::Four,
            second_decimal_digit: Some(Digit::Two),
          }),
        }),
      },
      price
    );
    assert_invariant_decimal_addition!();
    assert_no_price_change!(
      price,
      assert_eq!(
        Err(AddInputError::MoreThanTwoDecimalPlaces),
        price.try_add(Digit::Zero)
      )
    );
  }

  #[test]
  fn test_as_inputs() {
    assert_eq!(
      vec![
        Input::from(Digit::Zero),
        Input::Decimal,
        Input::from(Digit::Four),
        Input::from(Digit::Two)
      ],
      Price {
        value: vec![],
        decimal_part: Some(DecimalPart {
          first_decimal_digit: Some(FirstDecimalDigit {
            digit: Digit::Four,
            second_decimal_digit: Some(Digit::Two)
          }),
        }),
      }
      .as_inputs()
    );
  }

  #[test]
  fn test_as_cents() {
    assert_eq!(
      42,
      Price {
        value: vec![],
        decimal_part: Some(DecimalPart {
          first_decimal_digit: Some(FirstDecimalDigit {
            digit: Digit::Four,
            second_decimal_digit: Some(Digit::Two)
          })
        })
      }
      .as_cents()
    );
    assert_eq!(
      2830,
      Price {
        value: vec![Digit::Two, Digit::Eight],
        decimal_part: Some(DecimalPart {
          first_decimal_digit: Some(FirstDecimalDigit {
            digit: Digit::Three,
            second_decimal_digit: None,
          })
        })
      }
      .as_cents()
    );
  }

  #[test]
  fn test_from_cents() {
    assert_eq!(
      Price {
        value: vec![],
        decimal_part: Some(DecimalPart {
          first_decimal_digit: Some(FirstDecimalDigit {
            digit: Digit::Four,
            second_decimal_digit: Some(Digit::Two)
          })
        })
      },
      Price::from_cents(42)
    );
    assert_eq!(
      Price {
        value: vec![Digit::Two, Digit::Eight],
        decimal_part: Some(DecimalPart {
          first_decimal_digit: Some(FirstDecimalDigit {
            digit: Digit::Three,
            second_decimal_digit: Some(Digit::Zero),
          })
        })
      },
      Price::from_cents(2830)
    );
  }
}