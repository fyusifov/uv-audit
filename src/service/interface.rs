use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::models::VulnerabilityReport;
use crate::uv_cli::Dependency;
use anyhow::Result;


/// Interface that any Vulnerability service should implement.
pub trait VulnerabilityService {
    /// query Vulnerability Service for single dependency.
    // async fn query(&self, dependency: Dependency, client: reqwest::Client, semaphore: Arc<Semaphore>) -> Result<VulnerabilityReport>;

    fn query(&self, dependency: Dependency, client: reqwest::Client, semaphore: Arc<Semaphore>) -> impl std::future::Future<Output = Result<VulnerabilityReport>> + Send;

    /// returns timeout for connections made to Vulnerability service
    fn get_timeout(&self) -> u8;

    /// return amount of simultaneous connections
    fn get_connection_limit(&self) -> usize;
}
