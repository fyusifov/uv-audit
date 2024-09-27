mod cli;
mod service;
mod formatters;
mod models;
mod auditor;
mod uv_cli;

use std::{fs::File, io::{self, Write}, process::{self}};

use clap::Parser;

use crate::auditor::Auditor;
use crate::cli::{OutputFormatSelector, ServiceSelector};
use crate::service::pypi::PyPi;
use crate::formatters::{format_table, format_cyclonedx, format_json};
use crate::uv_cli::{UVArgs, UV};


#[tokio::main]
async fn main() {
    let args = cli::Config::parse();
    cli::validate_config(&args);

    let uv = UV::new();
    let dependencies = {
        if args.requirement.is_some() {
            uv.run(UVArgs::Compile { filename: args.requirement.unwrap(), all_extras: false, index_url: args.index_url, extra_index_url: args.extra_index_url })
        } else if args.project_path.is_some() {
            uv.run(UVArgs::Compile { filename: args.project_path.unwrap(), all_extras: true, index_url: args.index_url, extra_index_url: args.extra_index_url })
        } else {
            uv.run(UVArgs::Freeze)
        }
    };

    let service = match args.service {
        ServiceSelector::Osv => {
            eprintln!("OSV Service is not implemented");
            process::exit(1)
        }
        ServiceSelector::Pypi => PyPi::new(args.timeout, args.connections)
    };

    let output: Box<dyn Write> = match args.output {
        Some(file) => Box::new(File::create(file).unwrap()),
        _ => Box::new(io::stdout()),
    };

    let auditor = Auditor::from_service(service);
    let reports = auditor.audit(dependencies.unwrap()).await.unwrap();

    match args.format {
        OutputFormatSelector::Columns => format_table(&reports),
        OutputFormatSelector::Json => format_json(&reports, output),
        OutputFormatSelector::CyclonedxJson => format_cyclonedx(&reports, output)
    }
}
