use eggplant_transpiler::ast::parse::Parser;
use eggplant_transpiler::eggplant::{convert_to_eggplant_with_source, EggplantCodeGenerator};

fn main() {
    // Test basic type filtering with (Add (Num a) (Num b))
    let program = r#"
        (datatype Expr
          (Num i64)
          (Add Expr Expr))

        (rewrite (Add (Num a) (Num b)) (Num (+ a b)))
    "#;

    let mut parser = Parser::default();
    let commands = parser.get_program_from_string(None, program).unwrap();

    println!("Parsed {} commands:", commands.len());
    for (i, cmd) in commands.iter().enumerate() {
        println!("  Command {}: {:?}", i, cmd);
    }

    let eggplant_commands =
        convert_to_eggplant_with_source(&commands, Some("test.egg".to_string()));

    println!("\nGenerated {} eggplant commands:", eggplant_commands.len());
    for (i, cmd) in eggplant_commands.iter().enumerate() {
        println!("  Command {}: {:?}", i, cmd.command);
    }

    // Print the generated Rust code
    let mut codegen = EggplantCodeGenerator::new();
    let rust_code = codegen.generate_rust(&eggplant_commands);

    println!("\n=== Generated Rust Code ===");
    println!("{}", rust_code);
}