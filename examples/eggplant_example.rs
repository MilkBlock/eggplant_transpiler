//! Eggplant code generation example
//!
//! This example demonstrates how to convert egglog programs to eggplant
//! and generate code in multiple target languages.

use eggplant_transpiler::ast::parse::Parser;
use eggplant_transpiler::eggplant::*;
use eggplant_transpiler::{Expr, Literal, Span};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Eggplant Code Generation Example ===");

    // Parse calc.egg and convert to eggplant
    println!("\n1. Parsing calc.egg and converting to eggplant...");
    let calc_content = fs::read_to_string("examples/calc.egg")?;
    let mut parser = Parser::default();
    let calc_commands = parser.get_program_from_string(Some("calc.egg".to_string()), &calc_content)?;
    let eggplant_commands = convert_to_eggplant(&calc_commands);

    println!("Converted {} egglog commands to {} eggplant commands",
        calc_commands.len(), eggplant_commands.len());

    // Generate Rust code
    println!("\n2. Generating Rust code...");
    let mut rust_gen = EggplantCodeGenerator::new();
    let rust_code = rust_gen.generate_rust(&eggplant_commands);
    println!("Generated {} lines of Rust code", rust_code.lines().count());

    // Parse bool.egg and convert to eggplant
    println!("\n4. Parsing bool.egg and converting to eggplant...");
    let bool_content = fs::read_to_string("examples/bool.egg")?;
    let bool_commands = parser.get_program_from_string(Some("bool.egg".to_string()), &bool_content)?;
    let bool_eggplant = convert_to_eggplant(&bool_commands);

    println!("Converted {} egglog commands to {} eggplant commands",
        bool_commands.len(), bool_eggplant.len());

    // Generate Rust code for bool example
    println!("\n5. Generating Rust code for bool example...");
    let bool_rust = rust_gen.generate_rust(&bool_eggplant);
    println!("Generated {} lines of Rust code", bool_rust.lines().count());

    // Parse math.egg and convert to eggplant
    println!("\n6. Parsing math.egg and converting to eggplant...");
    let math_content = fs::read_to_string("examples/math.egg")?;
    let math_commands = parser.get_program_from_string(Some("math.egg".to_string()), &math_content)?;
    let math_eggplant = convert_to_eggplant(&math_commands);

    println!("Converted {} egglog commands to {} eggplant commands",
        math_commands.len(), math_eggplant.len());

    // Generate Rust code for math example
    println!("\n7. Generating Rust code for math example...");
    let math_rust = rust_gen.generate_rust(&math_eggplant);
    println!("Generated {} lines of Rust code", math_rust.lines().count());

    // Save generated code to files
    println!("\n8. Saving generated code to files...");

    fs::create_dir_all("generated/eggplant")?;

    fs::write("generated/eggplant/calc.rs", &rust_code)?;
    fs::write("generated/eggplant/bool.rs", &bool_rust)?;
    fs::write("generated/eggplant/math.rs", &math_rust)?;

    println!("\nGenerated files saved to:");
    println!("  - generated/eggplant/calc.rs");
    println!("  - generated/eggplant/bool.rs");
    println!("  - generated/eggplant/math.rs");

    // Demonstrate simple eggplant program
    println!("\n9. Demonstrating simple eggplant program...");

    let simple_commands = vec![
        EggplantCommand::DslType(DslType {
            name: "Math".to_string(),
            variants: vec![
                DslVariant {
                    name: "Num".to_string(),
                    fields: vec![DslField {
                        name: "value".to_string(),
                        field_type: "i64".to_string(),
                    }],
                },
                DslVariant {
                    name: "Add".to_string(),
                    fields: vec![
                        DslField {
                            name: "left".to_string(),
                            field_type: "Math".to_string(),
                        },
                        DslField {
                            name: "right".to_string(),
                            field_type: "Math".to_string(),
                        },
                    ],
                },
            ],
        }),
        EggplantCommand::PatternVars(PatternVars {
            name: "AddPat".to_string(),
            variables: vec![
                PatternVariable {
                    name: "l".to_string(),
                    var_type: "Num".to_string(),
                },
                PatternVariable {
                    name: "r".to_string(),
                    var_type: "Num".to_string(),
                },
                PatternVariable {
                    name: "p".to_string(),
                    var_type: "Add".to_string(),
                },
            ],
        }),
        EggplantCommand::Rule(EggplantRule {
            name: "add_rule".to_string(),
            pattern_query: "let l = Num::query();\nlet r = Num::query();\nlet p = Add::query(&l, &r);\nAddPat::new(l, r, p)".to_string(),
            action: "let cal = ctx.devalue(pat.l.num) + ctx.devalue(pat.r.num);\nlet add_value = ctx.insert_num(cal);\nctx.union(pat.p, add_value);".to_string(),
            ruleset: "default_ruleset".to_string(),
        }),
        EggplantCommand::Rewrite {
            name: "add_simplify".to_string(),
            pattern: Expr::Call(
                Span::new(None, 1, 1),
                "Add".to_string(),
                vec![
                    Expr::Call(Span::new(None, 1, 1), "Num".to_string(),
                        vec![Expr::Lit(Span::new(None, 1, 1), Literal::Int(1))]),
                    Expr::Call(Span::new(None, 1, 1), "Num".to_string(),
                        vec![Expr::Lit(Span::new(None, 1, 1), Literal::Int(2))])
                ]
            ),
            replacement: Expr::Call(Span::new(None, 1, 1), "Num".to_string(),
                vec![Expr::Lit(Span::new(None, 1, 1), Literal::Int(3))]),
        },
        EggplantCommand::Assert {
            expr: Expr::Call(Span::new(None, 1, 1), "eval".to_string(),
                vec![Expr::Call(Span::new(None, 1, 1), "Add".to_string(),
                    vec![
                        Expr::Call(Span::new(None, 1, 1), "Num".to_string(),
                            vec![Expr::Lit(Span::new(None, 1, 1), Literal::Int(1))]),
                        Expr::Call(Span::new(None, 1, 1), "Num".to_string(),
                            vec![Expr::Lit(Span::new(None, 1, 1), Literal::Int(2))])
                    ]
                )]
            ),
            expected: Expr::Call(Span::new(None, 1, 1), "Num".to_string(),
                vec![Expr::Lit(Span::new(None, 1, 1), Literal::Int(3))]),
        },
    ];

    let simple_rust = rust_gen.generate_rust(&simple_commands);

    fs::write("generated/eggplant/simple.rs", &simple_rust)?;

    println!("  - generated/eggplant/simple.rs");

    println!("\n=== Eggplant Code Generation Complete ===");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eggplant_example() {
        // Test that we can parse and convert egglog files
        let calc_content = fs::read_to_string("examples/calc.egg").unwrap();
        let mut parser = Parser::default();
        let commands = parser
            .get_program_from_string(Some("calc.egg".to_string()), &calc_content)
            .unwrap();

        let eggplant_commands = convert_to_eggplant(&commands);
        assert!(!eggplant_commands.is_empty());

        // Test code generation
        let mut codegen = EggplantCodeGenerator::new();
        let rust_code = codegen.generate_rust(&eggplant_commands);

        assert!(rust_code.contains("#[eggplant::dsl]"));
        assert!(rust_code.contains("enum"));
    }

    #[test]
    fn test_simple_eggplant_program() {
        let commands = vec![
            EggplantCommand::DslType(DslType {
                name: "Test".to_string(),
                variants: vec![
                    DslVariant {
                        name: "A".to_string(),
                        fields: vec![],
                    },
                    DslVariant {
                        name: "B".to_string(),
                        fields: vec![],
                    },
                ],
            }),
            EggplantCommand::Let {
                var: "x".to_string(),
                expr: Expr::Call(Span::new(None, 1, 1), "A".to_string(), vec![]),
            },
        ];

        let mut codegen = EggplantCodeGenerator::new();
        let rust_code = codegen.generate_rust(&commands);

        assert!(rust_code.contains("Test"));
        assert!(rust_code.contains("let x ="));
    }
}