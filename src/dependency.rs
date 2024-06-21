use crate::ARGS;

/// Used to hold the information about the Casper dependencies which will be required by the
/// generated Cargo.toml files.
#[derive(Debug)]
pub struct Dependency {
    name: String,
    version: String,
}

impl Dependency {
    #[must_use]
    pub fn new(name: &str, version: &str) -> Self {
        Dependency {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    pub fn display_with_features(&self, default_features: bool, features: &[&str]) -> String {
        let version = if ARGS.casper_overrides().is_some() {
            "*"
        } else {
            &self.version
        };

        if default_features && features.is_empty() {
            return format!("{} = \"{version}\"\n", self.name);
        }

        let mut output = format!(r#"{} = {{ version = "{version}""#, self.name);

        if !default_features {
            output = format!("{output}, default-features = false");
        }

        if !features.is_empty() {
            output = format!("{output}, features = {features:?}");
        }

        format!("{output} }}\n")
    }

    #[cfg(test)]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[cfg(test)]
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}
