/// Configuration
#[derive(Clone, Copy, Debug)]
pub struct Configuration {
    /// H; min: 480 μs, max: 960 μs.
    h: u32,
    /// I; min: 15 μs, max: 300 μs.
    i: u32,
    /// J; min: 480 - I μs.
    j: u32,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            h: 480,
            i: 70,
            j: 410,
        }
    }
}
