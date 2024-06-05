pub use self::{
    memory::{
        ConvertTemperature, CopyScratchpad, ReadPowerSupply, ReadScratchpad, RecallE2,
        WriteScratchpad,
    },
    rom::{MatchRom, ReadRom, SearchAlarm, SearchRom, SkipRom},
};

use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

mod memory;
mod rom;
