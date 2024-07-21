use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Vulnerability {
    #[serde(rename(serialize = "description"))]
    pub details: String,
    pub fixed_in: Vec<String>,
    pub id: String,
    #[serde(default = "get_recommendation")]
    recommendation: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vulnerabilities {
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Deserialize)]
pub enum VulnerabilityReport {
    Vulnerable { name: String, version: String, vulnerabilities: Vulnerabilities },
    NotVulnerable,
    Skipped { name: String, reason: String },
}

fn get_recommendation() -> String {
    "Upgrade".to_string()
}