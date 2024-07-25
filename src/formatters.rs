use std::io::Write;
use crate::models::{VulnerabilityReport};
use prettytable::{Table, Row, Cell};


pub fn format_json(reports: &Vec<VulnerabilityReport>, mut output: Box<dyn Write>) {
    let mut vulnerable_dependencies = vec![];
    for report in reports {
        match report {
            VulnerabilityReport::Vulnerable { .. } => { vulnerable_dependencies.push(report) }
            _ => ()
        }
    }

    let serialized = serde_json::to_string(&vulnerable_dependencies)
        .expect("Cannot serialize generated report into `json`");

    writeln!(output, "{}", &serialized).unwrap();
}

pub fn format_table(reports: &Vec<VulnerabilityReport>) {
    let mut vuln_table = Table::new();
    let mut skip_table = Table::new();

    vuln_table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Version"),
        Cell::new("ID"),
        Cell::new("Fixed Versions"),
    ]));

    skip_table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Skip Reason"),
    ]));

    for report in reports {
        match report {
            VulnerabilityReport::Vulnerable { name, version, vulnerabilities } => {
                for vuln in vulnerabilities {
                    vuln_table.add_row(Row::new(vec![
                        Cell::new(name),
                        Cell::new(version),
                        Cell::new(&vuln.id),
                        Cell::new(&vuln.fixed_in.join(", ")),
                    ]));
                }
            }
            VulnerabilityReport::NotVulnerable => {}
            VulnerabilityReport::Skipped { name, reason } => {
                skip_table.add_row(Row::new(vec![
                    Cell::new(name),
                    Cell::new(reason),
                ]));
            }
        }
    }
    println!("{}", "\nVulnerable Dependencies:");
    vuln_table.printstd();
    println!("{}", "\nSkipped Dependencies:");
    skip_table.printstd();
}
