use core::convert::Infallible;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Error
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error("invalid configuration register (resolution)")]
    ConfigurationRegister,
    #[error("the bus was expected to be pulled high by a ~5K ohm pull-up resistor, but it wasn't")]
    NotHigh,
    #[error("pin")]
    Pin(#[from] Infallible),
    #[error("an unexpected response was received from a command")]
    OneWire(#[from] one_wire::Error),
    #[error("family code mismatch")]
    MismatchedFamilyCode,
    #[error("timeout expired")]
    Timeout,
}
