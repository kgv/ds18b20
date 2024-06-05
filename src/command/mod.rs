pub use self::{
    memory::{
        ConvertTemperature, CopyScratchpad, ReadPowerSupply, ReadScratchpad, RecallE2,
        WriteScratchpad,
    },
    rom::{MatchRom, ReadRom, SearchAlarm, SearchRom, SkipRom},
};

use crate::{error::Error, Driver};
use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin},
};

/// Ds18b20 command
pub trait Command {
    type Output;

    fn execute(
        &self,
        driver: &mut Driver<impl InputPin + OutputPin + ErrorType<Error = Error>, impl DelayNs>,
    ) -> Self::Output;
}

mod memory;
mod rom;
