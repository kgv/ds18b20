use crate::{
    commands::{MatchRom, SkipRom},
    Rom,
};

/// Load config from EEPROM to scratchpad.
///
/// If `rom` is `None` - for all devices simultaneously.
#[derive(Clone, Copy, Debug, Default)]
pub struct Transaction {
    pub rom: Option<Rom>,
}

// impl Command for MemoryRecall {
//     type Output = Result<()>;

//     fn execute(&self, driver: &mut OneWireDriver<impl Pin, impl DelayNs>) -> Self::Output {
//         driver.initialization()?;
//         match self.rom {
//             Some(rom) => driver.run(MatchRom { rom })?,
//             None => driver.run(SkipRom)?,
//         }
//         driver.write_byte(COMMAND_MEMORY_RECALL)?;
//         // wait for the recall to finish (up to 10ms)
//         let max_retries = (10000 / READ_SLOT_DURATION_MICROS) + 1;
//         for _ in 0..max_retries {
//             if driver.read_bit()? == true {
//                 return Ok(());
//             }
//         }
//         Err(Error::Timeout)
//     }
// }
