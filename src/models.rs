use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Vulnerability {
    pub id: String,
    aliases: Vec<String>,
    pub fixed_in: Vec<String>,
    #[serde(rename(serialize = "description"))]
    details: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vulnerabilities {
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum VulnerabilityReport {
    Vulnerable { name: String, version: String, vulnerabilities: Vec<Vulnerability> },
    NotVulnerable,
    Skipped { name: String, reason: String },
}
