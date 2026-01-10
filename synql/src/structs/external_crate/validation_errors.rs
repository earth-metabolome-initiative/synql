//! Submodule registering validation errors for external crates.

use crate::structs::ExternalCrate;

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `validation_errors` crate.
    #[must_use]
    pub fn validation_errors() -> ExternalCrate {
        ExternalCrate::new("validation-errors")
            .unwrap()
            .git("https://github.com/LucaCappelletti94/diesel-builders", "main")
            .into()
    }
}
