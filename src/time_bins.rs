use lazy_static::lazy_static;
use std::convert::TryInto;
use std::ops::{Add, Sub};
use std::time::Duration;

pub const COUNT: usize = 48; // instead of u8 for external use
const TIME_BIN_SECS: usize = 30 * 60;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TimeBin(u8);
impl Add<Duration> for TimeBin {
    type Output = Self;
    fn add(self, other: Duration) -> Self {
        let offset = (other.as_secs_f64() / (TIME_BIN_SECS as f64)).ceil() as u8;
        TimeBin((self.0 + offset) % (COUNT as u8))
    }
}
impl Sub for TimeBin {
    type Output = u8;
    fn sub(self, other: Self) -> Self::Output {
        if self.0 > other.0 {
            self.0 - other.0
        } else {
            self.0 + (COUNT as u8) - other.0
        }
    }
}
impl TimeBin {
    pub fn value(&self) -> usize {
        self.0 as usize
    }
}

lazy_static! {
    pub static ref TIME_BINS: [TimeBin; COUNT] = (0..COUNT)
        .map(|i| TimeBin(i as u8))
        .collect::<Vec<TimeBin>>()
        .try_into()
        .unwrap();
}
