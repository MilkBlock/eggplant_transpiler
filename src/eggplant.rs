//! Eggplant AST and code generator
//!
//! Eggplant is a simplified version of egglog with focus on educational examples

use heck::ToSnakeCase;
use walkdir::IntoIter;

use crate::ast::*;
use std::collections::HashMap;

/// Eggplant DSL type definition
#[derive(Debug, Clone, PartialEq)]
pub struct DslType {
    pub name: String,
    pub variants: Vec<DslVariant>,
}

/// Eggplant DSL variant definition
#[derive(Debug, Clone, PartialEq)]
pub struct DslVariant {
    pub name: String,
    pub fields: Vec<DslField>,
    pub source_file: Option<String>,
    pub source_line: Option<usize>,
}

/// Eggplant DSL field definition
#[derive(Debug, Clone, PartialEq)]
pub struct DslField {
    pub name: String,
    pub field_type: String,
}

/// Eggplant pattern variable definition
#[derive(Debug, Clone, PartialEq)]
pub struct PatternVars {
    pub name: String,
    pub variables: Vec<PatternVariable>,
}

/// Eggplant pattern variable
#[derive(Debug, Clone, PartialEq)]
pub struct PatternVariable {
    pub name: String,
    pub var_type: String,
}

/// Eggplant rule definition
#[derive(Debug, Clone, PartialEq)]
pub struct EggplantRule {
    pub name: String,
    pub pattern_query: String,
    pub action: String,
    pub ruleset: String,
}

/// Eggplant-specific AST nodes
#[derive(Debug, Clone, PartialEq)]
pub enum EggplantCommand {
    /// Define a DSL type with #[eggplant::dsl]
    DslType(DslType),
    /// Define pattern variables with #[eggplant::pat_vars]
    PatternVars(PatternVars),
    /// Define a rule with add_rule
    Rule(EggplantRule),
    /// Create a ruleset
    Ruleset(String),
    /// Run ruleset
    RunRuleset(String, String),
    /// Commit operation
    Commit(String),
    /// Pull operation
    Pull(String),
    /// Transaction definition
    Transaction(String),
    /// Pattern recorder definition
    PatternRecorder(String),
    /// Simple rewrite rule
    Rewrite {
        name: String,
        pattern: Expr,
        replacement: Expr,
    },
    /// Test assertion
    Assert { expr: Expr, expected: Expr },
    /// Variable assignment
    Let { var: String, expr: Expr },
    /// Print statement
    Print { expr: Expr },
}

/// Eggplant command with source file information
#[derive(Debug, Clone, PartialEq)]
pub struct EggplantCommandWithSource {
    pub command: EggplantCommand,
    pub source_file: Option<String>,
    pub source_line: Option<usize>,
}

/// Eggplant code generator
pub struct EggplantCodeGenerator {
    output: String,
    indent_level: usize,
    type_context: HashMap<String, Vec<String>>,
    function_context: HashMap<String, (Vec<String>, String)>,
}

impl EggplantCodeGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
            type_context: HashMap::new(),
            function_context: HashMap::new(),
        }
    }

    /// Generate Rust code with eggplant macros
    pub fn generate_rust(&mut self, commands: &[EggplantCommandWithSource]) -> String {
        self.output.clear();

        self.add_line("// Generated Eggplant Rust Code");
        self.add_line("// Source files referenced in comments below");
        self.add_line("use eggplant::{{prelude::*, tx_rx_vt_pr}};");
        self.add_line("use log::info;");
        self.add_line("");

        // Generate type definitions (outside main)
        for cmd_with_source in commands {
            match &cmd_with_source.command {
                EggplantCommand::DslType(_) | EggplantCommand::PatternVars(_) => {
                    self.generate_rust_command_with_source(cmd_with_source);
                }
                _ => {}
            }
        }

        // Collect ruleset definitions
        let mut ruleset_definitions = Vec::new();
        let mut other_commands = Vec::new();

        for cmd_with_source in commands {
            match &cmd_with_source.command {
                EggplantCommand::Ruleset(_) => {
                    ruleset_definitions.push(cmd_with_source);
                }
                EggplantCommand::DslType(_) | EggplantCommand::PatternVars(_) => {
                    // Skip type definitions (already generated above)
                }
                _ => {
                    other_commands.push(cmd_with_source);
                }
            }
        }

        // Add main function
        self.add_line("fn main() {");
        self.indent();
        self.add_line("env_logger::init();");

        // Generate ruleset definitions at the beginning of main
        for cmd_with_source in ruleset_definitions {
            self.generate_rust_command_with_source(cmd_with_source);
        }

        // Generate other runtime commands
        for cmd_with_source in other_commands {
            self.generate_rust_command_with_source(cmd_with_source);
        }

        self.add_line("info!(\"Eggplant program executed successfully!\");");
        self.dedent();
        self.add_line("}");

        self.output.clone()
    }

    /// Generate Python code from eggplant commands
    pub fn generate_python(&mut self, commands: &[EggplantCommand]) -> String {
        self.output.clear();

        self.add_line("# Generated Eggplant Python Code");
        self.add_line("");
        self.add_line("class EggplantEngine:");
        self.indent();
        self.add_line("def __init__(self):");
        self.indent();
        self.add_line("self.egraph = {}");
        self.add_line("self.rules = {}");
        self.add_line("self.functions = {}");
        self.dedent();
        self.dedent();
        self.add_line("");

        for command in commands {
            self.generate_python_command(command);
        }

        self.output.clone()
    }

    /// Generate JavaScript code from eggplant commands
    pub fn generate_javascript(&mut self, commands: &[EggplantCommand]) -> String {
        self.output.clear();

        self.add_line("// Generated Eggplant JavaScript Code");
        self.add_line("");
        self.add_line("class EggplantEngine {");
        self.indent();
        self.add_line("constructor() {");
        self.indent();
        self.add_line("this.egraph = {};");
        self.add_line("this.rules = {};");
        self.add_line("this.functions = {};");
        self.dedent();
        self.add_line("}");
        self.dedent();
        self.add_line("}");
        self.add_line("");

        for command in commands {
            self.generate_javascript_command(command);
        }

        self.output.clone()
    }

    fn generate_rust_command_with_source(&mut self, cmd_with_source: &EggplantCommandWithSource) {
        // Add source file comment if available
        if let (Some(file), Some(line)) =
            (&cmd_with_source.source_file, cmd_with_source.source_line)
        {
            self.add_line(&format!("// Source: {}:{}", file, line));
        }

        match &cmd_with_source.command {
            EggplantCommand::DslType(dsl_type) => {
                self.add_line(&format!(
                    "// Datatype '{}' defined with variants:",
                    dsl_type.name
                ));
                for variant in &dsl_type.variants {
                    if let (Some(file), Some(line)) = (&variant.source_file, variant.source_line) {
                        self.add_line(&format!(
                            "//   - {}: variant (defined at {}:{})",
                            variant.name, file, line
                        ));
                    } else {
                        self.add_line(&format!("//   - {}: variant", variant.name));
                    }
                }
                self.add_line(&format!("#[eggplant::dsl]"));
                self.add_line(&format!("enum {} {{", dsl_type.name));
                self.indent();
                for variant in &dsl_type.variants {
                    if variant.fields.is_empty() {
                        self.add_line(&format!("{} {{}},", variant.name));
                    } else {
                        self.add_line(&format!("{} {{ ", variant.name));
                        for field in &variant.fields {
                            self.add_line(&format!("    {}: {},", field.name, field.field_type));
                        }
                        self.add_line("},");
                    }
                }
                self.dedent();
                self.add_line("}");
                self.add_line("");
            }
            EggplantCommand::PatternVars(pattern_vars) => {
                self.add_line(&format!("// Pattern variables for rule matching"));
                let var_names: Vec<String> = pattern_vars
                    .variables
                    .iter()
                    .map(|v| v.name.clone())
                    .collect();
                self.add_line(&format!("// Variables: {}", var_names.join(", ")));
                self.add_line(&format!("#[eggplant::pat_vars]"));
                self.add_line(&format!("struct {} {{", pattern_vars.name));
                self.indent();
                for var in &pattern_vars.variables {
                    self.add_line(&format!("{}: {},", var.name, var.var_type));
                }
                self.dedent();
                self.add_line("}");
                self.add_line("");
            }
            EggplantCommand::Rule(rule) => {
                self.add_line(&format!("// Rule: {}", rule.name));
                self.add_line(&format!("MyTx::add_rule("));
                self.indent();
                self.add_line(&format!("\"{}\",", rule.name));
                self.add_line(&format!("{},", rule.ruleset));
                self.add_line(&format!("|| {{"));
                self.indent();
                self.add_line(&format!("{}", rule.pattern_query));
                self.dedent();
                self.add_line("},");
                self.add_line(&format!("|ctx, pat| {{"));
                self.indent();
                self.add_line(&format!("{}", rule.action));
                self.dedent();
                self.add_line("},");
                self.dedent();
                self.add_line(");");
                self.add_line("");
            }
            EggplantCommand::Ruleset(name) => {
                self.add_line(&format!("let {} = MyTx::new_ruleset(\"{}\");", name, name));
            }
            EggplantCommand::Transaction(name) => {
                self.add_line(&format!("tx_rx_vt_pr!({}, MyPatRec);", name));
                self.add_line("");
            }
            EggplantCommand::PatternRecorder(name) => {
                self.add_line(&format!("// Pattern recorder: {}", name));
            }
            EggplantCommand::Commit(expr) => {
                self.add_line(&format!("{}.commit();", expr));
            }
            EggplantCommand::Pull(expr) => {
                self.add_line(&format!("{}.pull();", expr));
            }
            EggplantCommand::RunRuleset(ruleset, config) => {
                self.add_line(&format!(
                    "{}::run_ruleset({}, RunConfig::{});",
                    ruleset, "default_ruleset", config
                ));
            }
            EggplantCommand::Rewrite {
                name,
                pattern,
                replacement,
            } => {
                self.add_line(&format!("// Rewrite rule: {}", name));
                self.add_line(&format!(
                    "// {} => {}",
                    self.expr_to_string(pattern),
                    self.expr_to_string(replacement)
                ));
            }
            EggplantCommand::Assert { expr, expected } => {
                self.add_line(&format!(
                    "// Assert: {} == {}",
                    self.expr_to_string(expr),
                    self.expr_to_string(expected)
                ));
            }
            EggplantCommand::Let { var, expr } => {
                let expr_str = self.expr_to_string(expr);
                // println!("DEBUG: Let command: {} = {}", var, expr_str);
                self.add_line(&format!("let {} = {};", var, expr_str));
            }
            EggplantCommand::Print { expr } => {
                self.add_line(&format!(
                    "info!(\"{{:?}}\", {});",
                    self.expr_to_string(expr)
                ));
            }
        }
    }

    fn generate_python_command(&mut self, command: &EggplantCommand) {
        match command {
            EggplantCommand::DslType(dsl_type) => {
                self.add_line(&format!("# DSL Type definition: {}", dsl_type.name));
                self.add_line(&format!("self.egraph['{}'] = {{}}", dsl_type.name));
                for variant in &dsl_type.variants {
                    self.add_line(&format!(
                        "self.egraph['{}']['{}'] = lambda *args: ('{}', *args)",
                        dsl_type.name, variant.name, variant.name
                    ));
                }
                self.add_line("");
            }
            EggplantCommand::PatternVars(pattern_vars) => {
                self.add_line(&format!("# Pattern variables: {}", pattern_vars.name));
                for var in &pattern_vars.variables {
                    self.add_line(&format!("#   {}: {}", var.name, var.var_type));
                }
                self.add_line("");
            }
            EggplantCommand::Rule(rule) => {
                self.add_line(&format!("# Rule definition: {}", rule.name));
                self.add_line(&format!("self.rules['{}'] = lambda egraph:", rule.name));
                self.indent();
                self.add_line("# Pattern query");
                self.add_line(&format!(
                    "#   {}",
                    rule.pattern_query.replace("\n", "\n#   ")
                ));
                self.add_line("# Action");
                self.add_line(&format!("#   {}", rule.action.replace("\n", "\n#   ")));
                self.dedent();
                self.add_line("");
            }
            EggplantCommand::Ruleset(name) => {
                self.add_line(&format!("# Ruleset: {}", name));
                self.add_line(&format!("self.rulesets['{}'] = []", name));
            }
            EggplantCommand::Transaction(name) => {
                self.add_line(&format!("# Transaction: {}", name));
            }
            EggplantCommand::PatternRecorder(name) => {
                self.add_line(&format!("# Pattern recorder: {}", name));
            }
            EggplantCommand::Commit(expr) => {
                self.add_line(&format!("# Commit: {}", expr));
            }
            EggplantCommand::Pull(expr) => {
                self.add_line(&format!("# Pull: {}", expr));
            }
            EggplantCommand::RunRuleset(ruleset, config) => {
                self.add_line(&format!(
                    "# Run ruleset: {} with config {}",
                    ruleset, config
                ));
            }
            EggplantCommand::Rewrite {
                name,
                pattern,
                replacement,
            } => {
                self.add_line(&format!("# Rewrite rule: {}", name));
                self.add_line(&format!(
                    "self.rules['{}'] = lambda egraph: egraph.rewrite({}, {})",
                    name,
                    self.expr_to_string(pattern),
                    self.expr_to_string(replacement)
                ));
                self.add_line("");
            }
            EggplantCommand::Assert { expr, expected } => {
                self.add_line(&format!("# Assertion"));
                self.add_line(&format!(
                    "assert {} == {}, 'Assertion failed: {} != {}'",
                    self.expr_to_string(expr),
                    self.expr_to_string(expected),
                    self.expr_to_string(expr),
                    self.expr_to_string(expected)
                ));
                self.add_line("");
            }
            EggplantCommand::Let { var, expr } => {
                self.add_line(&format!("{} = {}", var, self.expr_to_string(expr)));
            }
            EggplantCommand::Print { expr } => {
                self.add_line(&format!("print({})", self.expr_to_string(expr)));
            }
        }
    }

    fn generate_javascript_command(&mut self, command: &EggplantCommand) {
        match command {
            EggplantCommand::DslType(dsl_type) => {
                self.add_line(&format!("// DSL Type definition: {}", dsl_type.name));
                self.add_line(&format!("this.egraph['{}'] = {{}};", dsl_type.name));
                for variant in &dsl_type.variants {
                    self.add_line(&format!(
                        "this.egraph['{}']['{}'] = (...args) => ['{}', ...args];",
                        dsl_type.name, variant.name, variant.name
                    ));
                }
                self.add_line("");
            }
            EggplantCommand::PatternVars(pattern_vars) => {
                self.add_line(&format!("// Pattern variables: {}", pattern_vars.name));
                for var in &pattern_vars.variables {
                    self.add_line(&format!("//   {}: {}", var.name, var.var_type));
                }
                self.add_line("");
            }
            EggplantCommand::Rule(rule) => {
                self.add_line(&format!("// Rule definition: {}", rule.name));
                self.add_line(&format!("this.rules['{}'] = (egraph) => {{", rule.name));
                self.indent();
                self.add_line("// Pattern query");
                self.add_line(&format!(
                    "//   {}",
                    rule.pattern_query.replace("\n", "\n//   ")
                ));
                self.add_line("// Action");
                self.add_line(&format!("//   {}", rule.action.replace("\n", "\n//   ")));
                self.dedent();
                self.add_line("};");
                self.add_line("");
            }
            EggplantCommand::Ruleset(name) => {
                self.add_line(&format!("// Ruleset: {}", name));
                self.add_line(&format!("this.rulesets['{}'] = [];", name));
            }
            EggplantCommand::Transaction(name) => {
                self.add_line(&format!("// Transaction: {}", name));
            }
            EggplantCommand::PatternRecorder(name) => {
                self.add_line(&format!("// Pattern recorder: {}", name));
            }
            EggplantCommand::Commit(expr) => {
                self.add_line(&format!("// Commit: {}", expr));
            }
            EggplantCommand::Pull(expr) => {
                self.add_line(&format!("// Pull: {}", expr));
            }
            EggplantCommand::RunRuleset(ruleset, config) => {
                self.add_line(&format!(
                    "// Run ruleset: {} with config {}",
                    ruleset, config
                ));
            }
            EggplantCommand::Rewrite {
                name,
                pattern,
                replacement,
            } => {
                self.add_line(&format!("// Rewrite rule: {}", name));
                self.add_line(&format!(
                    "this.rules['{}'] = (egraph) => egraph.rewrite({}, {});",
                    name,
                    self.expr_to_string(pattern),
                    self.expr_to_string(replacement)
                ));
                self.add_line("");
            }
            EggplantCommand::Assert { expr, expected } => {
                self.add_line(&format!("// Assertion"));
                self.add_line(&format!(
                    "console.assert({} === {}, 'Assertion failed: {} !== {}');",
                    self.expr_to_string(expr),
                    self.expr_to_string(expected),
                    self.expr_to_string(expr),
                    self.expr_to_string(expected)
                ));
                self.add_line("");
            }
            EggplantCommand::Let { var, expr } => {
                let expr_str = self.expr_to_string(expr);
                // println!("DEBUG: Let command: {} = {}", var, expr_str);
                self.add_line(&format!("let {} = {};", var, expr_str));
            }
            EggplantCommand::Print { expr } => {
                self.add_line(&format!("console.log({});", self.expr_to_string(expr)));
            }
        }
    }

    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::Lit(_, lit) => match lit {
                Literal::Int(i) => i.to_string(),
                Literal::Float(f) => f.0.to_string(),
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Bool(b) => b.to_string(),
                Literal::Unit => "()".to_string(),
            },
            Expr::Var(_, var) => normalize_identifier(var),
            Expr::Call(_, func, args) => {
                let arg_str = args
                    .iter()
                    .map(|a| self.expr_with_reference(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                let result = format!("{}::new({})", normalize_identifier(func), arg_str);
                // println!("DEBUG: Call expression: {} -> {}", func, result);
                result
            }
        }
    }

    fn expr_with_reference(&self, expr: &Expr) -> String {
        let expr_str = self.expr_to_string(expr);
        // Check if this is a basic type (literal) or custom type (variable/call)
        match expr {
            Expr::Lit(_, _) => expr_str, // Basic types don't need references
            Expr::Var(_, _) => {
                // Check if variable type is a basic type or custom type
                let var_type = infer_type_from_expr(expr);
                if self.is_basic_type(&var_type) {
                    expr_str // Basic types don't need references
                } else {
                    format!("&{}", expr_str) // Custom types need references
                }
            }
            Expr::Call(_, func, _) => {
                // Check if function return type is a basic type or custom type
                // For constructor calls like g_::new, we need to check the return type
                // Since g_ returns G (custom type), it should use references
                if self.is_basic_type(func) {
                    expr_str // Basic types don't need references
                } else {
                    format!("&{}", expr_str) // Custom types need references
                }
            }
        }
    }

    /// Check if a type is a basic type (i64, f64, bool, String, etc.)
    fn is_basic_type(&self, type_name: &str) -> bool {
        let basic_types = ["i64", "f64", "bool", "String", "()"];
        let rst = basic_types.contains(&type_name) || type_name.starts_with('&');
        if rst {
            println!("{} is basic type", type_name);
        } else {
            println!("{} is complex type", type_name);
        }
        rst
    }

    fn add_line(&mut self, line: &str) {
        let indent = "    ".repeat(self.indent_level);
        self.output.push_str(&format!("{}{}\n", indent, line));
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
}

/// Convert egglog commands to eggplant commands with source information
pub fn convert_to_eggplant_with_source(
    commands: &[Command],
    source_file: Option<String>,
) -> Vec<EggplantCommandWithSource> {
    let mut eggplant_commands = Vec::new();
    let mut dsl_types: HashMap<String, DslType> = HashMap::new();

    // Add transaction definition
    eggplant_commands.push(EggplantCommandWithSource {
        command: EggplantCommand::Transaction("MyTx".to_string()),
        source_file: source_file.clone(),
        source_line: Some(1),
    });

    // First pass: collect all constructors for each datatype/sort with source info
    let mut datatype_constructors: HashMap<String, Vec<DslVariant>> = HashMap::new();
    let mut constructor_line_numbers: HashMap<String, usize> = HashMap::new();
    let mut constructor_counter = 1;

    for command in commands {
        constructor_counter += 1;
        if let Command::Constructor { name, schema, .. } = command {
            // Extract the datatype/sort name from the schema output
            let datatype_name = &schema.output;

            let dsl_variant = DslVariant {
                name: normalize_identifier(name),
                fields: schema
                    .input
                    .iter()
                    .enumerate()
                    .map(|(i, field_type)| DslField {
                        name: format!("arg{}", i),
                        field_type: normalize_identifier(field_type),
                    })
                    .collect(),
                source_file: source_file.clone(),
                source_line: Some(constructor_counter),
            };

            constructor_line_numbers.insert(name.clone(), constructor_counter);

            datatype_constructors
                .entry(normalize_identifier(datatype_name))
                .or_insert_with(Vec::new)
                .push(dsl_variant);
        }
    }

    for command in commands {
        match command {
            Command::Datatype {
                name,
                span,
                variants,
            } => {
                // Use the collected constructors for this datatype
                let mut dsl_variants = datatype_constructors
                    .get(name)
                    .cloned()
                    .unwrap_or_else(Vec::new);

                // Use the datatype command inner variants for this datatype
                for (counter, variant) in variants.iter().enumerate() {
                    dsl_variants.push(DslVariant {
                        name: variant.name.clone(),
                        fields: variant
                            .types
                            .iter()
                            .enumerate()
                            .map(|(i, ty)| (ty, format!("arg_{}_{}{}", ty, counter, i)))
                            .map(|(ty, name)| DslField {
                                name,
                                field_type: ty.clone(),
                            })
                            .collect(),
                        source_file: variant.span.file.clone(),
                        source_line: Some(variant.span.line),
                    });
                }

                let dsl_type = DslType {
                    name: normalize_identifier(name),
                    variants: dsl_variants.clone(),
                };
                dsl_types.insert(normalize_identifier(name), dsl_type.clone());

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::DslType(dsl_type),
                    source_file: source_file.clone(),
                    source_line: Some(span.line),
                });
            }
            // Command::Constructor { name, schema, .. } => {
            //     // For constructor commands, create DSL type definitions
            //     let datatype_name = &schema.output;

            //     let dsl_variant = DslVariant {
            //         name: normalize_identifier(name),
            //         fields: schema
            //             .input
            //             .iter()
            //             .enumerate()
            //             .map(|(i, field_type)| DslField {
            //                 name: format!("arg{}", i),
            //                 field_type: normalize_identifier(field_type),
            //             })
            //             .collect(),
            //         source_file: source_file.clone(),
            //         source_line: Some(span.line),
            //     };

            //     // Debug logging
            //     println!(
            //         "DEBUG: Processing Constructor command: {} for datatype: {}",
            //         name, datatype_name
            //     );

            //     // Check if we already have a DSL type for this datatype
            //     let existing_type_index = eggplant_commands.iter().position(|cmd| {
            //         if let EggplantCommand::DslType(dsl_type) = &cmd.command {
            //             dsl_type.name == normalize_identifier(datatype_name)
            //         } else {
            //             false
            //         }
            //     });

            //     if let Some(index) = existing_type_index {
            //         // Add variant to existing type
            //         if let EggplantCommand::DslType(dsl_type) =
            //             &mut eggplant_commands[index].command
            //         {
            //             dsl_type.variants.push(dsl_variant);
            //         }
            //     } else {
            //         // Create new DSL type
            //         eggplant_commands.push(EggplantCommandWithSource {
            //             command: EggplantCommand::DslType(DslType {
            //                 name: normalize_identifier(datatype_name),
            //                 variants: vec![dsl_variant],
            //             }),
            //             source_file: source_file.clone(),
            //             source_line: Some(span.line),
            //         });
            //     }
            // }
            Command::Function {
                name, schema, span, ..
            } => {
                // Create pattern variables for function - these are the matched nodes
                let pattern_vars = PatternVars {
                    name: format!("{}Pat", normalize_identifier(name)),
                    variables: schema
                        .input
                        .iter()
                        .enumerate()
                        .map(|(i, typ)| PatternVariable {
                            name: format!("arg{}", i),
                            var_type: normalize_identifier(typ),
                        })
                        .collect(),
                };

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::PatternVars(pattern_vars),
                    source_file: source_file.clone(),
                    source_line: Some(span.line),
                });

                // Create rule for function
                let rule = EggplantRule {
                    name: format!("{}_rule", normalize_identifier(name)),
                    pattern_query: format!(
                        "// TODO: implement pattern query for {}",
                        normalize_identifier(name)
                    ),
                    action: format!("// TODO: implement {} action", normalize_identifier(name)),
                    ruleset: "default_ruleset".to_string(),
                };

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::Rule(rule),
                    source_file: source_file.clone(),
                    source_line: Some(span.line),
                });
            }
            Command::Rule { name, rule, .. } => {
                // For rules, we need to analyze the pattern to determine what nodes are matched
                // For now, create a simple pattern with generic nodes
                let unique_name = if name == "default" {
                    format!("rule_{}", rule.span.line)
                } else {
                    normalize_identifier(name)
                };
                let pattern_vars = PatternVars {
                    name: format!("{}Pat", unique_name),
                    variables: vec![
                        PatternVariable {
                            name: "l".to_string(),
                            var_type: "Expr".to_string(),
                        },
                        PatternVariable {
                            name: "r".to_string(),
                            var_type: "Expr".to_string(),
                        },
                        PatternVariable {
                            name: "p".to_string(),
                            var_type: normalize_identifier(name),
                        },
                    ],
                };

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::PatternVars(pattern_vars),
                    source_file: source_file.clone(),
                    source_line: Some(rule.span.line),
                });

                // Create rule with proper pattern query
                let pattern_query = format!(
                    "let l = Expr::query_leaf();\nlet r = Expr::query_leaf();\nlet p = {}::query(&l, &r);\n{}Pat::new(l, r, p)",
                    name, unique_name
                );

                let action = format!(
                    "// TODO: implement action for {}\nlet result = ctx.insert_expr(/* calculation */);\nctx.union(p, result);",
                    unique_name
                );

                let eggplant_rule = EggplantRule {
                    name: unique_name.clone(),
                    pattern_query,
                    action,
                    ruleset: "default_ruleset".to_string(),
                };

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::Rule(eggplant_rule),
                    source_file: source_file.clone(),
                    source_line: Some(rule.span.line),
                });
            }
            Command::Rewrite(name, rewrite, _) => {
                // Analyze the rewrite pattern to extract variables and structure
                // Always use line counter for unique pattern variable names to avoid conflicts
                let unique_name = format!("rule_{}", rewrite.span.line);
                let (pattern_vars, pattern_query, context) =
                    analyze_rewrite_pattern(&rewrite.lhs, &unique_name, &dsl_types);

                // Create pattern variables
                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::PatternVars(pattern_vars),
                    source_file: source_file.clone(),
                    source_line: Some(rewrite.span.line),
                });

                // Create rule with pattern query and action
                let action = generate_rewrite_action_with_context(
                    &rewrite.rhs,
                    &unique_name,
                    &context,
                    &dsl_types,
                );

                let rule = EggplantRule {
                    name: unique_name.clone(),
                    pattern_query,
                    action,
                    ruleset: "default_ruleset".to_string(),
                };

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::Rule(rule),
                    source_file: source_file.clone(),
                    source_line: Some(rewrite.span.line),
                });
            }
            Command::Check(span, facts) => {
                for fact in facts {
                    if let Fact::Eq(_, e1, e2) = fact {
                        eggplant_commands.push(EggplantCommandWithSource {
                            command: EggplantCommand::Assert {
                                expr: e1.clone(),
                                expected: e2.clone(),
                            },
                            source_file: source_file.clone(),
                            source_line: Some(span.line),
                        });
                    }
                }
            }
            Command::Constructor {
                name, schema, span, ..
            } => {
                // Constructor commands are already handled above
                // Skip duplicate processing
            }
            Command::Action(action) => {
                if let Action::Let(span, var, expr) = action {
                    eggplant_commands.push(EggplantCommandWithSource {
                        command: EggplantCommand::Let {
                            var: normalize_identifier(var),
                            expr: expr.clone(),
                        },
                        source_file: source_file.clone(),
                        source_line: Some(span.line),
                    });
                }
            }
            Command::Push(_) => {
                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::Commit("current_expr".to_string()),
                    source_file: source_file.clone(),
                    source_line: Some(1), // Default line for Push command
                });
            }
            Command::Pop(span, _) => {
                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::Pull("current_expr".to_string()),
                    source_file: source_file.clone(),
                    source_line: Some(span.line),
                });
            }
            Command::Sort(span, name, _) => {
                // Sort command defines a basic type
                // Use the collected constructors for this sort
                let dsl_variants = datatype_constructors
                    .get(name)
                    .cloned()
                    .unwrap_or_else(Vec::new);

                let dsl_type = DslType {
                    name: normalize_identifier(name),
                    variants: dsl_variants.clone(),
                };
                dsl_types.insert(normalize_identifier(name), dsl_type.clone());

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::DslType(dsl_type),
                    source_file: source_file.clone(),
                    source_line: Some(span.line),
                });
            }
            _ => {
                // Skip unsupported commands for now
            }
        }
    }

    // Add ruleset and run commands
    eggplant_commands.push(EggplantCommandWithSource {
        command: EggplantCommand::Ruleset("default_ruleset".to_string()),
        source_file: source_file.clone(),
        source_line: Some(1), // Default line number
    });
    eggplant_commands.push(EggplantCommandWithSource {
        command: EggplantCommand::RunRuleset("MyTx".to_string(), "Sat".to_string()),
        source_file: source_file.clone(),
        source_line: Some(1), // Default line number
    });

    eggplant_commands
}

/// Normalize identifier by replacing invalid characters with underscores
fn normalize_identifier(identifier: &str) -> String {
    identifier
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Check if a type is a basic type (i64, f64, bool, String, etc.)
fn is_basic_type(type_name: &str) -> bool {
    let basic_types = [
        "i64",
        "f64",
        "bool",
        "String",
        "&'static str",
        "BigRational",
        "()",
    ];
    basic_types.contains(&type_name)
}

/// Infer type from expression context with better variable type inference
fn infer_type_from_expr(expr: &Expr) -> String {
    match expr {
        Expr::Call(_, func_name, _) => normalize_identifier(func_name),
        Expr::Var(sp, name) => format!("{:?} {name} UNKNOWN TYPE", sp), // Default type for variables
        Expr::Lit(_, lit) => match lit {
            Literal::Int(_) => "i64".to_string(),
            Literal::Float(_) => "f64".to_string(),
            Literal::String(_) => "String".to_string(),
            Literal::Bool(_) => "bool".to_string(),
            Literal::Unit => "()".to_string(),
        },
    }
}

/// Infer variable type from constructor context using DSL type information
fn infer_variable_type_from_constructor(
    constructor_name: &str,
    arg_index: usize,
    dsl_types: &HashMap<String, DslType>,
) -> String {
    // Look for the constructor in DSL types
    for (dsl_type_name, dsl_type) in dsl_types {
        // Find the variant that matches the constructor name
        if let Some(variant) = dsl_type
            .variants
            .iter()
            .find(|v| v.name == constructor_name)
        {
            // Check if we have a field at this argument index
            if arg_index < variant.fields.len() {
                println!("{} infer to be {}", constructor_name, dsl_type_name);
                return variant.fields[arg_index].field_type.clone();
            } else {
                println!("{} infer to be {}", constructor_name, dsl_type_name);
                // If no field at this index, return the DSL type name
                return dsl_type_name.clone();
            }
        }
    }

    // Fallback: if constructor not found in DSL types, panic
    panic!("unable to infer {constructor_name}")
}

/// Generate better pattern query with type inference and variable context
fn generate_pattern_query_with_context(
    lhs: &Expr,
    rule_name: &str,
    dsl_types: &HashMap<String, DslType>,
) -> (
    PatternVars,
    String,
    HashMap<String, (String, String, usize)>,
) {
    let mut variables = Vec::new();
    let mut pattern_query_parts = Vec::new();
    let mut node_counter = 0;
    let mut variable_constructors = HashMap::new();
    let mut all_nodes = Vec::new();

    // Extract variables with better type inference and build the query tree
    let root_node = extract_variables_with_types_and_context(
        lhs,
        &mut variables,
        &mut pattern_query_parts,
        &mut node_counter,
        dsl_types,
        &mut variable_constructors,
        &mut all_nodes,
    );

    // Create pattern variables struct - include constructor nodes only for basic type access
    let mut pattern_vars_variables = variables.clone();

    // Add constructor nodes to pattern variables only if they have basic type fields
    for (var_name, (constructor_name, node_name, _)) in &variable_constructors {
        if !pattern_vars_variables.iter().any(|v| v.name == *node_name) {
            // Check if this constructor has any basic type fields
            let has_basic_type_fields = dsl_types
                .values()
                .flat_map(|dsl_type| &dsl_type.variants)
                .find(|variant| variant.name == *constructor_name)
                .map_or(false, |variant| {
                    variant
                        .fields
                        .iter()
                        .any(|field| is_basic_type(&field.field_type))
                });

            if has_basic_type_fields {
                pattern_vars_variables.push(dbg!(PatternVariable {
                    name: node_name.clone(),
                    var_type: constructor_name.clone(),
                }));
            }
        }
    }

    // For rewrite rules, add the root node to PatternVars for union operation
    let mut pattern_vars_variables = dbg!(pattern_vars_variables);
    if !pattern_vars_variables.iter().any(|v| v.name == root_node) {
        // Infer the type of the root node
        let root_node_type = infer_type_from_expr(lhs);
        pattern_vars_variables.push(dbg!(PatternVariable {
            name: root_node.clone(),
            var_type: root_node_type,
        }));
    }

    // Generate improved pattern query
    let pattern_query = if pattern_query_parts.is_empty() {
        format!("// TODO: implement pattern query for {}", rule_name)
    } else {
        let mut query = pattern_query_parts.join("\n");
        query.push_str(&format!("\n{}Pat::new(", rule_name));

        // Add only the variables that are actually in PatternVars to the struct creation
        let pattern_var_refs: Vec<String> = pattern_vars_variables
            .iter()
            .map(|var| var.name.clone())
            .collect();
        query.push_str(&pattern_var_refs.join(", "));
        query.push(')');
        query
    };

    let pattern_vars = PatternVars {
        name: format!("{}Pat", rule_name),
        variables: pattern_vars_variables,
    };

    (pattern_vars, pattern_query, variable_constructors)
}

/// Extract variables with type inference and build query pattern tree with variable context
fn extract_variables_with_types_and_context(
    expr: &Expr,
    variables: &mut Vec<PatternVariable>,
    pattern_query_parts: &mut Vec<String>,
    node_counter: &mut usize,
    dsl_types: &HashMap<String, DslType>,
    variable_constructors: &mut HashMap<String, (String, String, usize)>,
    all_nodes: &mut Vec<String>,
) -> String {
    match expr {
        Expr::Var(_, var_name) => {
            // Variable reference - add to pattern variables with inferred type
            let var_type = infer_type_from_expr(expr);

            // Only add complex type variables to pattern variables
            // Basic type variables will be accessed through constructor instance fields
            let normalized_var_name = normalize_identifier(var_name);
            if !is_basic_type(&var_type) && !variables.iter().any(|v| v.name == normalized_var_name)
            {
                variables.push(dbg!(PatternVariable {
                    name: normalized_var_name.clone(),
                    var_type: var_type.clone(),
                }));
                // For complex types, add query
                let query_method = if is_leaf_constructor(&var_type, dsl_types) {
                    "query"
                } else {
                    "query"
                };
                pattern_query_parts.push(dbg!(format!(
                    "let {} = {}::{}();",
                    normalized_var_name, var_type, query_method
                )));
                // Add variable node to all nodes
                all_nodes.push(normalized_var_name.clone());
            }
            normalized_var_name
        }
        Expr::Call(_, func_name, args) => {
            // Function call - recursively extract variables from arguments and build the tree
            let mut complex_arg_nodes = Vec::new();
            let mut basic_conditions = Vec::new();

            // Generate unique node name for this constructor call (shared for all arguments)
            *node_counter += 1;
            let constructor_node_name = format!(
                "{}_node{}",
                normalize_identifier(&func_name.to_snake_case()),
                node_counter
            );

            for (index, arg) in args.iter().enumerate() {
                // For constructor calls, infer variable types from context
                let arg_node = if let Expr::Var(_, var_name) = arg {
                    // This variable appears in a constructor call - infer its type
                    let inferred_type =
                        infer_variable_type_from_constructor(func_name, index, dsl_types);

                    // Record the constructor context for this variable - use shared constructor node
                    let normalized_var_name = normalize_identifier(var_name);
                    variable_constructors.insert(
                        normalized_var_name.clone(),
                        (func_name.clone(), constructor_node_name.clone(), index),
                    );

                    // Only add complex type variables to pattern variables
                    // Basic type variables will be accessed through constructor instance fields
                    if !is_basic_type(&inferred_type)
                        && !variables.iter().any(|v| v.name == normalized_var_name)
                    {
                        variables.push(dbg!(PatternVariable {
                            name: normalized_var_name.clone(),
                            var_type: inferred_type.clone(),
                        }));
                        // For complex types, add query
                        let query_method = if is_leaf_constructor(&inferred_type, dsl_types) {
                            format!(r#"query_leaf /* infered as {} */"#, inferred_type)
                        } else {
                            format!(r#"query_leaf /* infered as {} */"#, inferred_type)
                        };
                        pattern_query_parts.push(dbg!(format!(
                            "let {} = {}::{}();",
                            normalized_var_name, inferred_type, query_method
                        )));
                    }
                    normalized_var_name
                } else {
                    // For non-variable arguments, use the normal recursive extraction
                    extract_variables_with_types_and_context(
                        arg,
                        variables,
                        pattern_query_parts,
                        node_counter,
                        dsl_types,
                        variable_constructors,
                        all_nodes,
                    )
                };

                // Check if this argument is a basic type (literal or basic type variable)
                let arg_type = infer_variable_type_from_constructor(func_name, index, dsl_types);
                if is_basic_type(&arg_type) {
                    // For basic types, create StrippedCondition instead of query parameter
                    if let Expr::Var(_, var_name) = arg {
                        // Basic type variables are accessed through their constructor variant fields
                        // We need to find the field name for this variable in the constructor
                        let field_name =
                            get_field_name_for_variable_in_constructor(func_name, index, dsl_types);
                        basic_conditions.push(format!(
                            "pat.{}.{} == {}",
                            constructor_node_name, field_name, arg_node
                        ));
                    } else {
                        // For literals, just use the value directly
                        basic_conditions
                            .push(format!("pat.{} == {}", constructor_node_name, arg_node));
                    }
                } else {
                    // For complex types, add to argument nodes only if this is not a leaf constructor
                    if !is_leaf_constructor(func_name, dsl_types) {
                        println!("push {}:{}", arg_node, arg_type);
                        complex_arg_nodes.push(arg_node);
                    }
                }
            }

            // Special case: if this is a constructor call with only variable arguments,
            // we still need to create query nodes for the constructor to represent the hyperedge
            // but we can simplify the return value
            let all_args_are_variables = args.iter().all(|arg| matches!(arg, Expr::Var(_, _)));
            if all_args_are_variables && complex_arg_nodes.len() == args.len() {
                // For constructor calls like (Const b), we still need the constructor query
                // but we can use the first variable node as the return value for pattern matching
                if !complex_arg_nodes.is_empty() {
                    // We still create the constructor query node, but return the variable for pattern
                    // This ensures all constructor queries are generated
                }
            }

            // For leaf constructors (only basic type arguments), use the shared constructor node
            // For internal constructors (with complex type arguments), create a new query node
            let node_name = if is_leaf_constructor(func_name, dsl_types) {
                // Use the shared constructor node for leaf constructors
                constructor_node_name
            } else {
                // Generate unique node name for internal constructors
                // *node_counter += 1;
                // format!(
                //     "{}_node{}",
                //     normalize_identifier(&func_name.to_snake_case()),
                //     node_counter
                // )
                constructor_node_name
            };

            // Add this node to the list of all nodes
            all_nodes.push(node_name.clone());

            // Build argument references - only for complex types
            let arg_refs: Vec<String> = complex_arg_nodes
                .iter()
                .map(|node_name| {
                    // Check if this is a literal (starts with digit or quote)
                    if node_name
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_digit(10) || c == '"')
                    {
                        node_name.clone()
                    } else {
                        println!(" recog as complex {:?}", node_name);
                        format!("&{}", node_name)
                    }
                })
                .collect();

            // Add the query for this function call (the hyperedge) - only complex args
            // Use query_leaf() for leaf nodes (no arguments), query() for internal nodes
            if is_leaf_constructor(func_name, dsl_types) {
                // Leaf nodes use query_leaf() without arguments
                // Only add the query if we haven't already added it for this constructor
                if !pattern_query_parts
                    .iter()
                    .any(|part| part.contains(&node_name))
                {
                    pattern_query_parts.push(dbg!(format!(
                        "let {} = {}::query();",
                        node_name,
                        normalize_identifier(func_name)
                    )));
                }
            } else {
                // Internal nodes use query() with arguments
                pattern_query_parts.push(dbg!(format!(
                    "let {} = {}::query({});",
                    node_name,
                    normalize_identifier(func_name),
                    arg_refs.join(", ")
                )));
            }

            // Add StrippedCondition checks for basic types
            if !basic_conditions.is_empty() {
                pattern_query_parts.push(dbg!(format!(
                    "// StrippedCondition checks: {}",
                    basic_conditions.join(" && ")
                )));
            }

            node_name
        }
        Expr::Lit(_, lit) => {
            // Literal - no variables to extract, return literal value
            match lit {
                Literal::Int(i) => i.to_string(),
                Literal::Float(f) => f.0.to_string(),
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Bool(b) => b.to_string(),
                Literal::Unit => "()".to_string(),
            }
        }
    }
}

/// Convert egglog commands to eggplant commands (backward compatibility)
pub fn convert_to_eggplant(commands: &[Command]) -> Vec<EggplantCommand> {
    convert_to_eggplant_with_source(commands, None)
        .into_iter()
        .map(|cmd_with_source| cmd_with_source.command)
        .collect()
}

/// Variable context that maps variables to their constructor and field information
#[derive(Debug, Clone)]
struct VariableContext {
    variables: Vec<PatternVariable>,
    variable_constructors: HashMap<String, (String, String, usize)>, // var_name -> (constructor_name, node_name, arg_index)
}

/// Analyze a rewrite pattern to extract pattern variables and generate pattern query
fn analyze_rewrite_pattern(
    lhs: &Expr,
    rule_name: &str,
    dsl_types: &HashMap<String, DslType>,
) -> (PatternVars, String, VariableContext) {
    let (pattern_vars, pattern_query, variable_constructors) =
        generate_pattern_query_with_context(lhs, rule_name, dsl_types);
    let context = VariableContext {
        variables: pattern_vars.variables.clone(),
        variable_constructors,
    };
    (pattern_vars, pattern_query, context)
}

/// Generate rewrite action from the right-hand side pattern with variable context
fn generate_rewrite_action_with_context(
    rhs: &Expr,
    rule_name: &str,
    context: &VariableContext,
    dsl_types: &HashMap<String, DslType>,
) -> String {
    let result_expr = generate_insert_expr(rhs, context, dsl_types);

    // Find the root node in PatternVars - this should be the node that matches the LHS pattern structure
    // For rewrite rules, we need to find the node that represents the entire pattern being rewritten
    // Look for constructor nodes that are not basic type variables
    let root_node_name = context
        .variables
        .iter()
        .filter(|v| !is_basic_type(&v.var_type))
        .find(|v| v.name.ends_with("_node") || v.name.contains("node"))
        .map(|v| v.name.clone())
        .unwrap_or_else(|| {
            // Fallback: use the first non-basic type variable
            context
                .variables
                .iter()
                .find(|v| !is_basic_type(&v.var_type))
                .map(|v| v.name.clone())
                .unwrap_or_else(|| {
                    format!("{}_node1", normalize_identifier(&rule_name.to_snake_case()))
                })
        });

    format!(
        "let result = {};\nctx.union(pat.{}, result);",
        result_expr, root_node_name
    )
}

/// Generate insert expression for RHS nodes with proper function names and parameter ordering
fn generate_insert_expr(
    expr: &Expr,
    context: &VariableContext,
    dsl_types: &HashMap<String, DslType>,
) -> String {
    match expr {
        Expr::Var(_, var_name) => {
            // Simple variable reference
            // Check if this variable is a basic type accessed through constructor
            let normalized_var_name = normalize_identifier(var_name);
            if let Some((constructor_name, node_name, arg_index)) =
                context.variable_constructors.get(&normalized_var_name)
            {
                // Check if this constructor has basic type fields at this argument position
                let has_basic_type_field = dsl_types
                    .values()
                    .flat_map(|dsl_type| &dsl_type.variants)
                    .find(|variant| variant.name == *constructor_name)
                    .map_or(false, |variant| {
                        if *arg_index < variant.fields.len() {
                            is_basic_type(&variant.fields[*arg_index].field_type)
                        } else {
                            false
                        }
                    });

                if has_basic_type_field {
                    // Basic type variable accessed through constructor field
                    let field_name = get_field_name_for_variable_in_constructor(
                        constructor_name,
                        *arg_index,
                        dsl_types,
                    );
                    format!("ctx.devalue(pat.{}.{})", node_name, field_name)
                } else {
                    // TODO insert function might be different when insert container
                    // Complex type variable - use the variable directly
                    if let Some(var_info) = context
                        .variables
                        .iter()
                        .find(|v| v.name == normalized_var_name)
                    {
                        // let insert_function =
                        //     format!("insert_{}", var_info.var_type.to_snake_case());
                        // format!("ctx.{}(pat.{})", insert_function, normalized_var_name)
                        format!("pat.{}", normalized_var_name)
                    } else {
                        // Fallback for complex type variables
                        format!("pat.{}", normalized_var_name)
                    }
                }
            } else {
                // TODO insert function might be different when insert container
                // Complex type variable - we need to access its fields
                // Find the variable type and generate appropriate insert function
                if let Some(var_info) = context
                    .variables
                    .iter()
                    .find(|v| v.name == normalized_var_name)
                {
                    // let insert_function = format!("insert_{}", var_info.var_type.to_snake_case());
                    // format!("ctx.{}(pat.{})", insert_function, normalized_var_name)
                    format!("pat.{}", normalized_var_name)
                } else {
                    // Fallback for complex type variables
                    format!("pat.{}", normalized_var_name)
                }
            }
        }
        Expr::Call(_, func_name, args) => {
            // Check if this is a basic operation (+, -, *, /) that needs ctx.devalue()
            if is_basic_operation(func_name) {
                // For basic operations, we need to extract values using ctx.devalue()
                let operation_args: Vec<String> = args
                    .iter()
                    .map(|arg| {
                        let expr = generate_insert_expr(arg, context, dsl_types);
                        // For basic operations, we need to extract the actual values, not insert them
                        if expr.starts_with("ctx.devalue(") {
                            // Already using devalue, just use as is
                            expr
                        } else if expr.starts_with("ctx.insert_") {
                            // For complex type inserts, we need to use ctx.devalue()
                            format!("ctx.devalue({})", expr)
                        } else {
                            expr
                        }
                    })
                    .collect();

                // For basic operations, compute the value
                format!(
                    "({} {} {})",
                    operation_args[0], func_name, operation_args[1]
                )
            } else {
                // Complex type constructor call - generate proper insert function
                // Get the variant information to ensure correct parameter ordering
                let variant_info = find_variant_info(func_name, dsl_types);

                let arg_exprs: Vec<String> = args
                    .iter()
                    .map(|arg| generate_insert_expr(arg, context, dsl_types))
                    .collect();

                // Generate the correct insert function name based on variant
                let insert_function = format!("insert_{}", func_name.to_snake_case());

                // For complex types, we need to ensure the arguments are in the correct order
                // based on the variant's field declarations
                if let Some(variant) = variant_info {
                    // The arguments should already be in the correct order from the AST
                    // We just need to make sure we're using the right insert function
                    format!("ctx.{}({})", insert_function, arg_exprs.join(", "))
                } else {
                    // Fallback for unknown variants
                    format!("ctx.{}({})", insert_function, arg_exprs.join(", "))
                }
            }
        }
        Expr::Lit(_, lit) => {
            // Literal value - use directly for basic literals
            match lit {
                Literal::Int(i) => i.to_string(),
                Literal::Float(f) => f.0.to_string(),
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Bool(b) => b.to_string(),
                Literal::Unit => "()".to_string(),
            }
        }
    }
}

/// Check if a function name represents a basic operation
fn is_basic_operation(func_name: &str) -> bool {
    let basic_operations = ["+", "-", "*", "/", "<", ">", "<=", ">=", "==", "!="];
    basic_operations.contains(&func_name)
}

/// Check if a constructor is a leaf node (has only basic type arguments)
fn is_leaf_constructor(func_name: &str, dsl_types: &HashMap<String, DslType>) -> bool {
    // Look for the constructor in DSL types
    for (_, dsl_type) in dsl_types {
        if let Some(variant) = dsl_type.variants.iter().find(|v| v.name == func_name) {
            // Check if all fields are basic types
            return variant
                .fields
                .iter()
                .all(|field| is_basic_type(&field.field_type));
        }
    }
    false
}

/// Get the field name for a variable in a specific constructor
fn get_field_name_for_variable_in_constructor(
    constructor_name: &str,
    arg_index: usize,
    dsl_types: &HashMap<String, DslType>,
) -> String {
    // Look for the constructor in DSL types
    for (_, dsl_type) in dsl_types {
        if let Some(variant) = dsl_type
            .variants
            .iter()
            .find(|v| v.name == constructor_name)
        {
            // Check if we have a field at this argument index
            if arg_index < variant.fields.len() {
                return variant.fields[arg_index].name.clone();
            }
        }
    }

    // Fallback: use generic field name
    format!("arg{}", arg_index)
}

/// Find variant information for a constructor
fn find_variant_info<'a>(
    constructor_name: &str,
    dsl_types: &'a HashMap<String, DslType>,
) -> Option<&'a DslVariant> {
    for (_, dsl_type) in dsl_types {
        if let Some(variant) = dsl_type
            .variants
            .iter()
            .find(|v| v.name == constructor_name)
        {
            return Some(variant);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::parse::Parser;

    #[test]
    fn test_eggplant_conversion() {
        let program = r#"
            (datatype Math)
            (constructor Num (i64) Math)
            (let x (Num 42))
        "#;

        let mut parser = Parser::default();
        let commands = parser.get_program_from_string(None, program).unwrap();
        let eggplant_commands = convert_to_eggplant(&commands);

        assert!(eggplant_commands.len() >= 3); // Transaction, DslType, Let, Ruleset, RunRuleset

        // Check for DslType
        let has_dsl_type = eggplant_commands.iter().any(|cmd| {
            if let EggplantCommand::DslType(dsl_type) = cmd {
                dsl_type.name == "Math"
            } else {
                false
            }
        });
        assert!(has_dsl_type);
    }

    #[test]
    fn test_rust_generation() {
        let commands = vec![
            EggplantCommandWithSource {
                command: EggplantCommand::DslType(DslType {
                    name: "Math".to_string(),
                    variants: vec![
                        DslVariant {
                            name: "Num".to_string(),
                            fields: vec![DslField {
                                name: "num".to_string(),
                                field_type: "i64".to_string(),
                            }],
                            source_file: Some("test.egg".to_string()),
                            source_line: Some(1),
                        },
                        DslVariant {
                            name: "Add".to_string(),
                            fields: vec![
                                DslField {
                                    name: "l".to_string(),
                                    field_type: "Math".to_string(),
                                },
                                DslField {
                                    name: "r".to_string(),
                                    field_type: "Math".to_string(),
                                },
                            ],
                            source_file: Some("test.egg".to_string()),
                            source_line: Some(2),
                        },
                    ],
                }),
                source_file: Some("test.egg".to_string()),
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
                source_file: Some("test.egg".to_string()),
                source_line: Some(2),
            },
            EggplantCommandWithSource {
                command: EggplantCommand::Rule(EggplantRule {
                    name: "add_rule".to_string(),
                    pattern_query: "let l = Num::query();\nlet r = Num::query();\nlet p = Add::query(&l, &r);\nAddPat::new(l, r, p)".to_string(),
                    action: "let cal = ctx.devalue(l.num) + ctx.devalue(r.num);\nlet add_value = ctx.insert_num(cal);\nctx.union(p, add_value);".to_string(),
                    ruleset: "default_ruleset".to_string(),
                }),
                source_file: Some("test.egg".to_string()),
                source_line: Some(3),
            },
        ];

        let mut codegen = EggplantCodeGenerator::new();
        let rust_code = codegen.generate_rust(&commands);

        assert!(rust_code.contains("#[eggplant::dsl]"));
        assert!(rust_code.contains("enum Math"));
        assert!(rust_code.contains("#[eggplant::pat_vars]"));
        assert!(rust_code.contains("struct AddPat"));
        assert!(rust_code.contains("add_rule"));
    }

    #[test]
    fn test_python_generation() {
        let commands = vec![
            EggplantCommand::DslType(DslType {
                name: "Math".to_string(),
                variants: vec![DslVariant {
                    name: "Num".to_string(),
                    fields: vec![DslField {
                        name: "num".to_string(),
                        field_type: "i64".to_string(),
                    }],
                    source_file: Some("test.egg".to_string()),
                    source_line: Some(1),
                }],
            }),
            EggplantCommand::Assert {
                expr: Expr::Call(
                    Span::new(None, 1, 1),
                    "add".to_string(),
                    vec![
                        Expr::Call(
                            Span::new(None, 1, 1),
                            "Num".to_string(),
                            vec![Expr::Lit(Span::new(None, 1, 1), Literal::Int(1))],
                        ),
                        Expr::Call(
                            Span::new(None, 1, 1),
                            "Num".to_string(),
                            vec![Expr::Lit(Span::new(None, 1, 1), Literal::Int(2))],
                        ),
                    ],
                ),
                expected: Expr::Call(
                    Span::new(None, 1, 1),
                    "Num".to_string(),
                    vec![Expr::Lit(Span::new(None, 1, 1), Literal::Int(3))],
                ),
            },
        ];

        let mut codegen = EggplantCodeGenerator::new();
        let python_code = codegen.generate_python(&commands);

        assert!(python_code.contains("class EggplantEngine"));
        assert!(python_code.contains("Math"));
        assert!(python_code.contains("assert"));
    }

    #[test]
    fn test_javascript_generation() {
        let commands = vec![
            EggplantCommand::DslType(DslType {
                name: "Bool".to_string(),
                variants: vec![
                    DslVariant {
                        name: "True".to_string(),
                        fields: vec![],
                        source_file: Some("test.egg".to_string()),
                        source_line: Some(1),
                    },
                    DslVariant {
                        name: "False".to_string(),
                        fields: vec![],
                        source_file: Some("test.egg".to_string()),
                        source_line: Some(1),
                    },
                ],
            }),
            EggplantCommand::Let {
                var: "x".to_string(),
                expr: Expr::Call(Span::new(None, 1, 1), "True".to_string(), vec![]),
            },
        ];

        let mut codegen = EggplantCodeGenerator::new();
        let js_code = codegen.generate_javascript(&commands);

        assert!(js_code.contains("class EggplantEngine"));
        assert!(js_code.contains("Bool"));
        assert!(js_code.contains("let x ="));
    }
}
