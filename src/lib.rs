//! Implementation of the 1-Wire protocol.
//!
//! [1-Wire](https://www.maximintegrated.com/en/design/technical-documents/app-notes/1/126.html)

#![no_std]
#![feature(error_in_core)]

pub use self::{error::Error, rom::Rom};

use self::configuration::Configuration;
use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin},
};
use error::Ds18b20Error;

pub const FAMILY_CODE: u8 = 0x28;

/// Ds18b20
pub struct Ds18b20 {
    rom: Rom,
}

impl Ds18b20 {
    /// Checks that the given code contains the correct family code, reads
    /// configuration data, then returns a device
    pub fn new(rom: Rom) -> Result<Ds18b20, Ds18b20Error> {
        match rom.family_code {
            FAMILY_CODE => Ok(Self { rom }),
            _ => Err(Ds18b20Error::UnexpectedFamilyCode {
                family_code: rom.family_code,
            }),
        }
    }

    /// Returns the device rom
    pub fn rom(&self) -> &Rom {
        &self.rom
    }
}

/// Ds18b20 driver
#[derive(Clone, Copy, Debug, Default)]
pub struct Driver<T, U> {
    pin: T,
    delay: U,
    configuration: Configuration,
}

impl<T: InputPin + OutputPin + ErrorType, U: DelayNs> Driver<T, U> {
    pub fn new(pin: T, delay: U) -> Result<Self, Error<T::Error>> {
        let mut driver = Self {
            pin,
            delay,
            configuration: Default::default(),
        };
        // Pin should be high during idle.
        driver.set_high()?;
        Ok(driver)
    }
}

/// Basic input pin operations
impl<T: InputPin + ErrorType, U> Driver<T, U> {
    pub fn is_high(&mut self) -> Result<bool, Error<T::Error>> {
        self.pin.is_high().map_err(Error::Pin)
    }

    pub fn is_low(&mut self) -> Result<bool, Error<T::Error>> {
        self.pin.is_low().map_err(Error::Pin)
    }
}

/// Basic output pin operations
impl<T: OutputPin + ErrorType, U> Driver<T, U> {
    /// Set the output as high.
    ///
    /// Disconnects the bus, letting another device (or the pull-up resistor)
    pub fn set_high(&mut self) -> Result<(), Error<T::Error>> {
        self.pin.set_high().map_err(Error::Pin)
    }

    /// Set the output as low.
    pub fn set_low(&mut self) -> Result<(), Error<T::Error>> {
        self.pin.set_low().map_err(Error::Pin)
    }
}

/// Basic delay operations
impl<T, U: DelayNs> Driver<T, U> {
    pub fn delay(&mut self, ns: u32) {
        self.delay.delay_ns(ns);
    }
}

/// Bit operations
impl<T: InputPin + OutputPin + ErrorType, U: DelayNs> Driver<T, U> {
    /// Read a bit from the 1-Wire bus and return it. Provide 10us recovery
    /// time.
    pub fn read_bit(&mut self) -> Result<bool, Error<T::Error>> {
        self.set_low()?;
        self.delay(self.configuration.a);
        self.set_high()?;
        self.delay(self.configuration.e);
        let bit = self.is_high()?;
        self.delay(self.configuration.f);
        Ok(bit)
    }

    /// Send a 1-Wire write bit. Provide 10us recovery time.
    pub fn write_bit(&mut self, bit: bool) -> Result<(), Error<T::Error>> {
        self.set_low()?;
        self.delay(if bit {
            self.configuration.a
        } else {
            self.configuration.c
        });
        self.set_high()?;
        self.delay(if bit {
            self.configuration.b
        } else {
            self.configuration.d
        });
        Ok(())
    }
}

/// Byte operations
impl<T: InputPin + OutputPin + ErrorType, U: DelayNs> Driver<T, U> {
    /// Read 1-Wire data byte.
    pub fn read_byte(&mut self) -> Result<u8, Error<T::Error>> {
        let mut byte = 0;
        for _ in 0..u8::BITS {
            byte >>= 1;
            if self.read_bit()? {
                byte |= 0x80;
            }
        }
        Ok(byte)
    }

    pub fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<(), Error<T::Error>> {
        for byte in bytes {
            *byte = self.read_byte()?;
        }
        Ok(())
    }

    /// Write 1-Wire data byte.
    pub fn write_byte(&mut self, mut byte: u8) -> Result<(), Error<T::Error>> {
        for _ in 0..u8::BITS {
            self.write_bit(byte & 0x01 == 0x01)?;
            byte >>= 1;
        }
        Ok(())
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error<T::Error>> {
        for byte in bytes {
            self.write_byte(*byte)?;
        }
        Ok(())
    }
}

pub mod command;
pub mod crc8;

mod configuration;
mod error;
mod rom;
mod scratchpad;
mod transactions;
