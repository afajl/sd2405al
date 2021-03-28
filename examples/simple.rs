// Simple example on how to set and read the date
//
// Use: i2cdetect -y -r 1 to check that you've wired
// the clock correctly. The device address 32 should show up.
//
#[cfg(not(target_os = "linux"))]
fn main() {}

#[cfg(target_os = "linux")]
use linux_embedded_hal::{Delay, I2cdev};

#[cfg(target_os = "linux")]
use embedded_hal::blocking::delay::DelayMs;

#[cfg(target_os = "linux")]
use sd2405al::prelude::*;

#[cfg(target_os = "linux")]
fn main() {
    let mut delay = Delay {};
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let mut clock = Sd2405al::new(i2c);

    let now = NaiveDateTime::new(
        NaiveDate::from_ymd(2019, 12, 31),
        NaiveTime::from_hms(23, 59, 59),
    );

    clock.set_datetime(&now).unwrap();
    delay.delay_ms(1500_u16);

    let later = clock.get_datetime().unwrap();

    println!(
        "later is: {}-{}-{} {:02}:{:02}:{:02}",
        later.date().year(),
        later.date().month(),
        later.date().day(),
        later.time().hour(),
        later.time().minute(),
        later.time().second()
    )
}
