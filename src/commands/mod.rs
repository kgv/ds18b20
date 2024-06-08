use crate::{Driver, Error};
use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin},
};

impl<T: InputPin + OutputPin + ErrorType, U: DelayNs> Driver<T, U> {
    /// Initialization.
    ///
    /// All transactions on the 1-Wire bus begin with an initialization
    /// sequence. The initialization sequence consists of a reset pulse
    /// transmitted by the bus master followed by presence pulse(s) transmitted
    /// by the slave(s). The presence pulse lets the bus master know that the
    /// DS18B20 is on the bus and is ready to operate.
    pub fn initialization(&mut self) -> Result<bool, Error<T::Error>> {
        self.set_low()?;
        self.delay(self.configuration.h);
        self.set_high()?;
        self.delay(self.configuration.i);
        let presence = self.is_low()?;
        self.delay(self.configuration.j);
        Ok(presence)
    }
}

pub(crate) mod memory;
pub(crate) mod rom;
