use std::{path, process};
use clap::{Parser, ValueEnum};


#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ServiceSelector {
    /// Python Package Index - pypi.org
    Osv,
    /// Open Source Vulnerabilities - api.osv.dev
    Pypi,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
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


/// This function is used to validate command-line interface (CLI) arguments.
/// For error printing, it avoids using third-party crates by utilizing custom
/// terminal coloring with ANSI escape sequences. The sequences used are as follows:
///
/// - `\x1b[1m`: Makes the text **bold**.
/// - `\x1b[91m`: Colors the text **red** to indicate an error or failure.
/// - `\x1b[93m`: Colors the text **yellow** to indicate a warning.
/// - `\x1b[0m`: Resets the text formatting to default.
///
/// Example usage:
///
/// ```rust
/// println!("\x1b[1mThis is bold text\x1b[0m");
/// println!("\x1b[91mThis is red text (error)\x1b[0m");
/// println!("\x1b[93mThis is yellow text (warning)\x1b[0m");
/// ```
///
/// This approach provides basic but effective text formatting without additional dependencies.
pub fn validate_config(config: &Config) {
    if config.format == OutputFormatSelector::Columns {
        if config.output.is_some() {
            eprintln!("\x1b[1m\x1b[91merror:\x1b[0m the argument \x1b[93m'--format'\x1b[0m with value \x1b[93m'columns'\x1b[0m cannot be used with \x1b[93m'--output <OUTPUT>'\x1b[0m");
            process::exit(1)
        }
    }
}
