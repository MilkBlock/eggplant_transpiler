//! Script to compile test all generated Rust files
//!
//! This script tests all generated .rs files in two levels:
//! 1. Compilation success
//! 2. Test execution success (currently returns Error)

use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Starting compilation test for generated Rust files ===");

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
    println!("Compilation success rate: {:.1}%", (compile_success as f32 / total_files as f32) * 100.0);

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
eggplant = {{ path = "../../../../eggplant" }}
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
        Err(format!("Compilation failed: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

fn test_execution(_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Currently returns Error as requested
    Err("Test execution not implemented yet".into())
}