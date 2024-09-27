use std::sync::Arc;

use reqwest;
use anyhow::Result;
use tokio::sync::Semaphore;
use crate::models::{VulnerabilityReport, Vulnerabilities};
use crate::uv_cli::Dependency;
use crate::service::interface::VulnerabilityService;


const BASE_URL: &str = "https://pypi.org/pypi";

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PyPi {
    timeout: u8,
    connections: usize,
}

impl PyPi {
    pub fn new(timeout: u8, connections: usize) -> Self {
        Self { timeout, connections }
    }
}

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
                        Ok(VulnerabilityReport::Vulnerable { name, version, vulnerabilities: parsed_response.vulnerabilities })
                    } else {
                        Ok(VulnerabilityReport::NotVulnerable)
                    }
                } else {
                    Ok(VulnerabilityReport::Skipped { name, reason: "Dependency was not found on PyPi and cannot be audited".to_string() })
                }
            }
            Dependency::Unresolved { identifier } => Ok(VulnerabilityReport::Skipped { name: identifier, reason: "Unresolved dependency cannot be audited".to_string() })
        }
    }

    fn get_timeout(&self) -> u8 {
        self.timeout
    }

    fn get_connection_limit(&self) -> usize {
        self.connections
    }
}
