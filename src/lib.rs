//! This is a Rust driver for the SD1605AL real-time clock,
//! based on the [`embedded-hal`] traits and [`rtcc`].
//!
//! This chip is used by the [Gravity I2C SD2405 RTC Module SKU DFR0469]
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//! [`rtcc`]: https://github.com/eldruin/rtcc-rs
//! [Gravity I2C SD2405 RTC Module SKU DFR0469]: https://wiki.dfrobot.com/Gravity__I2C_SD2405_RTC_Module_SKU__DFR0469#More_Documents
//!
//! This driver allow you to:
//!  - Read date and time in 12-hour and 24-hour format. See: [`get_datetime`].
//!  - Set date and time in 12-hour and 24-hour format. See: [`set_datetime`].
//!
//! [`get_datetime`]: struct.Sd2405al.html#method.get_datetime
//! [`set_datetime`]: struct.Sd2405al.html#method.set_datetime
//!
#![deny(unsafe_code, missing_docs)]
#![no_std]
mod datetime;
pub mod interface;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use interface::I2cInterface;

const DEVICE_ADDRESS: u8 = 0x32;

/// Errors this library can return
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2c(E),

    /// Invalid argument
    InvalidInputData,
}

mod register {
    pub const SECONDS: u8 = 0x00;
    pub const MINUTES: u8 = 0x01;
    pub const HOURS: u8 = 0x02;
    pub const DOW: u8 = 0x03;
    pub const DOM: u8 = 0x04;
    pub const MONTH: u8 = 0x05;
    pub const YEAR: u8 = 0x06;
    pub const CONTROL_1: u8 = 0x0F;
    pub const CONTROL_2: u8 = 0x10;
    pub const TIME_TRIMMING: u8 = 0x12;
}

mod flag {
    pub const H12_H24: u8 = 0b1000_0000;
    pub const AM_PM: u8 = 0b0010_0000;
    pub const WRITE_RTC1: u8 = 0b1000_0000;
    pub const WRITE_RTC2: u8 = 0b0000_0100;
    pub const WRITE_RTC3: u8 = 0b1000_0000;
}

/// Sd2405al driver
#[derive(Debug, Default)]
pub struct Sd2405al<I2C> {
    iface: I2cInterface<I2C>,
}

impl<I2C, E> Sd2405al<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance
    pub fn new(i2c: I2C) -> Self {
        Sd2405al {
            iface: I2cInterface { i2c },
        }
    }

    /// Destroy and return ownership of I2C
    pub fn destroy(self) -> I2C {
        self.iface.i2c
    }
}

mod private {
    use super::interface;
    pub trait Sealed {}
    impl<I2C> Sealed for interface::I2cInterface<I2C> {}
}

/// Useful exports
pub mod prelude {
    pub use crate::Sd2405al;
    pub use rtcc::{Datelike, Hours, NaiveDate, NaiveDateTime, NaiveTime, Rtcc, Timelike};
}
