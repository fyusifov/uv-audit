use std::process::Command;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use anyhow::{Result, anyhow};
use indicatif::{ProgressBar, ProgressStyle};


#[derive(Debug, PartialEq)]
pub enum Dependency {
    Resolved { name: String, version: String },
    Unresolved { identifier: String },
}

pub enum UVArgs {
    Freeze,
    Compile { filename: String, all_extras: bool, index_url: Option<String> },
}

pub struct UV;

impl UV {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, args: UVArgs) -> Result<Vec<Dependency>> {
        let mut command = Command::new("uv");
        command.arg("pip");
        match args {
            UVArgs::Freeze => {
                command.arg("freeze");
            }
            UVArgs::Compile { filename, all_extras, index_url, .. } => {
                command.arg("compile").arg(filename);
                if all_extras {
                    command.arg("--all-extras");
                } else if index_url.is_some() {
                    command.arg("--index-url")
                        .arg(index_url.unwrap());
                }
            }
        }
        let pb = self.get_progress_bar();
        pb.set_message("Resolving Dependencies...");
        let output = command.output()?;
        pb.finish_with_message("Done");
        if output.status.success() {
            Ok(self.parse(&output.stdout))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Resolving dependencies failed: {:?}", error))
        }
    }

    pub fn read_requirements(&self, filename: String) -> Result<Vec<Dependency>> {
        let mut content = vec![];
        let mut file = File::open(filename)?;
        file.read_to_end(&mut content)?;
        Ok(self.parse(&content))
    }

    fn parse(&self, output: &[u8]) -> Vec<Dependency> {
        let stdout = String::from_utf8_lossy(output);
        let mut dependencies = vec![];
        for line in stdout.lines() {
            if !line.contains("#") {
                if line.contains("==") {
                    let name_version = line
                        .split("==")
                        .collect::<Vec<&str>>();
                    dependencies
                        .push(Dependency::Resolved {
                            name: name_version[0].to_string(),
                            version: name_version[1].to_string(),
                        })
                } else {
                    dependencies.push(Dependency::Unresolved { identifier: line.to_string() })
                }
            }
        }
        dependencies
    }

    fn get_progress_bar(&self) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.yellow} {msg}")
                .unwrap()
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        pb
    }
}