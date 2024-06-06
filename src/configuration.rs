/// Configuration
#[derive(Clone, Copy, Debug)]
pub struct Configuration {
    /// A;.
    pub a: u32,
    /// B;.
    pub b: u32,
    /// C;.
    pub c: u32,
    /// D;.
    pub d: u32,
    /// E;.
    pub e: u32,
    /// F;.
    pub f: u32,
    /// G;.
    pub g: u32,
    /// H; min: 480 μs, max: 960 μs.
    pub h: u32,
    /// I; min: 15 μs, max: 300 μs.
    pub i: u32,
    /// J; min: 480 - I μs.
    pub j: u32,
}

impl Configuration {
    pub fn overdrive() -> Self {
        Self {
            a: 1_000,
            b: 7_500,
            c: 7_500,
            d: 2_500,
            e: 1_000,
            f: 7_000,
            g: 2_500,
            h: 70_000,
            i: 8_500,
            j: 40_000,
        }
    }

    pub fn standard() -> Self {
        Self {
            a: 6_000,
            b: 64_000,
            c: 60_000,
            d: 10_000,
            e: 9_000,
            f: 55_000,
            g: 0_000,
            h: 480_000,
            i: 70_000,
            j: 410_000,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::standard()
    }
}

/// Speed
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Speed {
    #[default]
    Standard,
    Overdrive,
}

// mod standard {
//     pub(super) const A: u32 = 6;
//     pub(super) const B: u32 = 64;
//     pub(super) const C: u32 = 60;
//     pub(super) const D: u32 = 10;
//     pub(super) const E: u32 = 9;
//     pub(super) const F: u32 = 55;
//     pub(super) const G: u32 = 0;
//     pub(super) const H: u32 = 480;
//     pub(super) const I: u32 = 70;
//     pub(super) const J: u32 = 410;
// }

// mod overdrive {
//     pub(super) const A: f32 = 1.0;
//     pub(super) const B: f32 = 7.5;
//     pub(super) const C: f32 = 7.5;
//     pub(super) const D: f32 = 2.5;
//     pub(super) const E: f32 = 1.0;
//     pub(super) const F: f32 = 7.0;
//     pub(super) const G: f32 = 2.5;
//     pub(super) const H: f32 = 70.0;
//     pub(super) const I: f32 = 8.5;
//     pub(super) const J: f32 = 40.0;
// }
