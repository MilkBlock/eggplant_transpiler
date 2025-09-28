//! Benchmark example to test compilation of all generated files
//!
//! This example copies each generated .rs file to the test crate as main.rs
//! and attempts to compile it, reporting success/failure rates.

use std::fs;
use std::process::Command;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("=== Starting benchmark test for generated Rust files ===");

    let generated_dir = "generated/eggplant/tests/web-demo";
    let test_crate_dir = "test_crate";
    let test_main_path = format!("{}/src/main.rs", test_crate_dir);

    let mut total_files = 0;
    let mut compile_success = 0;
    let mut compile_fail = 0;
    let mut failed_files = Vec::new();

    // Find all .rs files recursively
    for entry in WalkDir::new(generated_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            total_files += 1;
            let file_path = path.to_string_lossy();

            println!("\n--- Testing: {} ---", file_path);

            // Copy the file to test crate as main.rs
            let content = fs::read_to_string(path)?;
            fs::write(&test_main_path, content)?;

            // Clean before testing
            let _ = Command::new("cargo")
                .current_dir(test_crate_dir)
                .args(["clean", "--quiet"])
                .output();

            // Level 1: Compilation test
            match test_compilation(test_crate_dir) {
                Ok(_) => {
                    println!("✅ Compilation SUCCESS: {}", file_path);
                    compile_success += 1;
                }
                Err(e) => {
                    println!("❌ Compilation FAILED: {}", file_path);
                    println!("   Error: {}", e);
                    compile_fail += 1;
                    failed_files.push(file_path.to_string());
                }
            }

            // Level 2: Test execution (currently returns Error as requested)
            match test_execution() {
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

    if !failed_files.is_empty() {
        println!("\nFailed files:");
        for file in failed_files {
            println!("  - {}", file);
        }
    }

    Ok(())
}

fn test_compilation(test_crate_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Try to compile the test crate
    let output = Command::new("cargo")
        .current_dir(test_crate_dir)
        .args(["check", "--quiet"])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Compilation failed: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

fn test_execution() -> Result<(), Box<dyn std::error::Error>> {
    // Currently returns Error as requested
    Err("Test execution not implemented yet".into())
}