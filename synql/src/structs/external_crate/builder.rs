//! Submodule providing a builder for the `ExternalCrate` struct.

use std::fmt::Display;

use crate::{
    Error,
    structs::{ExternalCrate, ExternalFunction, ExternalType, TomlDependency},
};

/// Builder for the `ExternalCrate` struct.
pub struct ExternalCrateBuilder {
    dependency: TomlDependency,
    /// The types provided by the crate.
    types: Vec<ExternalType>,
    /// The functions provided by the crate.
    functions: Vec<ExternalFunction>,
}

impl ExternalCrateBuilder {
    /// Creates a new `ExternalCrateBuilder`.
    ///
    /// # Errors
    ///
    /// Returns `ExternalCrateBuilderError::InvalidName` if the name is empty or
    /// contains spaces.
    pub fn new(name: &str) -> Result<Self, ExternalCrateBuilderError> {
        if name.trim().is_empty() || name.contains(' ') {
            return Err(ExternalCrateBuilderError::InvalidName);
        }
        Ok(Self { dependency: TomlDependency::new(name), types: Vec::new(), functions: Vec::new() })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of a
/// `ExternalCrate`.
pub enum ExternalCrateBuilderError {
    /// The name of the crate is invalid.
    InvalidName,
    /// A type handling the same postgres type has already been added to the
    /// crate.
    DuplicatedPostgresType,
    /// A macro with the same name has already been added to the crate.
    DuplicatedMacro,
    /// A trait with the same name has already been added to the crate.
    DuplicatedTrait,
}

impl Display for ExternalCrateBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalCrateBuilderError::InvalidName => write!(f, "Invalid crate name"),
            ExternalCrateBuilderError::DuplicatedPostgresType => {
                write!(
                    f,
                    "A type handling the same postgres type has already been added to the crate"
                )
            }
            ExternalCrateBuilderError::DuplicatedMacro => {
                write!(f, "A macro with the same name has already been added to the crate")
            }
            ExternalCrateBuilderError::DuplicatedTrait => {
                write!(f, "A trait with the same name has already been added to the crate")
            }
        }
    }
}

impl ExternalCrateBuilder {
    /// Adds a type provided by the crate.
    ///
    /// # Arguments
    /// * `required_type` - The type provided by the crate.
    ///
    /// # Errors
    ///
    /// Returns `ExternalCrateBuilderError::DuplicatedPostgresType` if a type
    /// with the same postgres type is already added.
    pub fn add_type(
        mut self,
        required_type: ExternalType,
    ) -> Result<Self, ExternalCrateBuilderError> {
        for postgres_type in required_type.postgres_types() {
            if self.types.iter().any(|t| t.is_compatible_with(postgres_type)) {
                return Err(ExternalCrateBuilderError::DuplicatedPostgresType);
            }
        }
        self.types.push(required_type);
        Ok(self)
    }

    /// Adds the provided types to the crate.
    ///
    /// # Arguments
    /// * `required_types` - The types provided by the crate.
    ///
    /// # Errors
    ///
    /// Returns `ExternalCrateBuilderError::DuplicatedPostgresType` if a type
    /// with the same postgres type is already added.
    pub fn types<I>(mut self, required_types: I) -> Result<Self, ExternalCrateBuilderError>
    where
        I: IntoIterator<Item = ExternalType>,
    {
        for required_type in required_types {
            self = self.add_type(required_type)?;
        }
        Ok(self)
    }

    /// Sets whether the crate is a dependency.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidTomlDependency` if the dependency is a workspace
    /// dependency.
    pub fn version<S: ToString + ?Sized>(mut self, version: &S) -> Result<Self, Error> {
        self.dependency = self.dependency.version(version.to_string())?;
        Ok(self)
    }

    /// Sets the git to the crate, if it is a local dependency.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidTomlDependency` if the dependency is a workspace
    /// dependency.
    pub fn git<S: ToString + ?Sized>(mut self, repository: &S, branch: &S) -> Result<Self, Error> {
        self.dependency = self.dependency.git(repository.to_string(), Some(branch.to_string()))?;
        Ok(self)
    }

    /// Adds a feature to the crate.
    ///
    /// # Arguments
    /// * `feature` - The feature to add.
    #[must_use]
    pub fn feature<S: ToString + ?Sized>(mut self, feature: &S) -> Self {
        self.dependency = self.dependency.feature(feature.to_string());
        self
    }

    /// Adds several features required by the crate.
    ///
    /// # Arguments
    /// * `features` - The features to add.
    #[must_use]
    pub fn features<I, S>(mut self, features: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        for feature in features {
            self = self.feature(&feature);
        }
        self
    }

    /// Adds a function provided by the crate.
    ///
    /// # Arguments
    ///
    /// * `method` - The method signature of the function.
    /// * `path` - The path to the function.
    #[must_use]
    pub fn function(mut self, function: ExternalFunction) -> Self {
        self.functions.push(function);
        self
    }

    /// Adds several functions provided by the crate.
    ///
    /// # Arguments
    /// * `functions` - The functions to add.
    #[must_use]
    pub fn functions<I>(mut self, functions: I) -> Self
    where
        I: IntoIterator<Item = ExternalFunction>,
    {
        for function in functions {
            self = self.function(function);
        }
        self
    }
}

impl From<ExternalCrateBuilder> for ExternalCrate {
    fn from(value: ExternalCrateBuilder) -> Self {
        ExternalCrate {
            dependency: value.dependency,
            types: value.types,
            functions: value.functions,
        }
    }
}
