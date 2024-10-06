use core::time::Duration;
use std::sync::Arc;

use anyhow::Result;
use reqwest::ClientBuilder;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use indicatif::{ProgressBar, ProgressStyle};

use crate::uv_cli::Dependency;
use crate::models::VulnerabilityReport;
use crate::service::interface::VulnerabilityService;

pub struct Auditor<T: VulnerabilityService + Clone + Send + Sync + 'static> {
    service: T,
}

impl<T: VulnerabilityService + Clone + Send + Sync + 'static> Auditor<T> {
    pub fn from_service(service: T) -> Self {
        Self { service }
    }

    pub async fn audit(&self, dependencies: Vec<Dependency>) -> Result<Vec<VulnerabilityReport>> {
        let duration = Duration::from_secs(self.service.get_timeout());
        let client = ClientBuilder::new().timeout(duration).build()?;
        let semaphore = Arc::new(Semaphore::new(self.service.get_connection_limit()));
        let mut jobs = vec![];
        let pb = self.get_progress_bar(dependencies.len() as u64);

        for dependency in dependencies {
            let pb_copy = pb.clone();
            let service_copy = self.service.clone();
            let client_copy = client.clone();
            let semaphore_copy = semaphore.clone();
            let job = tokio::spawn(
                async move {
                    let report = service_copy.query(dependency, client_copy, semaphore_copy).await;
                    pb_copy.inc(1);
                    report
                }
            );
            jobs.push(job);
        }
        pb.abandon_with_message("Done");
        Ok(self.render_result(jobs).await)
    }

    async fn render_result(&self, jobs: Vec<JoinHandle<Result<VulnerabilityReport>>>) -> Vec<VulnerabilityReport> {
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
        reports
    }

    fn get_progress_bar(&self, dependencies_length: u64) -> ProgressBar {
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-");
        let pb = ProgressBar::new(dependencies_length);
        pb.set_style(sty);
        pb
    }
}
