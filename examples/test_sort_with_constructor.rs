use eggplant_transpiler::ast::parse::Parser;
use eggplant_transpiler::eggplant::{convert_to_eggplant_with_source, EggplantCodeGenerator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test sort with constructor parsing
    let program = r#"
        (sort Bool)
        (constructor True () Bool)
        (constructor False () Bool)

        (sort Int)
        (constructor Zero () Int)
        (constructor Succ (Int) Int)
    "#;

    let mut parser = Parser::default();
    let commands = parser.get_program_from_string(Some("sort_with_constructor.egg".to_string()), program)?;

    println!("Successfully parsed {} commands:", commands.len());
    for (i, cmd) in commands.iter().enumerate() {
        println!("  {}: {:?}", i, cmd);
    }

    // Convert to eggplant commands
    let eggplant_commands = convert_to_eggplant_with_source(&commands, Some("sort_with_constructor.egg".to_string()));

    println!("\nConverted to {} eggplant commands:", eggplant_commands.len());
    for (i, cmd) in eggplant_commands.iter().enumerate() {
        println!("  {}: {:?}", i, cmd);
    }

    // Generate Rust code
    let mut codegen = EggplantCodeGenerator::new();
    let rust_code = codegen.generate_rust(&eggplant_commands);

    println!("\nGenerated Rust code:");
    println!("{}", rust_code);

    // Save generated code
    std::fs::write("generated/sort_with_constructor_generated.rs", &rust_code)?;
    println!("\nGenerated code saved to: generated/sort_with_constructor_generated.rs");

    Ok(())
}