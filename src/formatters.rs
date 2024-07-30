use std::io::Write;

use cyclonedx_bom::models::component::Classification;
use cyclonedx_bom::models::dependency::{Dependencies, Dependency};
use cyclonedx_bom::models::vulnerability::{Vulnerabilities as BomVulnerabilities, Vulnerability as BomVulnerability};
use cyclonedx_bom::prelude::*;
use prettytable::{Table, Row, Cell};
use rand::Rng;
use uuid::Uuid;

use crate::models::{VulnerabilityReport};

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
    println!("\nVulnerable Dependencies ({} vulnerabilities):", vuln_table.len() - 1);
    vuln_table.printstd();
    println!("\nSkipped Dependencies ({} skipped):", skip_table.len() - 1);
    skip_table.printstd();
}

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

    writeln!(output, "{}", &serialized).expect("Failed to write generated JSON result");
}

pub fn format_cyclonedx(reports: &Vec<VulnerabilityReport>, mut output: Box<dyn Write>) {
    let mut rng = rand::thread_rng();
    let mut vulnerable_components = vec![];
    let mut dependencies = vec![];
    let mut bom_vulnerabilities = vec![];

    for report in reports {
        let bom_ref_part_1: u64 = rng.gen_range(1_000_000_000_000_000..10_000_000_000_000_000);
        let bom_ref_part_2: u64 = rng.gen_range(1_000_000_000_000_000..10_000_000_000_000_000);
        let bom_ref = format!("BomRef.{bom_ref_part_1}.{bom_ref_part_2}");
        match report {
            VulnerabilityReport::Vulnerable { name, version, vulnerabilities } => {
                vulnerable_components.push(Component::new(
                    Classification::Library,
                    name,
                    version,
                    Some(bom_ref.clone())));
                dependencies.push(Dependency { dependency_ref: bom_ref.clone(), dependencies: vec![] });

                for vuln in vulnerabilities {
                    bom_vulnerabilities.push(BomVulnerability {
                        bom_ref: Some(bom_ref.clone()),
                        id: Some(NormalizedString::new(&vuln.id)),
                        vulnerability_source: None,
                        vulnerability_references: None,
                        vulnerability_ratings: None,
                        cwes: None,
                        description: Some(vuln.details.clone()),
                        detail: None,
                        recommendation: Some("Upgrade".to_string()),
                        workaround: None,
                        proof_of_concept: None,
                        advisories: None,
                        created: None,
                        published: None,
                        updated: None,
                        rejected: None,
                        vulnerability_credits: None,
                        tools: None,
                        vulnerability_analysis: None,
                        vulnerability_targets: None,
                        properties: None,
                    })
                }
            }
            _ => ()
        }
    }

    let id = Uuid::new_v4();
    let bom = Bom {
        serial_number: Some(UrnUuid::new(format!("urn:uuid:{id}"))
            .expect("Failed to generate unique identifier for BOM report")),
        metadata: Some(Metadata {
            timestamp: Some(DateTime::now()
                .expect("Failed to generate timestamp for BOM report")),
            ..Metadata::default()
        }),
        components: Some(Components(vulnerable_components)),
        dependencies: Some(Dependencies(dependencies)),
        vulnerabilities: Some(BomVulnerabilities(bom_vulnerabilities)),
        ..Bom::default()
    };

    bom.output_as_json_v1_4(&mut output)
        .expect("Failed to write generated BOM report");
}
