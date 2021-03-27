// use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
// use embedded_hal_mock::Transaction as I2cTransaction;
use embedded_hal_mock::i2c::Mock as I2cMock;
use sd2405al;

#[test]
fn can_create_and_destroy() {
    let i2c = I2cMock::new(&[]);

    let driver = sd2405al::Sd2405al::new(i2c);
    driver.destroy();
}
