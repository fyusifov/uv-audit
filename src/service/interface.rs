use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::models::VulnerabilityReport;
use crate::extractor::Dependency;
use anyhow::Result;


/// Interface that any Vulnerability service should implement.
pub trait VulnerabilityService {
    /// query Vulnerability Service for single dependency.
    async fn query(&self, dependency: Dependency, client: reqwest::Client, semaphore: Arc<Semaphore>) -> Result<VulnerabilityReport>;

    // Bulk query of Vulnerability service.
    // Default implementation just calls `query` method by iterating
    // through dependencies
}
