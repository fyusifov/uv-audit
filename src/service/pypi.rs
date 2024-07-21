use std::sync::Arc;

use reqwest;
use anyhow::Result;
use tokio::sync::Semaphore;
use crate::models::{VulnerabilityReport, Vulnerabilities};
use crate::extractor::Dependency;
use crate::service::interface::VulnerabilityService;


const BASE_URL: &str = "https://pypi.org/pypi";

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PyPi;

impl VulnerabilityService for PyPi {
    async fn query(&self, dependency: Dependency, client: reqwest::Client, semaphore: Arc<Semaphore>) -> Result<VulnerabilityReport>
    {
        match dependency {
            Dependency::Resolved { name, version } => {
                let url = format!("{}/{}/{}/json", BASE_URL, name, version);
                let permit = semaphore.acquire().await?;
                let response = client.get(url)
                    .send()
                    .await?;
                drop(permit);
                if response.status().as_u16() == 200 {
                    let parsed_response = response.json::<Vulnerabilities>().await?;
                    if parsed_response.vulnerabilities.len() > 0 {
                        Ok(VulnerabilityReport::Vulnerable { name, version, vulnerabilities: parsed_response })
                    } else {
                        Ok(VulnerabilityReport::NotVulnerable)
                    }
                } else {
                    Ok(VulnerabilityReport::Skipped { name, reason: "Dependency was not found on PyPi and cannot be audited".to_string() })
                }
            }
            Dependency::Unresolved { name, .. } => Ok(VulnerabilityReport::Skipped { name, reason: "Dependency is missing version specifier and cannot be audited".to_string() }),
            Dependency::Unknown { identifier } => Ok(VulnerabilityReport::Skipped { name: identifier, reason: "Unknown dependency cannot be audited".to_string() })
        }
    }
}
