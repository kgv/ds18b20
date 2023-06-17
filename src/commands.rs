// pub use memory::{
//     Convert, Load as MemoryLoad, PowerSupply, Read as MemoryRead, Save as MemorySave,
//     Write as MemoryWrite,
// };

use crate::{
    error::{Error, Result},
    scratchpad::Scratchpad,
};
use core::convert::Infallible;
use embedded_hal::{
    delay::DelayUs,
    digital::{ErrorType, InputPin, OutputPin},
};
use one_wire::{
    commands::{Match as RomMatch, Skip as RomSkip},
    Code, Command, Commander, OneWire,
};

pub const COMMAND_MEMORY_CONVERT: u8 = 0x44;
pub const COMMAND_MEMORY_RECALL: u8 = 0xB8;
pub const COMMAND_MEMORY_POWER_SUPPLY_READ: u8 = 0xB4;
pub const COMMAND_MEMORY_SCRATCHPAD_WRITE: u8 = 0x4E;
pub const COMMAND_MEMORY_SCRATCHPAD_READ: u8 = 0xBE;
pub const COMMAND_MEMORY_SCRATCHPAD_COPY: u8 = 0x48;

const READ_SLOT_DURATION_MICROS: u16 = 70;

/// Alias for `InputPin` + `OutputPin` + `ErrorType`.
pub trait Pin = InputPin + OutputPin + ErrorType<Error = Infallible>;

/// Initiates temperature conversion.
///
/// You should wait for the measurement to finish before reading the
/// measurement. The amount of time you need to wait depends on the current
/// resolution configuration
#[derive(Clone, Copy, Debug, Default)]
pub struct Convert {
    pub code: Option<Code>,
}

impl Command for Convert {
    type Output = Result<()>;

    fn execute(&self, one_wire: &mut OneWire<impl Pin, impl DelayUs>) -> Self::Output {
        one_wire.reset()?;
        match self.code {
            Some(code) => one_wire.run(RomMatch { code })?,
            None => one_wire.run(RomSkip)?,
        }
        one_wire.write_byte(COMMAND_MEMORY_CONVERT)?;
        Ok(())
    }
}

/// Signals the mode of DS18B20 power supply to the master.
#[derive(Clone, Copy, Debug)]
pub enum PowerSupply {
    /// Signals the mode of DS18B20 power supply to the master.
    Read,
}

/// Load config from EEPROM to scratchpad.
pub type Load = Recall;

/// Recalls values stored in nonvolatile memory (EEPROM) into scratchpad
/// (temperature triggers).
///
/// If `code` is `None` - for all devices simultaneously.
#[derive(Clone, Copy, Debug, Default)]
pub struct Recall {
    pub code: Option<Code>,
}

impl Command for Recall {
    type Output = Result<()>;

    fn execute(&self, one_wire: &mut OneWire<impl Pin, impl DelayUs>) -> Self::Output {
        one_wire.reset()?;
        match self.code {
            Some(code) => one_wire.run(RomMatch { code })?,
            None => one_wire.run(RomSkip)?,
        }
        one_wire.write_byte(COMMAND_MEMORY_RECALL)?;
        // wait for the recall to finish (up to 10ms)
        let max_retries = (10000 / READ_SLOT_DURATION_MICROS) + 1;
        for _ in 0..max_retries {
            if one_wire.read_bit()? == true {
                return Ok(());
            }
        }
        Err(Error::Timeout)
    }
}

// Save config from scratchpad to EEPROM.
pub type Save = Copy;

/// Copies scratchpad into nonvolatile memory (EEPROM) (addresses 2 through
/// 4 only).
///
/// If `code` is `None` - for all devices simultaneously.
#[derive(Clone, Copy, Debug, Default)]
pub struct Copy {
    pub code: Option<Code>,
}

impl Command for Copy {
    type Output = Result<()>;

    fn execute(&self, one_wire: &mut OneWire<impl Pin, impl DelayUs>) -> Self::Output {
        one_wire.reset()?;
        match self.code {
            Some(code) => one_wire.run(RomMatch { code })?,
            None => one_wire.run(RomSkip)?,
        }
        one_wire.write_byte(COMMAND_MEMORY_SCRATCHPAD_COPY)?;
        one_wire.wait(10000); // delay 10ms for the write to complete
        Ok(())
    }
}

/// Reads bytes from scratchpad and reads CRC byte.
#[derive(Clone, Copy, Debug)]
pub struct Read {
    pub code: Code,
}

impl Command for Read {
    type Output = Result<Scratchpad>;

    fn execute(&self, one_wire: &mut OneWire<impl Pin, impl DelayUs>) -> Self::Output {
        one_wire.reset()?;
        one_wire.run(RomMatch { code: self.code })?;
        one_wire.write_byte(COMMAND_MEMORY_SCRATCHPAD_READ)?;
        let mut bytes = [0; 9];
        one_wire.read_bytes(&mut bytes)?;
        bytes.try_into()
    }
}

/// Writes bytes into scratchpad at addresses 2 through 4 (TH and TL
/// temperature triggers and config).
#[derive(Clone, Copy, Debug, Default)]
pub struct Write {
    pub code: Option<Code>,
    pub scratchpad: Scratchpad,
}

impl Command for Write {
    type Output = Result<()>;

    fn execute(&self, one_wire: &mut OneWire<impl Pin, impl DelayUs>) -> Self::Output {
        one_wire.reset()?;
        match self.code {
            Some(code) => one_wire.run(RomMatch { code })?,
            None => one_wire.run(RomSkip)?,
        }
        one_wire.write_byte(COMMAND_MEMORY_SCRATCHPAD_WRITE)?;
        one_wire.write_byte(self.scratchpad.triggers.low as _)?;
        one_wire.write_byte(self.scratchpad.triggers.high as _)?;
        one_wire.write_byte(self.scratchpad.configuration.resolution as _)?;
        Ok(())
    }
}

/// And command
#[derive(Clone, Copy, Debug, Default)]
pub struct And<T, U>(pub T, pub U);

// impl<T: Command<Output = V>, U: Command<Output = V>, V> Command for And<T, U> {
//     type Output = Result<()>;

//     fn execute(&self, one_wire: &mut OneWire<impl Pin, impl DelayUs>) -> Self::Output {
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
//     fn execute(&self, one_wire: &mut OneWire<impl Pin, impl DelayUs>) -> Self::Output {
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
