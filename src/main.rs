mod cli;
mod service;
mod formatters;
mod models;
mod extractor;

use std::{fs::File, io::{self, Write}, sync::Arc, process::{self}};

use tokio::sync::Semaphore;
use clap::Parser;
use requirements_txt::{RequirementsTxt};
use uv_client::BaseClientBuilder;
use reqwest;

use crate::service::interface::VulnerabilityService;
use crate::cli::{OutputFormatSelector, ServiceSelector};
use crate::extractor::Dependency;
use crate::service::{pypi};
use crate::formatters::{format_cyclonedx, format_json, format_table};


#[tokio::main]
async fn main() {
    let args = cli::Config::parse();

    cli::validate_config(&args);

    let service = match args.service {
        ServiceSelector::Osv => {
            eprintln!("OSV Service is not implemented");
            process::exit(1)
        }
        ServiceSelector::Pypi => pypi::PyPi
    };

    let output: Box<dyn Write> = match args.output {
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

    match args.format {
        OutputFormatSelector::Columns => format_table(&reports),
        OutputFormatSelector::Json => format_json(&reports, output),
        OutputFormatSelector::CyclonedxJson => format_cyclonedx(&reports, output)
    }
}
