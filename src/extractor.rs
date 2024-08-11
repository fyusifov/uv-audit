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

#[tokio::test]
async fn test_extractor() {
    use std::{path, env};
    use uv_client::BaseClientBuilder;
    use requirements_txt::{RequirementsTxt};

    let requirements_file = path::Path::new("./test-data/requirements.txt");
    let parsed_requirements = RequirementsTxt::parse(
        requirements_file,
        env::current_dir().unwrap(),
        &BaseClientBuilder::new(),
    ).
        await.
        unwrap();

    let mut dependencies = vec![];

    for requirement in &parsed_requirements.requirements {
        dependencies.push(Dependency::from_requirement_entry(requirement));
    }

    assert_eq!(dependencies[0], Dependency::Resolved { name: "named-requirement-with-version".to_string(), version: "1.2.3".to_string() });
    assert_eq!(dependencies[1], Dependency::Unresolved { name: "named-requirement-without-version".to_string(), url: "git+https://github.com/somerepo.git".to_string() });
    assert_eq!(dependencies[2], Dependency::Unknown { identifier: "https://github.com/somerepo.git".to_string() });
}
