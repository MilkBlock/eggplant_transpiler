use eggplant_transpiler::ast::parse::Parser;
use eggplant_transpiler::eggplant::{
    EggplantCodeGenerator, convert_to_eggplant_with_source_and_program,
};

fn main() {
    let program = r#"
(datatype Math
	(MNum i64)
	(MVar String)
	(MAdd Math Math)
	(MSub Math Math)
	(MMul Math Math)
	(MDiv Math Math)
	(MMod Math Math)
	(MMin Math Math)
	(MMax Math Math)
	(MAnd Math Math)
	(MOr Math Math)
	(MGte Math Math)
	(MLt Math Math)
	(MFloorTo Math Math)
    (MReplace Math Math Math)
    (MAccum String) ; this marks that we feed the output (also marked with MAccum) back in
)

(datatype LoopType (Loop String Math))

(datatype
 	Expr
     	(GMEM String)
     	(LoopIn Expr LoopType Math)
     	(LoopOut Expr LoopType Math)
      	(SMEM)
       	(SMEMLoad Expr Expr)
        (SMEMRead Expr Expr)
     	(Exp2 Expr)
      	(Log2 Expr)
    	(Sqrt Expr)
     	(Sin Expr)
      	(Recip Expr)
       	(Neg Expr)
     	(Add Expr Expr)
     	(Mul Expr Expr)
      	(Max Expr Expr)
        (Unary String Expr)
     	(Binary String Expr Expr)
      	(SwapLoops Expr String String) ; Swap two loops, identified by their string
       	(TileLoop Expr String) ; Tile a loop, identified by it's string
        (UnpadLoop Expr String) ; Remove a padding loop, identified by it's string
)
    (rewrite (MMul (MNum a) (MNum b)) (MNum (* a b)) :when ((< a 10000) (< b 10000)) :ruleset default)

           
"#;

    let rust_code = transform_with_log(&program);
    // Save the generated code to a file
    let output_path = "generated/eggplant/code.rs";
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(output_path, &rust_code).expect("Failed to write generated code");
    println!("\n✅ File has been saved to: {}", output_path);
}
#[test]
fn when_prim() {
    let program = r#"
(datatype Math
	(MNum i64)
	(MVar String)
	(MAdd Math Math)
	(MSub Math Math)
	(MMul Math Math)
	(MDiv Math Math)
	(MMod Math Math)
	(MMin Math Math)
	(MMax Math Math)
	(MAnd Math Math)
	(MOr Math Math)
	(MGte Math Math)
	(MLt Math Math)
	(MFloorTo Math Math)
    (MReplace Math Math Math)
    (MAccum String) ; this marks that we feed the output (also marked with MAccum) back in
)

(datatype LoopType (Loop String Math))

(datatype
 	Expr
     	(GMEM String)
     	(LoopIn Expr LoopType Math)
     	(LoopOut Expr LoopType Math)
      	(SMEM)
       	(SMEMLoad Expr Expr)
        (SMEMRead Expr Expr)
     	(Exp2 Expr)
      	(Log2 Expr)
    	(Sqrt Expr)
     	(Sin Expr)
      	(Recip Expr)
       	(Neg Expr)
     	(Add Expr Expr)
     	(Mul Expr Expr)
      	(Max Expr Expr)
        (Unary String Expr)
     	(Binary String Expr Expr)
      	(SwapLoops Expr String String) ; Swap two loops, identified by their string
       	(TileLoop Expr String) ; Tile a loop, identified by it's string
        (UnpadLoop Expr String) ; Remove a padding loop, identified by it's string
)

    (rewrite
        (TileLoop (LoopIn ?body (Loop ?loop (MNum ?range)) ?stride) ?loop)
        (LoopIn (LoopIn ?body (Loop ?loop (MNum (/ ?range 8))) (MReplace ?stride (MVar "z") (MMul (MVar "z") (MNum 8)))) (Loop (+ ?loop "_tile") (MNum 8)) ?stride)
	:when ((> ?range 8) (= (% ?range 8) 0))
)"#;
    transform(program);
}

#[test]
fn when_lt_le() {
    let program = r#"(datatype Math
	(MNum i64)
	(MMul Math Math)
    )

   (rewrite (MMul (MNum a) (MNum b)) (MNum (* a b)) :when ((< a 10000) (<= b 10000)))
"#;
    transform(program);
}
#[test]
fn when_complex() {
    let program = r#"
(datatype Math
	(MNum i64)
	(MVar String)
	(MAdd Math Math)
	(MSub Math Math)
	(MMul Math Math)
	(MDiv Math Math)
	(MMod Math Math)
	(MMin Math Math)
	(MMax Math Math)
	(MAnd Math Math)
	(MOr Math Math)
	(MGte Math Math)
	(MLt Math Math)
	(MFloorTo Math Math)
    (MReplace Math Math Math)
    (MAccum String) ; this marks that we feed the output (also marked with MAccum) back in
)

(datatype LoopType (Loop String Math))

(datatype
 	Expr
     	(GMEM String)
     	(LoopIn Expr LoopType Math)
     	(LoopOut Expr LoopType Math)
      	(SMEM)
       	(SMEMLoad Expr Expr)
        (SMEMRead Expr Expr)
     	(Exp2 Expr)
      	(Log2 Expr)
    	(Sqrt Expr)
     	(Sin Expr)
      	(Recip Expr)
       	(Neg Expr)
     	(Add Expr Expr)
     	(Mul Expr Expr)
      	(Max Expr Expr)
        (Unary String Expr)
     	(Binary String Expr Expr)
      	(SwapLoops Expr String String) ; Swap two loops, identified by their string
       	(TileLoop Expr String) ; Tile a loop, identified by it's string
        (UnpadLoop Expr String) ; Remove a padding loop, identified by it's string
)

(rewrite
	(LoopOut (Binary ?bin2 (Binary ?bin ?a ?b) ?c) (Loop ?l ?r) ?s)
	(LoopOut (LoopOut (Binary ?bin2 (Binary ?bin (LoopIn ?a (Loop "newpad" (MNum 1)) (MNum 0)) (LoopIn ?b (Loop "newpad" (MNum 1)) (MNum 0))) (LoopIn ?c (Loop "newpad" (MNum 1)) (MNum 0))) (Loop "newpad" (MNum 1)) (MNum 0)) (Loop ?l ?r) ?s)
	:when ((!= ?r (MNum 1)) (!= ?s (MNum 0)))
)"#;
    transform(program);
}
#[test]
fn when_eq() {
    let program = r#"(datatype Math
	(MNum i64)
	(MMul Math Math)
    )
   (rewrite (MMul (MNum a) (MNum b)) (MNum (* a b)) :when ((= a 10000) (= b 10000)))
"#;
    transform(program);
}
#[test]
fn rewrite_union_what() {
    let program = r#"(datatype Math
	(MNum i64)
	(MAdd Math Math)
	(MAnd Math Math)
    )
    (rewrite (MAnd (MNum a) (MNum b)) (MNum (& a b)))
"#;
    transform(program);
}

#[test]
fn long_dsl() {
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
    transform(program);
}
#[test]
fn div_primitive() {
    let program = r#"
        (datatype ExprE
          (ConstE i64)
          (DivE i64 i64))

        (rewrite (DivE a b) (ConstE (/ a b)))
    "#;
    transform(program);
}

#[test]
fn rewrite() {
    let program = r#"
        (datatype Expr
          (Add Expr Expr)
          (Mul Expr Expr)
          (Const i64)
          (Comb Expr Expr)
        )

        (rewrite (Add (Mul a b) (Mul a b)) (Mul (Add a b) (Add a b)))
    "#;
    transform(program);
}
#[test]
fn two_dsl() {
    let program = r#"
        (datatype ExprE
          (ConstE i64 MarkE)
          (DivE i64 i64))

        (datatype MarkE
          (Mark i64)
        )

        (rewrite (DivE a b) (ConstE (/ a b) (Mark 0)))
    "#;
    transform(program);
}

fn transform(program: &str) -> String {
    let mut parser = Parser::default();
    let mut codegen = EggplantCodeGenerator::new();
    let commands = parser.get_program_from_string(None, program).unwrap();
    let eggplant_commands =
        convert_to_eggplant_with_source_and_program(&commands, Some("test.egg".to_string()));
    let rust_code = codegen.generate_rust(&eggplant_commands);
    println!("\n=== Generated Rust Code ===");
    println!("{}", rust_code);
    rust_code
}

fn transform_with_log(program: &str) -> String {
    env_logger::init();
    let mut parser = Parser::default();
    let commands = parser.get_program_from_string(None, program).unwrap();

    println!("Parsed {} commands:", commands.len());
    for (i, cmd) in commands.iter().enumerate() {
        println!("  Command {}: {:?}", i, cmd);
        // Debug rewrite conditions
        if let eggplant_transpiler::ast::Command::Rewrite(_, rewrite, _) = cmd {
            println!("    Conditions: {:?}", rewrite.conditions);
            println!("    Conditions length: {}", rewrite.conditions.len());
        }
    }

    let eggplant_commands =
        convert_to_eggplant_with_source_and_program(&commands, Some("test.egg".to_string()));

    println!("\nGenerated {} eggplant commands:", eggplant_commands.len());
    for (i, cmd) in eggplant_commands.iter().enumerate() {
        println!("  Command {}: {:?}", i, cmd.command);
    }

    // Print the generated Rust code
    let mut codegen = EggplantCodeGenerator::new();
    let rust_code = codegen.generate_rust(&eggplant_commands);
    println!("\n=== Generated Rust Code ===");
    println!("{}", rust_code);
    rust_code
}

#[test]
fn unary() {
    let program = r#"(datatype
 	Expr
     	(Sin Expr)
        (Unary String Expr)
)
       (rewrite (Unary "Sin" ?x) (Sin ?x)) "#;
    transform(program);
}
