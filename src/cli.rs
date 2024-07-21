use std::path;
use clap::{Parser, ValueEnum};


#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ServiceSelector {
    /// Python Package Index - pypi.org
    Osv,
    /// Open Source Vulnerabilities - api.osv.dev
    Pypi,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum OutputFormatSelector {
    /// Format output as a table
    Columns,
    /// Format output as a json
    Json,
    /// Format output as CycloneDX json
    CyclonedxJson
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, help("Audit the given requirements file"))]
    pub requirement: path::PathBuf,

    #[arg(
        short,
        long,
        default_value("pypi"),
        help("Vulnerability service to audit dependencies against")
    )]
    pub service: ServiceSelector,

    #[arg(
        short,
        long,
        default_value("columns"),
        help("Format to emit audit results in")
    )]
    pub format: OutputFormatSelector,

    #[arg(
        short,
        long,
        default_value("5"),
        help("Set the socket timeout")
    )]
    pub timeout: u8,

    #[arg(short, long, default_value("20"), help("Set the number of concurrent connections"))]
    pub connections: usize,

    #[arg(short, long, help("Output results to the given file [default: stdout]"))]
    pub output: Option<path::PathBuf>,
}
