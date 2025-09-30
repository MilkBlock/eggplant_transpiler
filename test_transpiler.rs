use eggplant_transpiler::ast::parse::Parser;
use eggplant_transpiler::eggplant::{EggplantCodeGenerator, convert_to_eggplant_with_source};

fn main() {
    let _program = r#"
        (datatype ExprE
          (ConstE i64)
          (DivE i64 i64))

        (rewrite (DivE a b) (ConstE (/ a b)))
    "#;
    let program = r#"
        (datatype ExprE
          (ConstE i64 MarkE)
          (DivE i64 i64))

        (datatype MarkE
          (Mark i64)
        )

        (rewrite (DivE a b) (ConstE (/ a b) (Mark 0)))
    "#;

    let _program = r#"
        (datatype Expr
          (Add Expr Expr)
          (Mul Expr Expr)
          (Const i64)
          (Comb Expr Expr)
        )

        (rewrite (Add (Mul a b) (Mul a b)) (Mul (Add a b) (Add a b)))
    "#;
    let program = "(datatype LoopType (Loop String Math))
(datatype*
 	(Expr
  		; General kernel stuff
     	(GMEM String)
     	(LoopIn Expr LoopType Math)
     	(LoopOut Expr LoopType Math)
      	(SMEM)
       	(SMEMLoad Expr Expr)
        (SMEMRead Expr Expr)

        ; Unary Ops
     	(Exp2 Expr)
      	(Log2 Expr)
    	(Sqrt Expr)
     	(Sin Expr)
      	(Recip Expr)
       	(Neg Expr)

        ; Binary Ops
     	(Add Expr Expr)
     	(Mul Expr Expr)
      	(Max Expr Expr)

        ; search helpers
        (Unary String Expr)
     	(Binary String Expr Expr)
      	(SwapLoops Expr String String) ; Swap two loops, identified by their string
       	(TileLoop Expr String) ; Tile a loop, identified by it's string
        (UnpadLoop Expr String) ; Remove a padding loop, identified by it's string
     )
)";
    let program = r#"(datatype
 	Expr
     	(Sin Expr)
        (Unary String Expr)

)
       (rewrite (Unary "Sin" ?x) (Sin ?x))
 
        "#;

    env_logger::init();
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

    // Save the generated code to a file
    let output_path = "generated/eggplant/code.rs";
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(output_path, &rust_code).expect("Failed to write generated code");
    println!("\n✅ File has been saved to: {}", output_path);
}
