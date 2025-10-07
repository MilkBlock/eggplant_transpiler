//! Eggplant code generation example
//!
//! This example demonstrates how to convert egglog programs to eggplant
//! and generate code in multiple target languages.

use clap::Parser;
use eggplant_transpiler::ast::parse::Parser as EggParser;
use eggplant_transpiler::eggplant::*;
use log::{debug, info, warn};
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about = "Generate eggplant Rust code from egglog files")]
struct Args {
    /// Specific .egg file to process
    #[arg(short, long)]
    file: Option<String>,

    /// Process all .egg files in examples folder
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Generate all files (equivalent to --all)
    #[arg(long, default_value_t = false)]
    generate_all: bool,

    /// Output directory for generated Rust files
    #[arg(short, long, default_value = "generated/eggplant")]
    output_dir: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    info!("=== Eggplant Code Generation Example ===");

    if args.all || args.generate_all {
        process_all_files(&args.output_dir)
    } else if let Some(file) = args.file {
        process_single_file(&file, &args.output_dir)
    } else {
        println!("Please specify either --file <filename> or --all or --generate-all");
        Ok(())
    }
}

fn process_all_files(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("1. Traversing all .egg files in examples folder...");
    let mut parser = EggParser::default();
    let mut rust_gen = EggplantCodeGenerator::new();
    let mut generated_files = Vec::new();

    for entry in walkdir::WalkDir::new("examples") {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "egg") {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            info!("=== Parsing {} ===", file_name);

            let content = fs::read_to_string(&path)?;
            let commands = parser.get_program_from_string(Some(file_name.clone()), &content)?;

            println!("DEBUG: Parsed {} egglog commands", commands.len());
            for (i, cmd) in commands.iter().enumerate() {
                println!("  Command {}: {:?}", i, cmd);
            }

            let eggplant_commands = convert_to_eggplant_with_source(
                &commands,
                Some(path.to_string_lossy().to_string()),
            );

            println!(
                "DEBUG: Generated {} eggplant commands",
                eggplant_commands.len()
            );
            for cmd in &eggplant_commands {
                match &cmd.command {
                    EggplantCommand::DslType(dsl_type) => {
                        println!(
                            "  - DSL Type: {} with {} variants",
                            dsl_type.name,
                            dsl_type.variants.len()
                        );
                    }
                    EggplantCommand::PatternVars(pattern_vars) => {
                        println!(
                            "  - PatternVars: {} with {} variables",
                            pattern_vars.name,
                            pattern_vars.variables.len()
                        );
                    }
                    EggplantCommand::Rule(rule) => {
                        println!("  - Rule: {}", rule.name);
                    }
                    _ => {
                        println!("  - Other command: {:?}", cmd.command);
                    }
                }
            }

            debug!(
                "Converted {} egglog commands to {} eggplant commands",
                commands.len(),
                eggplant_commands.len()
            );

            let rust_code = rust_gen.generate_rust(&eggplant_commands);
            debug!("Generated {} lines of Rust code", rust_code.lines().count());

            let relative_path = path.strip_prefix("examples").unwrap_or(&path);
            let output_file = format!(
                "{}/{}.rs",
                output_dir,
                relative_path.with_extension("").to_string_lossy()
            );
            if let Some(parent) = std::path::Path::new(&output_file).parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&output_file, &rust_code)?;

            // Format the generated Rust code
            let fmt_result = std::process::Command::new("cargo")
                .args(["fmt", "--", &output_file])
                .output();

            match fmt_result {
                Ok(output) if output.status.success() => {
                    debug!("Successfully formatted {}", output_file);
                }
                Ok(output) => {
                    warn!(
                        "Failed to format {}: {}",
                        output_file,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                Err(e) => {
                    warn!("Failed to run cargo fmt on {}: {}", output_file, e);
                }
            }

            generated_files.push(output_file);
        }
    }

    info!("Generated files saved to:");
    for file in &generated_files {
        info!("  - {}", file);
    }

    Ok(())
}

fn process_single_file(
    file_path: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("1. Processing single file: {}", file_path);

    let mut parser = EggParser::default();
    let mut rust_gen = EggplantCodeGenerator::new();

    // Check if file exists
    if !Path::new(file_path).exists() {
        // Try to find it in examples directory
        let examples_path = format!("examples/{}", file_path);
        if Path::new(&examples_path).exists() {
            return process_single_file(&examples_path, output_dir);
        } else {
            return Err(format!("File not found: {}", file_path).into());
        }
    }

    let content = fs::read_to_string(file_path)?;
    let file_name = Path::new(file_path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let commands = parser.get_program_from_string(Some(file_name.clone()), &content)?;
    let eggplant_commands = convert_to_eggplant_with_source(&commands, Some(file_path.to_string()));

    info!(
        "Converted {} egglog commands to {} eggplant commands",
        commands.len(),
        eggplant_commands.len()
    );

    let rust_code = rust_gen.generate_rust(&eggplant_commands);
    info!("Generated {} lines of Rust code", rust_code.lines().count());

    let output_file = format!(
        "{}/{}.rs",
        output_dir,
        Path::new(file_path)
            .with_extension("")
            .file_name()
            .unwrap()
            .to_string_lossy()
    );
    if let Some(parent) = std::path::Path::new(&output_file).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_file, &rust_code)?;

    info!("Generated file: {}", output_file);

    // Format the generated Rust code
    let fmt_result = std::process::Command::new("cargo")
        .args(["fmt", "--", &output_file])
        .output();

    match fmt_result {
        Ok(output) if output.status.success() => {
            info!("Successfully formatted {}", output_file);
        }
        Ok(output) => {
            warn!(
                "Failed to format {}: {}",
                output_file,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            warn!("Failed to run cargo fmt on {}: {}", output_file, e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_eggplant_example() {
        // Test that we can parse and convert egglog files
        let calc_content = fs::read_to_string("examples/calc.egg").unwrap();
        let mut parser = EggParser::default();
        let commands = parser
            .get_program_from_string(Some("calc.egg".to_string()), &calc_content)
            .unwrap();

        let eggplant_commands =
            convert_to_eggplant_with_source(&commands, Some("examples/calc.egg".to_string()));
        assert!(!eggplant_commands.is_empty());

        // Test code generation
        let mut codegen = EggplantCodeGenerator::new();
        let rust_code = codegen.generate_rust(&eggplant_commands);

        assert!(rust_code.contains("#[eggplant::dsl]"));
        assert!(rust_code.contains("enum"));
    }
}
