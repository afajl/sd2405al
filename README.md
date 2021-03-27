# Rust driver for the SD2405AL real-time clock

Rust driver for the [SSD1681] real-time clock used on the
[Gravity I2C SD2405 RTC Module] for use with [embedded-hal] and
the [Rtcc] crate.

[![crates.io](https://img.shields.io/crates/v/sd2405al.svg)](https://crates.io/crates/sd2405al)
[![Documentation](https://docs.rs/sd2405al/badge.svg)](https://docs.rs/sd2405al/)


## Description

This simple driver allow you to set and read the time. The chip
only allows setting all datetime fields at once.

## Examples
The examples must be built on a Raspberry Pi. Use the
`run-example.sh` script to copy the sources, compile and run the
example.

## Credits

* [ds323x-rs](https://github.com/eldruin/ds323x-rs)

## License

`sd2405al` is dual licenced under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) **or**
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[embedded-hal]: https://crates.io/crates/embedded-hal
[LICENSE-APACHE]: https://github.com/wezm/ssd1675/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/wezm/ssd1675/blob/master/LICENSE-MIT
[SD2405AL]: https://www.robotshop.com/media/files/PDF/datasheet-toy0021.pdf
[Gravity I2C SD2405 RTC Module]: https://wiki.dfrobot.com/Gravity__I2C_SD2405_RTC_Module_SKU__DFR0469
[Rtcc]: https://github.com/eldruin/rtcc-rs
