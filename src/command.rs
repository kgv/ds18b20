use crate::Ds18b20Driver;
use core::convert::Infallible;
use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin},
};

/// Commander
pub trait Commander {
    fn run<C: Command>(&mut self, command: C) -> C::Output;
}

impl<T: InputPin + OutputPin + ErrorType<Error = Infallible>, U: DelayNs> Commander
    for Ds18b20Driver<T, U>
{
    fn run<C: Command>(&mut self, command: C) -> C::Output {
        command.execute(self)
    }
}

/// Command
pub trait Command {
    type Output;

    fn execute(
        &self,
        driver: &mut Ds18b20Driver<
            impl InputPin + OutputPin + ErrorType<Error = Infallible>,
            impl DelayNs,
        >,
    ) -> Self::Output;
}
