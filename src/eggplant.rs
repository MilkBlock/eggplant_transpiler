//! Eggplant AST and code generator
//!
//! Eggplant is a simplified version of egglog with focus on educational examples

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
    Assert {
        expr: Expr,
        expected: Expr,
    },
    /// Variable assignment
    Let {
        var: String,
        expr: Expr,
    },
    /// Print statement
    Print {
        expr: Expr,
    },
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

        // Add main function
        self.add_line("fn main() {");
        self.indent();

        // Generate runtime commands (inside main)
        for cmd_with_source in commands {
            match &cmd_with_source.command {
                EggplantCommand::DslType(_) | EggplantCommand::PatternVars(_) => {
                    // Skip type definitions (already generated above)
                }
                _ => {
                    self.generate_rust_command_with_source(cmd_with_source);
                }
            }
        }

        self.add_line("println!(\"Eggplant program executed successfully!\");");
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
        if let (Some(file), Some(line)) = (&cmd_with_source.source_file, cmd_with_source.source_line) {
            self.add_line(&format!("// Source: {}:{}", file, line));
        }

        match &cmd_with_source.command {
            EggplantCommand::DslType(dsl_type) => {
                self.add_line(&format!("// Datatype '{}' defined with variants:", dsl_type.name));
                for variant in &dsl_type.variants {
                    if let (Some(file), Some(line)) = (&variant.source_file, variant.source_line) {
                        self.add_line(&format!("//   - {}: variant (defined at {}:{})", variant.name, file, line));
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
                let var_names: Vec<String> = pattern_vars.variables.iter().map(|v| v.name.clone()).collect();
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
                self.add_line(&format!("{}::run_ruleset({}, RunConfig::{});", ruleset, ruleset, config));
            }
            EggplantCommand::Rewrite { name, pattern, replacement } => {
                self.add_line(&format!("// Rewrite rule: {}", name));
                self.add_line(&format!("// {} => {}", self.expr_to_string(pattern), self.expr_to_string(replacement)));
            }
            EggplantCommand::Assert { expr, expected } => {
                self.add_line(&format!("// Assert: {} == {}", self.expr_to_string(expr), self.expr_to_string(expected)));
            }
            EggplantCommand::Let { var, expr } => {
                self.add_line(&format!("let {} = {};", var, self.expr_to_string(expr)));
            }
            EggplantCommand::Print { expr } => {
                self.add_line(&format!("println!(\"{{:?}}\", {});", self.expr_to_string(expr)));
            }
        }
    }

    fn generate_python_command(&mut self, command: &EggplantCommand) {
        match command {
            EggplantCommand::DslType(dsl_type) => {
                self.add_line(&format!("# DSL Type definition: {}", dsl_type.name));
                self.add_line(&format!("self.egraph['{}'] = {{}}", dsl_type.name));
                for variant in &dsl_type.variants {
                    self.add_line(&format!("self.egraph['{}']['{}'] = lambda *args: ('{}', *args)",
                        dsl_type.name, variant.name, variant.name));
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
                self.add_line(&format!("#   {}", rule.pattern_query.replace("\n", "\n#   ")));
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
                self.add_line(&format!("# Run ruleset: {} with config {}", ruleset, config));
            }
            EggplantCommand::Rewrite { name, pattern, replacement } => {
                self.add_line(&format!("# Rewrite rule: {}", name));
                self.add_line(&format!("self.rules['{}'] = lambda egraph: egraph.rewrite({}, {})",
                    name, self.expr_to_string(pattern), self.expr_to_string(replacement)));
                self.add_line("");
            }
            EggplantCommand::Assert { expr, expected } => {
                self.add_line(&format!("# Assertion"));
                self.add_line(&format!("assert {} == {}, 'Assertion failed: {} != {}'",
                    self.expr_to_string(expr), self.expr_to_string(expected),
                    self.expr_to_string(expr), self.expr_to_string(expected)));
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
                    self.add_line(&format!("this.egraph['{}']['{}'] = (...args) => ['{}', ...args];",
                        dsl_type.name, variant.name, variant.name));
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
                self.add_line(&format!("//   {}", rule.pattern_query.replace("\n", "\n//   ")));
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
                self.add_line(&format!("// Run ruleset: {} with config {}", ruleset, config));
            }
            EggplantCommand::Rewrite { name, pattern, replacement } => {
                self.add_line(&format!("// Rewrite rule: {}", name));
                self.add_line(&format!("this.rules['{}'] = (egraph) => egraph.rewrite({}, {});",
                    name, self.expr_to_string(pattern), self.expr_to_string(replacement)));
                self.add_line("");
            }
            EggplantCommand::Assert { expr, expected } => {
                self.add_line(&format!("// Assertion"));
                self.add_line(&format!("console.assert({} === {}, 'Assertion failed: {} !== {}');",
                    self.expr_to_string(expr), self.expr_to_string(expected),
                    self.expr_to_string(expr), self.expr_to_string(expected)));
                self.add_line("");
            }
            EggplantCommand::Let { var, expr } => {
                self.add_line(&format!("let {} = {};", var, self.expr_to_string(expr)));
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
                Literal::String(s) => format!("'{}'", s),
                Literal::Bool(b) => b.to_string(),
                Literal::Unit => "()".to_string(),
            },
            Expr::Var(_, var) => normalize_identifier(var),
            Expr::Call(_, func, args) => {
                let arg_str = args.iter().map(|a| self.expr_to_string(a)).collect::<Vec<_>>().join(", ");
                format!("{}({})", normalize_identifier(func), arg_str)
            }
        }
    }

    fn action_to_string(&self, action: &Action) -> String {
        match action {
            Action::Let(_, var, expr) => {
                format!("let {} = {}", var, self.expr_to_string(expr))
            }
            Action::Set(_, func, args, value) => {
                let arg_str = args.iter().map(|a| self.expr_to_string(a)).collect::<Vec<_>>().join(", ");
                format!("{}.set({}, {})", func, arg_str, self.expr_to_string(value))
            }
            Action::Union(_, e1, e2) => {
                format!("egraph.union({}, {})", self.expr_to_string(e1), self.expr_to_string(e2))
            }
            Action::Expr(_, expr) => {
                self.expr_to_string(expr)
            }
        }
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
pub fn convert_to_eggplant_with_source(commands: &[Command], source_file: Option<String>) -> Vec<EggplantCommandWithSource> {
    let mut eggplant_commands = Vec::new();

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
                fields: schema.input.iter().enumerate().map(|(i, field_type)| {
                    DslField {
                        name: format!("arg{}", i),
                        field_type: normalize_identifier(field_type),
                    }
                }).collect(),
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

    let mut line_counter = 1;

    for command in commands {
        line_counter += 1;

        match command {
            Command::Datatype { name, .. } => {
                // Use the collected constructors for this datatype
                let dsl_variants = datatype_constructors
                    .get(name)
                    .cloned()
                    .unwrap_or_else(Vec::new);

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::DslType(DslType {
                        name: normalize_identifier(name),
                        variants: dsl_variants,
                    }),
                    source_file: source_file.clone(),
                    source_line: Some(line_counter),
                });
            }
            Command::Function { name, schema, .. } => {
                // Create pattern variables for function - these are the matched nodes
                let pattern_vars = PatternVars {
                    name: format!("{}Pat", normalize_identifier(name)),
                    variables: schema.input.iter().enumerate().map(|(i, typ)| {
                        PatternVariable {
                            name: format!("arg{}", i),
                            var_type: normalize_identifier(typ),
                        }
                    }).collect(),
                };

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::PatternVars(pattern_vars),
                    source_file: source_file.clone(),
                    source_line: Some(line_counter),
                });

                // Create rule for function
                let rule = EggplantRule {
                    name: format!("{}_rule", normalize_identifier(name)),
                    pattern_query: format!("// TODO: implement pattern query for {}", normalize_identifier(name)),
                    action: format!("// TODO: implement {} action", normalize_identifier(name)),
                    ruleset: "default_ruleset".to_string(),
                };

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::Rule(rule),
                    source_file: source_file.clone(),
                    source_line: Some(line_counter),
                });
            }
            Command::Rule { name, rule, .. } => {
                // For rules, we need to analyze the pattern to determine what nodes are matched
                // For now, create a simple pattern with generic nodes
                let unique_name = if name == "default" {
                    format!("rule_{}", line_counter)
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
                    source_line: Some(line_counter),
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
                    source_line: Some(line_counter),
                });
            }
            Command::Rewrite(name, rewrite, _) => {
                // Analyze the rewrite pattern to extract variables and structure
                let unique_name = if name == "default" {
                    format!("rule_{}", line_counter)
                } else {
                    normalize_identifier(name)
                };
                let (pattern_vars, pattern_query) = analyze_rewrite_pattern(&rewrite.lhs, &unique_name);

                // Create pattern variables
                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::PatternVars(pattern_vars),
                    source_file: source_file.clone(),
                    source_line: Some(line_counter),
                });

                // Create rule with pattern query and action
                let action = generate_rewrite_action(&rewrite.rhs, &unique_name);

                let rule = EggplantRule {
                    name: unique_name.clone(),
                    pattern_query,
                    action,
                    ruleset: "default_ruleset".to_string(),
                };

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::Rule(rule),
                    source_file: source_file.clone(),
                    source_line: Some(line_counter),
                });
            }
            Command::Check(_, facts) => {
                for fact in facts {
                    if let Fact::Eq(_, e1, e2) = fact {
                        eggplant_commands.push(EggplantCommandWithSource {
                            command: EggplantCommand::Assert {
                                expr: e1.clone(),
                                expected: e2.clone(),
                            },
                            source_file: source_file.clone(),
                            source_line: Some(line_counter),
                        });
                    }
                }
            }
            Command::Constructor { name, schema, .. } => {
                // Constructor commands are already handled in the first pass
                // We only need to process the let bindings if they exist
                // For now, skip the constructor command itself
            }
            Command::Action(action) => {
                if let Action::Let(_, var, expr) = action {
                    eggplant_commands.push(EggplantCommandWithSource {
                        command: EggplantCommand::Let {
                            var: normalize_identifier(var),
                            expr: expr.clone(),
                        },
                        source_file: source_file.clone(),
                        source_line: Some(line_counter),
                    });
                }
            }
            Command::Push(_) => {
                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::Commit("current_expr".to_string()),
                    source_file: source_file.clone(),
                    source_line: Some(line_counter),
                });
            }
            Command::Pop(_, _) => {
                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::Pull("current_expr".to_string()),
                    source_file: source_file.clone(),
                    source_line: Some(line_counter),
                });
            }
            Command::Sort(_, name, _) => {
                // Sort command defines a basic type
                // Use the collected constructors for this sort
                let dsl_variants = datatype_constructors
                    .get(name)
                    .cloned()
                    .unwrap_or_else(Vec::new);

                eggplant_commands.push(EggplantCommandWithSource {
                    command: EggplantCommand::DslType(DslType {
                        name: normalize_identifier(name),
                        variants: dsl_variants,
                    }),
                    source_file: source_file.clone(),
                    source_line: Some(line_counter),
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
        source_line: Some(line_counter),
    });
    eggplant_commands.push(EggplantCommandWithSource {
        command: EggplantCommand::RunRuleset("MyTx".to_string(), "Sat".to_string()),
        source_file: source_file.clone(),
        source_line: Some(line_counter),
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

/// Infer type from expression context
fn infer_type_from_expr(expr: &Expr) -> String {
    match expr {
        Expr::Call(_, func_name, _) => normalize_identifier(func_name),
        Expr::Var(_, _) => "Expr".to_string(), // Default type for variables
        Expr::Lit(_, lit) => match lit {
            Literal::Int(_) => "i64".to_string(),
            Literal::Float(_) => "f64".to_string(),
            Literal::String(_) => "String".to_string(),
            Literal::Bool(_) => "bool".to_string(),
            Literal::Unit => "()".to_string(),
        },
    }
}

/// Generate better pattern query with type inference
fn generate_pattern_query(lhs: &Expr, rule_name: &str) -> (PatternVars, String) {
    let mut variables = Vec::new();
    let mut pattern_query_parts = Vec::new();
    let mut node_counter = 0;

    // Extract variables with better type inference
    extract_variables_with_types(lhs, &mut variables, &mut pattern_query_parts, &mut node_counter);

    // Create pattern variables struct
    let pattern_vars = PatternVars {
        name: format!("{}Pat", rule_name),
        variables: variables.clone(),
    };

    // Generate improved pattern query
    let pattern_query = if pattern_query_parts.is_empty() {
        format!("// TODO: implement pattern query for {}", rule_name)
    } else {
        let mut query = pattern_query_parts.join("\n");
        query.push_str(&format!("\n{}Pat::new(", rule_name));

        // Add variable references
        let var_refs: Vec<String> = variables.iter().map(|v| v.name.clone()).collect();
        query.push_str(&var_refs.join(", "));
        query.push(')');
        query
    };

    (pattern_vars, pattern_query)
}

/// Extract variables with type inference
fn extract_variables_with_types(expr: &Expr, variables: &mut Vec<PatternVariable>, pattern_query_parts: &mut Vec<String>, node_counter: &mut usize) {
    match expr {
        Expr::Var(_, var_name) => {
            // Variable reference - add to pattern variables with inferred type
            if !variables.iter().any(|v| v.name == *var_name) {
                variables.push(PatternVariable {
                    name: var_name.clone(),
                    var_type: infer_type_from_expr(expr),
                });
                pattern_query_parts.push(format!("let {} = Expr::query_leaf();", var_name));
            }
        }
        Expr::Call(_, func_name, args) => {
            // Function call - extract variables from arguments
            for arg in args {
                extract_variables_with_types(arg, variables, pattern_query_parts, node_counter);
            }

            // Add pattern query for the function call with better naming
            let arg_refs: Vec<String> = args.iter().map(|arg| {
                match arg {
                    Expr::Var(_, name) => format!("&{}", name),
                    Expr::Call(_, sub_func, _) => {
                        // For nested calls, use the sub-node with counter
                        *node_counter += 1;
                        format!("&{}_node{}", normalize_identifier(&sub_func.to_lowercase()), node_counter)
                    }
                    Expr::Lit(_, lit) => {
                        // For literals, insert them directly
                        match lit {
                            Literal::Int(i) => i.to_string(),
                            Literal::Float(f) => f.0.to_string(),
                            Literal::String(s) => format!("\"{}\"", s),
                            Literal::Bool(b) => b.to_string(),
                            Literal::Unit => "()".to_string(),
                        }
                    }
                }
            }).collect();

            *node_counter += 1;
            pattern_query_parts.push(format!("let {}_node{} = {}::query({});",
                normalize_identifier(&func_name.to_lowercase()), node_counter,
                normalize_identifier(func_name), arg_refs.join(", ")));
        }
        Expr::Lit(_, _) => {
            // Literal - no variables to extract
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

/// Analyze a rewrite pattern to extract pattern variables and generate pattern query
fn analyze_rewrite_pattern(lhs: &Expr, rule_name: &str) -> (PatternVars, String) {
    generate_pattern_query(lhs, rule_name)
}

/// Extract variables from an expression and build pattern query
fn extract_variables_from_expr(expr: &Expr, variables: &mut Vec<PatternVariable>, pattern_query_parts: &mut Vec<String>, depth: usize) {
    match expr {
        Expr::Var(_, var_name) => {
            // Variable reference - add to pattern variables
            if !variables.iter().any(|v| v.name == *var_name) {
                variables.push(PatternVariable {
                    name: var_name.clone(),
                    var_type: "Expr".to_string(), // Default type
                });
                pattern_query_parts.push(format!("let {} = Expr::query_leaf();", var_name));
            }
        }
        Expr::Call(_, func_name, args) => {
            // Function call - extract variables from arguments
            for arg in args {
                extract_variables_from_expr(arg, variables, pattern_query_parts, depth + 1);
            }

            // Add pattern query for the function call
            let arg_refs: Vec<String> = args.iter().map(|arg| {
                match arg {
                    Expr::Var(_, name) => format!("&{}", name),
                    Expr::Call(_, sub_func, _) => {
                        // For nested calls, use the sub-node
                        format!("&{}_node", normalize_identifier(&sub_func.to_lowercase()))
                    }
                    Expr::Lit(_, lit) => {
                        // For literals, insert them directly
                        match lit {
                            Literal::Int(i) => i.to_string(),
                            Literal::Float(f) => f.0.to_string(),
                            Literal::String(s) => format!("\"{}\"", s),
                            Literal::Bool(b) => b.to_string(),
                            Literal::Unit => "()".to_string(),
                        }
                    }
                }
            }).collect();

            pattern_query_parts.push(format!("let {}_node = {}::query({});",
                normalize_identifier(&func_name.to_lowercase()), normalize_identifier(func_name), arg_refs.join(", ")));
        }
        Expr::Lit(_, _) => {
            // Literal - no variables to extract
        }
    }
}

/// Generate rewrite action from the right-hand side pattern
fn generate_rewrite_action(rhs: &Expr, rule_name: &str) -> String {
    match rhs {
        Expr::Var(_, var_name) => {
            // Simple variable reference
            format!("ctx.union(pat.{}_node1, pat.{});", rule_name.to_lowercase(), var_name)
        }
        Expr::Call(_, func_name, args) => {
            // Function call - generate action to create new node
            let mut node_counter = 1;
            let arg_refs: Vec<String> = args.iter().map(|arg| {
                match arg {
                    Expr::Var(_, name) => format!("pat.{}", name),
                    Expr::Call(_, sub_func, _) => {
                        // For nested calls, use the sub-node with counter
                        node_counter += 1;
                        format!("pat.{}_node{}", normalize_identifier(&sub_func.to_lowercase()), node_counter)
                    }
                    Expr::Lit(_, lit) => {
                        match lit {
                            Literal::Int(i) => i.to_string(),
                            Literal::Float(f) => f.0.to_string(),
                            Literal::String(s) => format!("\"{}\"", s),
                            Literal::Bool(b) => b.to_string(),
                            Literal::Unit => "()".to_string(),
                        }
                    }
                }
            }).collect();

            format!("let result = {}::new({});\nctx.union(pat.{}_node1, result);",
                normalize_identifier(func_name), arg_refs.join(", "), normalize_identifier(&rule_name.to_lowercase()))
        }
        Expr::Lit(_, lit) => {
            // Literal value
            let value = match lit {
                Literal::Int(i) => i.to_string(),
                Literal::Float(f) => f.0.to_string(),
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Bool(b) => b.to_string(),
                Literal::Unit => "()".to_string(),
            };
            format!("let result = ctx.insert_literal({});\nctx.union(pat.{}_node1, result);", value, normalize_identifier(&rule_name.to_lowercase()))
        }
    }
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
                expr: Expr::Call(Span::new(None, 1, 1), "add".to_string(), vec![
                    Expr::Call(Span::new(None, 1, 1), "Num".to_string(), vec![
                        Expr::Lit(Span::new(None, 1, 1), Literal::Int(1))
                    ]),
                    Expr::Call(Span::new(None, 1, 1), "Num".to_string(), vec![
                        Expr::Lit(Span::new(None, 1, 1), Literal::Int(2))
                    ])
                ]),
                expected: Expr::Call(Span::new(None, 1, 1), "Num".to_string(), vec![
                    Expr::Lit(Span::new(None, 1, 1), Literal::Int(3))
                ]),
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