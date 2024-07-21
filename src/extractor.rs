use pep508_rs::VersionOrUrl;
use requirements_txt::{RequirementEntry, RequirementsTxtRequirement};

#[derive(Debug, PartialEq)]
pub enum Dependency {
    /// Fully resolved dependency that has a name and a version properly parsed
    Resolved { name: String, version: String },
    /// Dependency that has a valid name, but no version (URL instead of version)
    Unresolved { name: String, url: String },
    /// Dependency without any valid data (Ex: only URL w/o name)
    Unknown { identifier: String },
}

impl Dependency {
    pub fn from_requirement_entry(entry: &RequirementEntry) -> Self {
        match &entry.requirement {
            RequirementsTxtRequirement::Named(value) => {
                match &value.version_or_url {
                    Some(v_or_url) => {
                        match v_or_url {
                            VersionOrUrl::VersionSpecifier(v) => Self::Resolved { name: value.name.to_string(), version: v[0].version().to_string() },
                            VersionOrUrl::Url(v) => Self::Unresolved { name: value.name.to_string(), url: v.to_string() }
                        }
                    }
                    None => Self::Unknown { identifier: value.name.to_string() }
                }
            }
            RequirementsTxtRequirement::Unnamed(v) => Self::Unknown { identifier: v.to_string() }
        }
    }
}