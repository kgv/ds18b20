use crate::{
    error::{Error, Result},
    scratchpad::Scratchpad,
    Command, Driver, Pin,
};
use embedded_hal::delay::DelayNs;

pub const COMMAND_MEMORY_CONVERT: u8 = 0x44;
pub const COMMAND_MEMORY_RECALL: u8 = 0xB8;
pub const COMMAND_MEMORY_POWER_SUPPLY_READ: u8 = 0xB4;
pub const COMMAND_MEMORY_SCRATCHPAD_COPY: u8 = 0x48;
pub const COMMAND_MEMORY_SCRATCHPAD_READ: u8 = 0xBE;
pub const COMMAND_MEMORY_SCRATCHPAD_WRITE: u8 = 0x4E;

const READ_SLOT_DURATION_MICROS: u16 = 70;

/// This command begins a temperature conversion. No further data is required.
/// The temperature conversion will be performed and then the DS18B20 will
/// remain idle. If the bus master issues read time slots following this
/// command, the DS18B20 will output 0 on the bus as long as it is busy making a
/// temperature conversion; it will return a 1 when the temperature conversion
/// is complete. If parasite-powered, the bus master has to enable a strong
/// pullup for a period greater than tconv immediately after issuing this
/// command.
///
/// You should wait for the measurement to finish before reading the
/// measurement. The amount of time you need to wait depends on the current
/// resolution configuration
#[derive(Clone, Copy, Debug, Default)]
pub struct ConvertTemperature;

impl Command for ConvertTemperature {
    type Output = Result<()>;

    fn execute(&self, driver: &mut Driver<impl Pin, impl DelayNs>) -> Self::Output {
        driver.write_byte(COMMAND_MEMORY_CONVERT)?;
        Ok(())
    }
}

/// Signals the mode of DS18B20 power supply to the master.
#[derive(Clone, Copy, Debug)]
pub enum ReadPowerSupply {
    /// Signals the mode of DS18B20 power supply to the master.
    Read,
}

/// Recalls values stored in nonvolatile memory (EEPROM, electrically erasable
/// programmable read-only memory) into scratchpad (temperature triggers). Load
/// config from EEPROM to scratchpad.
#[derive(Clone, Copy, Debug, Default)]
pub struct RecallE2;

impl Command for RecallE2 {
    type Output = Result<()>;

    fn execute(&self, driver: &mut Driver<impl Pin, impl DelayNs>) -> Self::Output {
        driver.write_byte(COMMAND_MEMORY_RECALL)?;
        // wait for the recall to finish (up to 10ms)
        let max_retries = (10000 / READ_SLOT_DURATION_MICROS) + 1;
        for _ in 0..max_retries {
            if driver.read_bit()? == true {
                return Ok(());
            }
        }
        Err(Error::Timeout)
    }
}

/// Copies scratchpad into nonvolatile memory (EEPROM) (addresses 2 through 4
/// only). Save config from scratchpad to EEPROM.
#[derive(Clone, Copy, Debug, Default)]
pub struct CopyScratchpad;

impl Command for CopyScratchpad {
    type Output = Result<()>;

    fn execute(&self, driver: &mut Driver<impl Pin, impl DelayNs>) -> Self::Output {
        driver.write_byte(COMMAND_MEMORY_SCRATCHPAD_COPY)?;
        driver.wait(10000); // delay 10ms for the write to complete
        Ok(())
    }
}

/// Reads bytes from scratchpad and reads CRC byte.
#[derive(Clone, Copy, Debug)]
pub struct ReadScratchpad;

impl Command for ReadScratchpad {
    type Output = Result<Scratchpad>;

    fn execute(&self, driver: &mut Driver<impl Pin, impl DelayNs>) -> Self::Output {
        driver.write_byte(COMMAND_MEMORY_SCRATCHPAD_READ)?;
        let mut bytes = [0; 9];
        driver.read_bytes(&mut bytes)?;
        bytes.try_into()
    }
}

/// Writes bytes into scratchpad at addresses 2 through 4 (TH and TL
/// temperature triggers and config).
#[derive(Clone, Copy, Debug, Default)]
pub struct WriteScratchpad {
    pub scratchpad: Scratchpad,
}

impl Command for WriteScratchpad {
    type Output = Result<()>;

    fn execute(&self, driver: &mut Driver<impl Pin, impl DelayNs>) -> Self::Output {
        driver.write_byte(COMMAND_MEMORY_SCRATCHPAD_WRITE)?;
        driver.write_byte(self.scratchpad.triggers.low as _)?;
        driver.write_byte(self.scratchpad.triggers.high as _)?;
        driver.write_byte(self.scratchpad.configuration.resolution as _)?;
        Ok(())
    }
}

/// And command
#[derive(Clone, Copy, Debug, Default)]
pub struct And<T, U>(pub T, pub U);

// impl<T: Command<Output = V>, U: Command<Output = V>, V> Command for And<T, U> {
//     type Output = Result<()>;

//     fn execute(&self, one_wire: &mut OneWireDriver<impl Pin, impl DelayNs>) -> Self::Output {
//         one_wire.reset()?;
//         one_wire.run(self.0)?;
//         one_wire.run(self.1)?;
//         Ok(())
//     }
// }

// /// Sends a reset, followed with either a SKIP_ROM or MATCH_ROM (with an
// /// address), and then the supplied command This should be followed by any
// /// reading/writing, if needed by the command used.
// #[derive(Clone, Copy, Debug)]
// pub enum MatchOrSkip {
//     Match { address: Address },
//     Skip,
// }
// impl Command for MatchOrSkip {
//     type Output = Result<()>;
//     fn execute(&self, one_wire: &mut OneWireDriver<impl Pin, impl DelayNs>) -> Self::Output {
//         one_wire.reset()?;
//         match *self {
//             Self::Match { address } => {
//                 one_wire.run(Match { address })?;
//             }
//             Self::Skip => {
//                 one_wire.run(Skip)?;
//             }
//         }
//         Ok(())
//     }
// }
