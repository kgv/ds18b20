use crate::{
    error::{Ds18b20Error, Error},
    scratchpad::Scratchpad,
    Driver,
};
use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin},
};

pub const COMMAND_MEMORY_CONVERT: u8 = 0x44;
pub const COMMAND_MEMORY_RECALL: u8 = 0xB8;
pub const COMMAND_MEMORY_POWER_SUPPLY_READ: u8 = 0xB4;
pub const COMMAND_MEMORY_SCRATCHPAD_COPY: u8 = 0x48;
pub const COMMAND_MEMORY_SCRATCHPAD_READ: u8 = 0xBE;
pub const COMMAND_MEMORY_SCRATCHPAD_WRITE: u8 = 0x4E;

const READ_SLOT_DURATION_MICROS: u16 = 70;

/// Memory commands
pub trait MemoryCommands<T: ErrorType> {
    /// This command begins a temperature conversion. No further data is
    /// required. The temperature conversion will be performed and then the
    /// DS18B20 will remain idle. If the bus master issues read time slots
    /// following this command, the DS18B20 will output 0 on the bus as long as
    /// it is busy making a temperature conversion; it will return a 1 when the
    /// temperature conversion is complete. If parasite-powered, the bus master
    /// has to enable a strong pullup for a period greater than tconv
    /// immediately after issuing this command.
    ///
    /// You should wait for the measurement to finish before reading the
    /// measurement. The amount of time you need to wait depends on the current
    /// resolution configuration
    fn convert_temperature(&mut self) -> Result<(), Error<T::Error>>;

    /// Signals the mode of DS18B20 power supply to the master.
    fn read_power_supply(&self) -> Result<(), Error<T::Error>>;

    /// Recalls values stored in nonvolatile memory (EEPROM, electrically
    /// erasable programmable read-only memory) into scratchpad (temperature
    /// triggers). Load config from EEPROM to scratchpad.
    fn recall_eeprom(&mut self) -> Result<(), Error<T::Error>>;

    /// Copies scratchpad into nonvolatile memory (EEPROM) (addresses 2 through
    /// 4 only). Save config from scratchpad to EEPROM.
    fn copy_scratchpad(&mut self) -> Result<(), Error<T::Error>>;

    /// Reads bytes from scratchpad and reads CRC byte.
    fn read_scratchpad(&mut self) -> Result<Scratchpad, Error<T::Error>>;

    /// Writes bytes into scratchpad at addresses 2 through 4 (TH and TL
    /// temperature triggers and config).
    fn write_scratchpad(&mut self, scratchpad: Scratchpad) -> Result<(), Error<T::Error>>;
}

impl<T: InputPin + OutputPin + ErrorType, U: DelayNs> MemoryCommands<T> for Driver<T, U> {
    fn convert_temperature(&mut self) -> Result<(), Error<T::Error>> {
        self.write_byte(COMMAND_MEMORY_CONVERT)?;
        Ok(())
    }

    fn read_power_supply(&self) -> Result<(), Error<T::Error>> {
        Ok(())
    }

    fn recall_eeprom(&mut self) -> Result<(), Error<T::Error>> {
        self.write_byte(COMMAND_MEMORY_RECALL)?;
        // wait for the recall to finish (up to 10ms)
        let max_retries = (10000 / READ_SLOT_DURATION_MICROS) + 1;
        for _ in 0..max_retries {
            if self.read_bit()? == true {
                return Ok(());
            }
        }
        Err(Ds18b20Error::Timeout)?
    }

    fn copy_scratchpad(&mut self) -> Result<(), Error<T::Error>> {
        self.write_byte(COMMAND_MEMORY_SCRATCHPAD_COPY)?;
        self.delay(10000); // delay 10ms for the write to complete
        Ok(())
    }

    fn read_scratchpad(&mut self) -> Result<Scratchpad, Error<T::Error>> {
        self.write_byte(COMMAND_MEMORY_SCRATCHPAD_READ)?;
        let mut bytes = [0; 9];
        self.read_bytes(&mut bytes)?;
        Ok(bytes.try_into()?)
    }

    fn write_scratchpad(&mut self, scratchpad: Scratchpad) -> Result<(), Error<T::Error>> {
        self.write_byte(COMMAND_MEMORY_SCRATCHPAD_WRITE)?;
        self.write_byte(scratchpad.triggers.high as _)?;
        self.write_byte(scratchpad.triggers.low as _)?;
        self.write_byte(scratchpad.configuration_register.into())?;
        Ok(())
    }
}

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
