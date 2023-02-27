use crate::{Vector, DELTA_TIME};

/// Number of substeps used in XPBD simulation
pub struct NumSubsteps(pub u32);

impl Default for NumSubsteps {
    fn default() -> Self {
        Self(8)
    }
}

/// Number of iterations used in XPBD position solving
pub struct NumPosIters(pub u32);

impl Default for NumPosIters {
    fn default() -> Self {
        Self(4)
    }
}

/// Substep delta time
pub(crate) struct SubDeltaTime(pub f32);

impl Default for SubDeltaTime {
    fn default() -> Self {
        Self(DELTA_TIME / NumSubsteps::default().0 as f32)
    }
}

#[derive(Debug)]
pub struct Gravity(pub Vector);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vector::Y * -9.81)
    }
}
