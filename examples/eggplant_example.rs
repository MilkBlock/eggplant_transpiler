//! Eggplant code generation example
//!
//! This example demonstrates how to convert egglog programs to eggplant
//! and generate code in multiple target languages.

use eggplant_transpiler::ast::parse::Parser;
use eggplant_transpiler::eggplant::*;
use eggplant_transpiler::{Expr, Literal, Span};
use log::{info, debug};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("=== Eggplant Code Generation Example ===");

    // Traverse all .egg files in examples folder
    info!("1. Traversing .egg files in examples folder...");
    let mut parser = Parser::default();
    let mut rust_gen = EggplantCodeGenerator::new();
    let mut generated_files = Vec::new();

    for entry in fs::read_dir("examples")? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "egg") {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            info!("=== Parsing {} ===", file_name);

            let content = fs::read_to_string(&path)?;
            let commands = parser.get_program_from_string(Some(file_name.clone()), &content)?;
            let eggplant_commands = convert_to_eggplant_with_source(&commands, Some(path.to_string_lossy().to_string()));

            debug!("Converted {} egglog commands to {} eggplant commands",
                commands.len(), eggplant_commands.len());

            let rust_code = rust_gen.generate_rust(&eggplant_commands);
            debug!("Generated {} lines of Rust code", rust_code.lines().count());

            let output_file = format!("generated/eggplant/{}.rs", path.file_stem().unwrap().to_string_lossy());
            fs::create_dir_all("generated/eggplant")?;
            fs::write(&output_file, &rust_code)?;

            generated_files.push(output_file);
        }
    }

    info!("Generated files saved to:");
    for file in &generated_files {
        info!("  - {}", file);
    }

    // Demonstrate simple eggplant program
    info!("9. Demonstrating simple eggplant program...");

    let simple_commands = vec![
        EggplantCommandWithSource {
            command: EggplantCommand::DslType(DslType {
                name: "Math".to_string(),
                variants: vec![
                    DslVariant {
                        name: "Num".to_string(),
                        fields: vec![DslField {
                            name: "value".to_string(),
                            field_type: "i64".to_string(),
                        }],
                        source_file: Some("examples/simple.egg".to_string()),
                        source_line: Some(1),
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
                        source_file: Some("examples/simple.egg".to_string()),
                        source_line: Some(2),
                    },
                ],
            }),
            source_file: Some("examples/simple.egg".to_string()),
            source_line: Some(1),
        },
        EggplantCommandWithSource {
            command: EggplantCommand::PatternVars(PatternVars {
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
            source_file: Some("examples/simple.egg".to_string()),
            source_line: Some(2),
        },
        EggplantCommandWithSource {
            command: EggplantCommand::Rule(EggplantRule {
                name: "add_rule".to_string(),
                pattern_query: "let l = Num::query();\nlet r = Num::query();\nlet p = Add::query(&l, &r);\nAddPat::new(l, r, p)".to_string(),
                action: "let cal = ctx.devalue(pat.l.num) + ctx.devalue(pat.r.num);\nlet add_value = ctx.insert_num(cal);\nctx.union(pat.p, add_value);".to_string(),
                ruleset: "default_ruleset".to_string(),
            }),
            source_file: Some("examples/simple.egg".to_string()),
            source_line: Some(3),
        },
        EggplantCommandWithSource {
            command: EggplantCommand::Rewrite {
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
            source_file: Some("examples/simple.egg".to_string()),
            source_line: Some(4),
        },
        EggplantCommandWithSource {
            command: EggplantCommand::Assert {
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
            source_file: Some("examples/simple.egg".to_string()),
            source_line: Some(5),
        },
    ];

    let simple_rust = rust_gen.generate_rust(&simple_commands);

    fs::write("generated/eggplant/simple.rs", &simple_rust)?;

    info!("  - generated/eggplant/simple.rs");

    info!("=== Eggplant Code Generation Complete ===");

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

        let eggplant_commands = convert_to_eggplant_with_source(&commands, Some("examples/calc.egg".to_string()));
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
            EggplantCommandWithSource {
                command: EggplantCommand::DslType(DslType {
                    name: "Test".to_string(),
                    variants: vec![
                        DslVariant {
                            name: "A".to_string(),
                            fields: vec![],
                            source_file: Some("test.egg".to_string()),
                            source_line: Some(1),
                        },
                        DslVariant {
                            name: "B".to_string(),
                            fields: vec![],
                            source_file: Some("test.egg".to_string()),
                            source_line: Some(1),
                        },
                    ],
                }),
                source_file: Some("test.egg".to_string()),
                source_line: Some(1),
            },
            EggplantCommandWithSource {
                command: EggplantCommand::Let {
                    var: "x".to_string(),
                    expr: Expr::Call(Span::new(None, 1, 1), "A".to_string(), vec![]),
                },
                source_file: Some("test.egg".to_string()),
                source_line: Some(2),
            },
        ];

        let mut codegen = EggplantCodeGenerator::new();
        let rust_code = codegen.generate_rust(&commands);

        assert!(rust_code.contains("Test"));
        assert!(rust_code.contains("let x ="));
    }
}