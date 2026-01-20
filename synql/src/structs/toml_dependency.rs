//! Submodule defining the `TomlDependency` struct.

/// Struct representing a TOML dependency.
#[derive(Debug, Clone)]
pub struct TomlDependency {
    /// Name of the dependency.
    name: String,
    /// Version requirements for the dependency.
    version: Option<String>,
    /// Git repository URL.
    git: Option<String>,
    /// Branch of the git repository.
    branch: Option<String>,
    /// Features to enable.
    features: Vec<String>,
    /// Whether the dependency is optional.
    optional: bool,
    /// Whether to use the workspace version.
    workspace: bool,
    /// Whether to use default features.
    default_features: Option<bool>,
    /// Path to the dependency.
    path: Option<String>,
}

impl TomlDependency {
    /// Creates a new `TomlDependency` instance.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
            git: None,
            branch: None,
            features: Vec::new(),
            optional: false,
            workspace: false,
            default_features: None,
            path: None,
        }
    }

    /// Sets the version requirements for the dependency.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Sets the git repository URL for the dependency.
    #[must_use]
    pub fn git(mut self, git: impl Into<String>) -> Self {
        self.git = Some(git.into());
        self
    }

    /// Sets the branch of the git repository for the dependency.
    #[must_use]
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// Adds a feature to the dependency.
    #[must_use]
    pub fn feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    /// Sets whether the dependency is optional.
    #[must_use]
    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    /// Sets whether to use the workspace version.
    #[must_use]
    pub fn workspace(mut self, workspace: bool) -> Self {
        self.workspace = workspace;
        self
    }

    /// Sets whether to use default features.
    #[must_use]
    pub fn default_features(mut self, default_features: bool) -> Self {
        self.default_features = Some(default_features);
        self
    }

    /// Sets the path to the dependency.
    #[must_use]
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl std::fmt::Display for TomlDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if self.workspace {
            write!(f, ".workspace = true")?;
            return Ok(());
        }

        write!(f, " = {{ ")?;

        let mut parts = Vec::new();

        if let Some(version) = &self.version {
            parts.push(format!("version = \"{version}\""));
        }

        if let Some(git) = &self.git {
            parts.push(format!("git = \"{git}\""));
        }

        if let Some(branch) = &self.branch {
            parts.push(format!("branch = \"{branch}\""));
        }

        if let Some(path) = &self.path {
            parts.push(format!("path = \"{path}\""));
        }

        if !self.features.is_empty() {
            let features_str = self
                .features
                .iter()
                .map(|feature| format!("\"{feature}\""))
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("features = [{features_str}]"));
        }

        if self.optional {
            parts.push("optional = true".to_string());
        }

        if let Some(default_features) = self.default_features {
            parts.push(format!("default-features = {default_features}"));
        }

        write!(f, "{}", parts.join(", "))?;
        write!(f, " }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_dependency() {
        let dep = TomlDependency::new("serde");
        assert_eq!(dep.to_string(), "serde = {  }");
    }

    #[test]
    fn test_version() {
        let dep = TomlDependency::new("serde").version("1.0");
        assert_eq!(dep.to_string(), "serde = { version = \"1.0\" }");
    }

    #[test]
    fn test_git_branch() {
        let dep = TomlDependency::new("geometric-traits")
            .git("https://github.com/earth-metabolome-initiative/geometric-traits")
            .branch("main");
        assert_eq!(
            dep.to_string(),
            "geometric-traits = { git = \"https://github.com/earth-metabolome-initiative/geometric-traits\", branch = \"main\" }"
        );
    }

    #[test]
    fn test_features() {
        let dep = TomlDependency::new("serde").version("1.0").feature("derive").feature("alloc");
        assert_eq!(
            dep.to_string(),
            "serde = { version = \"1.0\", features = [\"derive\", \"alloc\"] }"
        );
    }

    #[test]
    fn test_optional() {
        let dep = TomlDependency::new("serde").version("1.0").optional(true);
        assert_eq!(dep.to_string(), "serde = { version = \"1.0\", optional = true }");
    }

    #[test]
    fn test_workspace() {
        let dep = TomlDependency::new("serde").workspace(true);
        assert_eq!(dep.to_string(), "serde.workspace = true");
    }

    #[test]
    fn test_default_features() {
        let dep = TomlDependency::new("serde").version("1.0").default_features(false);
        assert_eq!(dep.to_string(), "serde = { version = \"1.0\", default-features = false }");
    }

    #[test]
    fn test_path() {
        let dep = TomlDependency::new("local-crate").path("../local");
        assert_eq!(dep.to_string(), "local-crate = { path = \"../local\" }");
    }

    #[test]
    fn test_complex_combination() {
        let dep = TomlDependency::new("complex")
            .version("2.0")
            .optional(true)
            .feature("feat1")
            .default_features(false);

        assert_eq!(
            dep.to_string(),
            "complex = { version = \"2.0\", features = [\"feat1\"], optional = true, default-features = false }"
        );
    }
}
