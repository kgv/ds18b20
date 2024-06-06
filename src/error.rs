use thiserror::Error;

/// Result
pub type Result<T, E> = core::result::Result<T, Error<E>>;

/// Error
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum Error<T> {
    #[error(transparent)]
    Pin(T),
    #[error(transparent)]
    Ds18b20(Ds18b20Error),
}

/// Error
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum Ds18b20Error {
    #[error("invalid configuration register (resolution)")]
    ConfigurationRegister,
    #[error("the bus was expected to be pulled high by a ~5K ohm pull-up resistor, but it wasn't")]
    NotHigh,
    #[error("family code mismatch")]
    MismatchedFamilyCode,
    #[error("there are no devices attached to the 1-Wire bus")]
    NoAttachedDevices,
    #[error("CRC mismatch {{ crc8={crc8} }}")]
    Crc { crc8: u8 },
    #[error("timeout expired")]
    Timeout,
}
