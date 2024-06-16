use crate::{Driver, Ds18b20Error, Error};
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
        self.wait_for_high()?;
        self.set_low()?;
        self.delay(self.configuration.h);
        self.set_high()?;
        self.delay(self.configuration.i);
        let presence = self.is_low()?;
        self.delay(self.configuration.j);
        Ok(presence)
    }

    /// wait up to 255 µs for the bus to become high (from the pull-up resistor)
    fn wait_for_high(&mut self) -> Result<(), Error<T::Error>> {
        // wait up to 250 µs for the bus to become high (from the pull-up resistor)
        for _ in 0..125 {
            if self.is_high()? {
                return Ok(());
            }
            self.delay.delay_us(2);
        }
        Err(Ds18b20Error::BusNotHigh)?
    }
}

pub(crate) mod memory;
pub(crate) mod rom;
