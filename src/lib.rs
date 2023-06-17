//! ds18b20
//!
//! [1-Wire Temperature Sensors](https://www.analog.com/en/parametricsearch/12873)
//! [analog.com](https://www.analog.com/en/technical-articles/interfacing-the-ds18x20ds1822-1wire-temperature-sensor-in-a-microcontroller-environment.html)
//!
//! - [ups_monitor](https://github.com/lesha108/sim800_ups_monitor)
//! - [Библиотека для работы с шиной 1-Wire на STM32](https://microtechnics.ru/biblioteka-dlya-raboty-s-shinoj-1-wire-na-stm32/)
//! - [STM32 и DS18B20](http://we.easyelectronics.ru/STM32/esche-raz-o-stm32-i-ds18b20-podpravleno.html)
//! - [Датчик температуры DS18B20](https://narodstream.ru/stm-urok-92-datchik-temperatury-ds18b20-chast-1/)

#![no_std]
#![feature(default_free_fn)]
#![feature(error_in_core)]
#![feature(trait_alias)]

use error::{Error, Result};
use one_wire::Code;

pub const FAMILY_CODE: u8 = 0x28;

pub struct Ds18b20 {
    code: Code,
}

impl Ds18b20 {
    /// Checks that the given code contains the correct family code, reads
    /// configuration data, then returns a device
    pub fn new(code: Code) -> Result<Ds18b20> {
        match code.family_code {
            FAMILY_CODE => Ok(Self { code }),
            _ => Err(Error::MismatchedFamilyCode),
        }
    }

    /// Returns the device code
    pub fn code(&self) -> &Code {
        &self.code
    }
}

pub mod commands;
pub mod scratchpad;

mod error;
