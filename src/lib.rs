pub mod keyboard;
pub mod app;

use std::num::NonZeroU32;

pub const FIVE: NonZeroU32 = NonZeroU32::new(5).expect("not zero");

#[inline]
pub fn double(cents: u32) -> u32 {
  cents * 2
}

#[inline]
pub fn round_down(cents: u32, to: NonZeroU32) -> u32 {
  cents / to * to.get()
}

#[inline]
pub fn round_down_to_five(cents: u32) -> u32 {
  round_down(cents, FIVE)
}

#[cfg(test)]
mod test {
  use crate::{double, round_down_to_five};

  #[test]
  fn test_round_down() {
    assert_eq!(5, round_down_to_five(7));
    assert_eq!(0, round_down_to_five(3));
    assert_eq!(195, round_down_to_five(198));
  }

  #[test]
  fn test_double_and_round() {
    assert_eq!(200, round_down_to_five(double(100)));
    assert_eq!(205, round_down_to_five(double(103)));
    assert_eq!(210, round_down_to_five(double(105)));
    assert_eq!(50, round_down_to_five(double(25)));
    assert_eq!(215, round_down_to_five(double(108)));
  }
}