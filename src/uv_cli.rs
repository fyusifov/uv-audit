use std::process::{Command, Output};

use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq)]
pub enum Dependency {
    Resolved { name: String, version: String },
    Unresolved { identifier: String },
}

pub enum UVArgs {
    Freeze,
    Compile { filename: String, all_extras: bool, index_url: Option<String>, extra_index_url: Option<Vec<String>> },
}

pub struct UV;

impl UV {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self, args: UVArgs) -> Result<Vec<Dependency>> {
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
        let output = command.output()?;
        if output.status.success() {
            Ok(self.parse(&output))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Resolving dependencies failed: {:?}", error))
        }
    }

    fn parse(self, output: &Output) -> Vec<Dependency> {
        let stdout = String::from_utf8_lossy(&output.stdout);
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
}