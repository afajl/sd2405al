//! I2C interface
use crate::{private, Error, DEVICE_ADDRESS};
use embedded_hal::blocking::i2c;

/// I2C interface
#[derive(Debug, Default)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
}

/// Write data
pub trait WriteData: private::Sealed {
    /// Error type
    type Error;

    /// Write to an u8 register
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;

    /// Write data. The first element corresponds to the starting address.
    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    type Error = Error<E>;

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write(DEVICE_ADDRESS, &payload).map_err(Error::I2c)
    }

    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.i2c.write(DEVICE_ADDRESS, &payload).map_err(Error::I2c)
    }
}

/// Read data
pub trait ReadData: private::Sealed {
    /// Error type
    type Error;
    /// Read an u8 register
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    /// Read some data. The first element corresponds to the starting address.
    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    type Error = Error<E>;

    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Error<E>> {
        let len = payload.len();
        self.i2c
            .write_read(DEVICE_ADDRESS, &[payload[0]], &mut payload[1..len])
            .map_err(Error::I2c)
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::I2c)
            .and(Ok(data[0]))
    }
}
