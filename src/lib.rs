use std::num::NonZeroU32;

#[inline]
pub fn double(cents: u32) -> u32 {
  cents * 2
}

#[inline]
pub fn round_down(cents: u32, to: NonZeroU32) -> u32 {
  cents / to * to.get()
}

#[cfg(test)]
mod test {
  use crate::{double, round_down};
  use std::num::NonZeroU32;

  #[test]
  fn test_round_down() {
    let round_to = NonZeroU32::new(5).unwrap();
    assert_eq!(5, round_down(7, round_to));
    assert_eq!(0, round_down(3, round_to));
    assert_eq!(195, round_down(198, round_to));
  }

  #[test]
  fn test_double_and_round() {
    let round_to = NonZeroU32::new(5).unwrap();
    assert_eq!(200, round_down(double(100), round_to));
    assert_eq!(205, round_down(double(103), round_to));
    assert_eq!(210, round_down(double(105), round_to));
    assert_eq!(50, round_down(double(25), round_to));
    assert_eq!(215, round_down(double(108), round_to));
  }
}