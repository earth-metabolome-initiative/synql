//! Submodule implementing the method `serde` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `serde` crate.

use crate::structs::ExternalCrate;

impl ExternalCrate {
    /// Returns `ExternalCrate` instance describing the `serde`
    /// crate.
    #[must_use]
    pub fn serde() -> ExternalCrate {
        ExternalCrate::new("serde").unwrap().version("1.0").unwrap().features(["derive"]).into()
    }
}
