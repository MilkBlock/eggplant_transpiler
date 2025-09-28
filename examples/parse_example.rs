use eggplant_transpiler::ast::parse::Parser;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse calc.egg
    println!("=== Parsing calc.egg ===");
    let calc_content = fs::read_to_string("examples/calc.egg")?;
    let mut parser = Parser::default();
    let calc_commands =
        parser.get_program_from_string(Some("calc.egg".to_string()), &calc_content)?;
    println!(
        "Successfully parsed {} commands from calc.egg",
        calc_commands.len()
    );

    // Parse bool.egg
    println!("\n=== Parsing bool.egg ===");
    let bool_content = fs::read_to_string("examples/bool.egg")?;
    let bool_commands =
        parser.get_program_from_string(Some("bool.egg".to_string()), &bool_content)?;
    println!(
        "Successfully parsed {} commands from bool.egg",
        bool_commands.len()
    );

    // Parse math.egg
    println!("\n=== Parsing math.egg ===");
    let math_content = fs::read_to_string("examples/math.egg")?;
    let math_commands =
        parser.get_program_from_string(Some("math.egg".to_string()), &math_content)?;
    println!(
        "Successfully parsed {} commands from math.egg",
        math_commands.len()
    );

    // Generate Rust code for each example
    use eggplant_transpiler::codegen::RustCodeGenerator;

    println!("\n=== Generating Rust code for calc.egg ===");
    let mut codegen = RustCodeGenerator::new();
    let calc_rust = codegen.generate(&calc_commands);
    println!("Generated {} lines of Rust code", calc_rust.lines().count());

    println!("\n=== Generating Rust code for bool.egg ===");
    let bool_rust = codegen.generate(&bool_commands);
    println!("Generated {} lines of Rust code", bool_rust.lines().count());

    println!("\n=== Generating Rust code for math.egg ===");
    let math_rust = codegen.generate(&math_commands);
    println!("Generated {} lines of Rust code", math_rust.lines().count());

    // Save generated code to files
    fs::write("generated/calc_generated.rs", &calc_rust)?;
    fs::write("generated/bool_generated.rs", &bool_rust)?;
    fs::write("generated/math_generated.rs", &math_rust)?;

    println!("\nGenerated Rust code saved to:");
    println!("  - generated/calc_generated.rs");
    println!("  - generated/bool_generated.rs");
    println!("  - generated/math_generated.rs");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use eggplant_transpiler::ast::parse::Parser;

    #[test]
    fn test_parse_calc_egg() {
        let content = fs::read_to_string("examples/calc.egg").unwrap();
        let mut parser = Parser::default();
        let commands = parser
            .get_program_from_string(Some("calc.egg".to_string()), &content)
            .unwrap();
        assert!(commands.len() > 0);
    }

    #[test]
    fn test_parse_bool_egg() {
        let content = fs::read_to_string("examples/bool.egg").unwrap();
        let mut parser = Parser::default();
        let commands = parser
            .get_program_from_string(Some("bool.egg".to_string()), &content)
            .unwrap();
        assert!(commands.len() > 0);
    }

    #[test]
    fn test_parse_math_egg() {
        let content = fs::read_to_string("examples/math.egg").unwrap();
        let mut parser = Parser::default();
        let commands = parser
            .get_program_from_string(Some("math.egg".to_string()), &content)
            .unwrap();
        assert!(commands.len() > 0);
    }
}
