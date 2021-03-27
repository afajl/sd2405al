#![forbid(unsafe_code)]

use crate::{flag, register};
use crate::{
    interface::{ReadData, WriteData},
    Error, Sd2405al,
};

use embedded_hal::blocking::i2c::{Write, WriteRead};
use rtcc::{Datelike, Hours, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

// Transforms a number in packed BCD format to decimal
fn packed_bcd_to_decimal(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0xF)
}

fn decimal_to_packed_bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}

fn hours_to_register<E>(hours: Hours) -> Result<u8, Error<E>> {
    match hours {
        Hours::H24(h) if h > 23 => Err(Error::InvalidInputData),
        Hours::H24(h) => Ok(flag::H12_H24 | decimal_to_packed_bcd(h)),
        Hours::AM(h) if h < 1 || h > 12 => Err(Error::InvalidInputData),
        Hours::AM(h) => Ok(decimal_to_packed_bcd(h)),
        Hours::PM(h) if h < 1 || h > 12 => Err(Error::InvalidInputData),
        Hours::PM(h) => Ok(flag::AM_PM | decimal_to_packed_bcd(h)),
    }
}

fn hours_from_register(data: u8) -> Hours {
    if is_24h_format(data) {
        Hours::H24(packed_bcd_to_decimal(data & !flag::H12_H24))
    } else if is_am(data) {
        Hours::AM(packed_bcd_to_decimal(data & !(flag::H12_H24 | flag::AM_PM)))
    } else {
        Hours::PM(packed_bcd_to_decimal(data & !(flag::H12_H24 | flag::AM_PM)))
    }
}

fn is_24h_format(hours_data: u8) -> bool {
    hours_data & flag::H12_H24 != 0
}

fn is_am(hours_data: u8) -> bool {
    hours_data & flag::AM_PM == 0
}

fn get_h24(hour: Hours) -> u8 {
    match hour {
        Hours::H24(h) => h,
        Hours::AM(h) => h,
        Hours::PM(h) => h + 12,
    }
}

impl<I2C, E> Sd2405al<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    fn read_register_decimal(&mut self, register: u8) -> Result<u8, Error<E>> {
        let data = self.iface.read_register(register)?;
        Ok(packed_bcd_to_decimal(data))
    }

    fn enable_write(&mut self) -> Result<(), Error<E>> {
        self.iface
            .write_register(register::CONTROL_2, flag::WRITE_RTC1)?;
        self.iface
            .write_register(register::CONTROL_1, flag::WRITE_RTC2 | flag::WRITE_RTC3)
    }

    fn disable_write(&mut self) -> Result<(), Error<E>> {
        self.iface.write_data(&mut [register::CONTROL_1, 0x0, 0x0])
    }
}

impl<I2C, E> rtcc::Rtcc for Sd2405al<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    type Error = Error<E>;

    fn get_seconds(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(register::SECONDS)
    }

    fn get_minutes(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(register::MINUTES)
    }

    fn get_hours(&mut self) -> Result<Hours, Self::Error> {
        let hours = self.read_register_decimal(register::HOURS)?;
        Ok(hours_from_register(hours))
    }

    fn get_time(&mut self) -> Result<rtcc::NaiveTime, Self::Error> {
        let mut data = [0; 4];
        self.iface.read_data(&mut data)?;
        let hour = hours_from_register(data[register::HOURS as usize + 1]);
        let minute = packed_bcd_to_decimal(data[register::MINUTES as usize + 1]);
        let second = packed_bcd_to_decimal(data[register::SECONDS as usize + 1]);

        Ok(NaiveTime::from_hms(
            get_h24(hour).into(),
            minute.into(),
            second.into(),
        ))
    }

    fn get_weekday(&mut self) -> Result<u8, Self::Error> {
        let weekday = self.read_register_decimal(register::DOW)?;
        Ok(weekday + 1)
    }

    fn get_day(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(register::DOM)
    }

    fn get_month(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(register::MONTH)
    }

    fn get_year(&mut self) -> Result<u16, Self::Error> {
        let year = self.read_register_decimal(register::YEAR)?;
        Ok(2000 + year as u16)
    }

    fn get_date(&mut self) -> Result<rtcc::NaiveDate, Self::Error> {
        let mut data = [0; 4];
        data[0] = register::DOM;
        self.iface.read_data(&mut data)?;

        let offset = register::DOM as usize;
        let year = 2000 + packed_bcd_to_decimal(data[register::YEAR as usize + 1 - offset]) as u16;
        let month = packed_bcd_to_decimal(data[register::MONTH as usize + 1 - offset]);
        let day = packed_bcd_to_decimal(data[register::DOM as usize + 1 - offset]);
        Ok(NaiveDate::from_ymd(year.into(), month.into(), day.into()))
    }

    fn get_datetime(&mut self) -> Result<NaiveDateTime, Self::Error> {
        let mut data = [0; 8];
        self.iface.read_data(&mut data)?;

        let year = 2000 + packed_bcd_to_decimal(data[register::YEAR as usize + 1]) as u16;
        let month = packed_bcd_to_decimal(data[register::MONTH as usize + 1]);
        let day = packed_bcd_to_decimal(data[register::DOM as usize + 1]);
        let hour = hours_from_register(data[register::HOURS as usize + 1]);
        let minute = packed_bcd_to_decimal(data[register::MINUTES as usize + 1]);
        let second = packed_bcd_to_decimal(data[register::SECONDS as usize + 1]);

        Ok(
            rtcc::NaiveDate::from_ymd(year.into(), month.into(), day.into()).and_hms(
                get_h24(hour).into(),
                minute.into(),
                second.into(),
            ),
        )
    }

    fn set_seconds(&mut self, _seconds: u8) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_minutes(&mut self, _minutes: u8) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_hours(&mut self, _hours: Hours) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_time(&mut self, _time: &rtcc::NaiveTime) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_weekday(&mut self, _weekday: u8) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_day(&mut self, _day: u8) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_month(&mut self, _month: u8) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_year(&mut self, _year: u16) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_date(&mut self, _date: &rtcc::NaiveDate) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_datetime(&mut self, datetime: &NaiveDateTime) -> Result<(), Self::Error> {
        if datetime.year() < 2000 || datetime.year() > 2100 {
            return Err(Error::InvalidInputData);
        }
        let year = decimal_to_packed_bcd((datetime.year() - 2000) as u8);

        let mut payload = [
            register::SECONDS,
            decimal_to_packed_bcd(datetime.second() as u8),
            decimal_to_packed_bcd(datetime.minute() as u8),
            hours_to_register(Hours::H24(datetime.hour() as u8))?,
            datetime.weekday().number_from_sunday() as u8,
            decimal_to_packed_bcd(datetime.day() as u8),
            decimal_to_packed_bcd(datetime.month() as u8),
            year,
        ];

        self.enable_write()?;
        self.iface.write_data(&mut payload)?;
        self.disable_write()?;
        self.iface.write_register(register::TIME_TRIMMING, 0x0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_to_h24() {
        assert_eq!(0, get_h24(Hours::H24(0)));
        assert_eq!(0, get_h24(Hours::AM(0)));
        assert_eq!(12, get_h24(Hours::PM(0)));

        assert_eq!(1, get_h24(Hours::H24(1)));
        assert_eq!(1, get_h24(Hours::AM(1)));
        assert_eq!(13, get_h24(Hours::PM(1)));

        assert_eq!(23, get_h24(Hours::H24(23)));
        assert_eq!(12, get_h24(Hours::AM(12)));
        assert_eq!(23, get_h24(Hours::PM(11)));
    }
}
