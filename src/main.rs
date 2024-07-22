mod cli;
mod service;
mod format;
mod models;
mod extractor;

use tokio::sync::Semaphore;

use clap::Parser;
use requirements_txt::{RequirementsTxt};
use uv_client::BaseClientBuilder;
use reqwest;

use service::interface::VulnerabilityService;
use crate::cli::ServiceSelector;
use crate::extractor::Dependency;
use crate::models::{Vulnerabilities, VulnerabilityReport};
use crate::service::{pypi};

use std::{
    fs::File,
    io::{self, Write},
    sync::Arc,
    process::{self},
};


#[tokio::main]
async fn main() {
    let args = cli::Config::parse();

    let service = match args.service {
        ServiceSelector::Osv => {
            eprintln!("OSV Service is not implemented");
            process::exit(1)
        }
        ServiceSelector::Pypi => pypi::PyPi
    };

    let mut output: Box<dyn Write> = match args.output {
        Some(file) => Box::new(File::create(file).unwrap()),
        _ => Box::new(io::stdout()),
    };

    let requirements = tokio::spawn(async move {
        RequirementsTxt::parse(
            &args.requirement,
            std::env::current_dir().expect("Error occurred while getting current directory"),
            &BaseClientBuilder::new())
            .await
            .expect("Error occurred while parsing provided requirements")
    })
        .await
        .unwrap();

    let mut dependencies = vec![];
    let client = reqwest::Client::new();
    let semaphore = Arc::new(Semaphore::new(args.connections));
    let mut jobs = vec![];

    for requirement in &requirements.requirements {
        dependencies.push(Dependency::from_requirement_entry(requirement));
    }

    for dependency in dependencies {
        let service_copy = service.clone();
        let client_copy = client.clone();
        let semaphore_copy = semaphore.clone();
        let job = tokio::spawn(
            async move { service_copy.query(dependency, client_copy, semaphore_copy).await }
        );
        jobs.push(job);
    }

    let mut reports = Vulnerabilities { vulnerabilities: vec![] };

    for job in jobs {
        let job_result = job.await;
        match job_result {
            Ok(report_result) => {
                match report_result {
                    Ok(report) => {
                        match report {
                            VulnerabilityReport::Vulnerable { vulnerabilities, .. } => {
                                reports.vulnerabilities.extend(vulnerabilities.vulnerabilities)
                            }
                            _ => ()
                        }
                    }
                    Err(e) => { eprintln!("Error `{}` occurred while checking vulnerability on remote server", e) }
                }
            }
            Err(e) => { eprintln!("Error `{}` occurred in async task", e) }
        }
    }

    let serialized = serde_json::to_string(&reports).expect("Cannot serialize generated report into `json`");

    writeln!(output, "{}", &serialized).unwrap();
}
