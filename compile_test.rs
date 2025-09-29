//! Script to compile test generated Rust files
//!
//! This script tests generated .rs files in two levels:
//! 1. Compilation success
//! 2. Test execution success (currently returns Error)

use clap::Parser;
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about = "Test compilation of generated Rust files")]
struct Args {
    /// Specific file to test (relative to tests/ directory)
    #[arg(short, long)]
    file: Option<String>,

    /// Test all files in generated/eggplant directory
    #[arg(short, long, default_value_t = false)]
    all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.all {
        test_all_files()
    } else if let Some(file) = args.file {
        test_single_file(&file)
    } else {
        println!("Please specify either --file <filename> or --all");
        Ok(())
    }
}

fn test_all_files() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Starting compilation test for all generated Rust files ===");

    let generated_dir = "generated/eggplant";
    let mut total_files = 0;
    let mut compile_success = 0;
    let mut compile_fail = 0;

    // Find all .rs files recursively
    for entry in WalkDir::new(generated_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            total_files += 1;
            let file_path = path.to_string_lossy();

            println!("\n--- Testing: {} ---", file_path);

            // Level 1: Compilation test
            match test_compilation(&file_path) {
                Ok(_) => {
                    println!("✅ Compilation SUCCESS: {}", file_path);
                    compile_success += 1;
                }
                Err(e) => {
                    println!("❌ Compilation FAILED: {}", file_path);
                    println!("   Error: {}", e);
                    compile_fail += 1;
                }
            }

            // Level 2: Test execution (currently returns Error)
            match test_execution(&file_path) {
                Ok(_) => {
                    println!("✅ Test execution SUCCESS: {}", file_path);
                }
                Err(e) => {
                    println!("⚠️  Test execution: {}", e);
                }
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Total files tested: {}", total_files);
    println!("Compilation success: {}", compile_success);
    println!("Compilation failed: {}", compile_fail);
    println!(
        "Compilation success rate: {:.1}%",
        (compile_success as f32 / total_files as f32) * 100.0
    );

    Ok(())
}

fn test_single_file(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing single file: {} ===", file_name);

    let _test_file_path = format!("tests/{}", file_name);
    let base_name = file_name.trim_end_matches(".egg");

    // Try different possible paths for the generated file
    let possible_paths = [
        format!("generated/eggplant/{}.rs", base_name),
        format!("generated/eggplant/tests/{}.rs", base_name),
    ];

    let generated_file_path = possible_paths
        .iter()
        .find(|path| Path::new(path).exists())
        .ok_or_else(|| {
            format!(
                "Generated file not found for: {}. Tried: {:?}",
                file_name, possible_paths
            )
        })?;

    println!("\n--- Testing: {} ---", generated_file_path);

    // Create temporary directory for compilation test
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Copy the generated file to temp directory
    let temp_file = temp_path.join(format!("{}.rs", base_name));
    fs::copy(generated_file_path, &temp_file)?;

    // Create Cargo.toml in temp directory
    let cargo_toml_content = format!(
        r#"[package]
name = "test_compile_{}"
version = "0.1.0"
edition = "2021"

[dependencies]
eggplant_transpiler = {{ path = "{}" }}
log = "0.4"
env_logger = "0.11"

[[bin]]
name = "test_binary"
path = "{}.rs"
"#,
        base_name,
        std::env::current_dir()?.to_string_lossy(),
        base_name
    );

    let cargo_toml_path = temp_path.join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content)?;

    // Test compilation
    let output = Command::new("cargo")
        .current_dir(temp_path)
        .args(["check", "--quiet"])
        .output()?;

    if output.status.success() {
        println!("✅ Compilation SUCCESS: {}", generated_file_path);
    } else {
        println!("❌ Compilation FAILED: {}", generated_file_path);
        println!("   Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Level 2: Test execution (currently returns Error)
    match test_execution(&generated_file_path) {
        Ok(_) => {
            println!("✅ Test execution SUCCESS: {}", generated_file_path);
        }
        Err(e) => {
            println!("⚠️  Test execution: {}", e);
        }
    }

    Ok(())
}

fn test_compilation(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary Cargo.toml for the test
    let dir = Path::new(file_path).parent().unwrap();
    let cargo_toml_content = format!(
        r#"[package]
name = "test_compile"
version = "0.1.0"
edition = "2021"

[dependencies]
eggplant = {{ path = "../../../../" }}
log = "0.4"
env_logger = "0.11"

[[bin]]
name = "test_binary"
path = "{}"
"#,
        file_path
    );

    let cargo_toml_path = dir.join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content)?;

    // Try to compile
    let output = Command::new("cargo")
        .current_dir(dir)
        .args(["check", "--quiet"])
        .output()?;

    // Clean up
    let _ = fs::remove_file(cargo_toml_path);

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}

fn test_execution(_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Currently returns Error as requested
    Err("Test execution not implemented yet".into())
}
