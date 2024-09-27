use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Semaphore;

use crate::uv_cli::Dependency;
use crate::models::VulnerabilityReport;
use crate::service::interface::VulnerabilityService;

pub struct Auditor<T: VulnerabilityService + Clone + Send + Sync + 'static> {
    service: T
}

impl<T: VulnerabilityService + Clone + Send + Sync + 'static> Auditor<T> {
    pub fn from_service(service: T) -> Self {
        Self { service }
    }

    pub async fn audit(&self, dependencies: Vec<Dependency>) -> Result<Vec<VulnerabilityReport>>{
        let client = reqwest::Client::new();
        let semaphore = Arc::new(Semaphore::new(self.service.get_connection_limit()));
        let mut jobs = vec![];

        for dependency in dependencies {
            let service_copy = self.service.clone();
            let client_copy = client.clone();
            let semaphore_copy = semaphore.clone();
            let job = tokio::spawn(
                async move { service_copy.query(dependency, client_copy, semaphore_copy).await }
            );
            jobs.push(job);
        }

        let mut reports = vec![];

        for job in jobs {
            let job_result = job.await;
            match job_result {
                Ok(report_result) => {
                    match report_result {
                        Ok(report) => { reports.push(report) }
                        Err(e) => { eprintln!("Error `{}` occurred while checking vulnerability on remote server", e) }
                    }
                }
                Err(e) => { eprintln!("Error `{}` occurred in async task", e) }
            }
        }
        Ok(reports)
    }
}