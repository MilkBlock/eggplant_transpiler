use crate::ast::*;

pub struct RustCodeGenerator {
    output: String,
    indent_level: usize,
}

impl RustCodeGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
        }
    }

    pub fn generate(&mut self, commands: &[Command]) -> String {
        self.output.clear();

        self.add_line("use egglog::prelude::*;");
        self.add_line("");
        self.add_line("fn main() {");
        self.indent();

        for command in commands {
            self.generate_command(command);
        }

        self.dedent();
        self.add_line("}");

        self.output.clone()
    }

    fn generate_command(&mut self, command: &Command) {
        match command {
            Command::Sort(_, name, None) => {
                self.add_line(&format!("let {} = egglog::Sort::new(\"{}\");", name, name));
            }
            Command::Sort(_, name, Some((container, args))) => {
                self.add_line(&format!("let {} = {}::new({});", name, container,
                    args.iter().map(|arg| self.expr_to_string(arg)).collect::<Vec<_>>().join(", ")));
            }
            Command::Datatype { name, variants, .. } => {
                self.add_line(&format!("let {} = egglog::Datatype::new(\"{}\");", name, name));
                for variant in variants {
                    self.add_line(&format!("let {} = {}.add_variant(\"{}\", vec![]);",
                        variant.name, name, variant.name));
                }
            }
            Command::Function { name, schema, .. } => {
                let inputs = schema.input.iter().map(|s| format!("\"{}\".into()", s)).collect::<Vec<_>>().join(", ");
                self.add_line(&format!("let {} = egglog::Function::new(\"{}\", vec![{}], \"{}\".into());",
                    name, name, inputs, schema.output));
            }
            Command::Constructor { name, schema, .. } => {
                let inputs = schema.input.iter().map(|s| format!("\"{}\".into()", s)).collect::<Vec<_>>().join(", ");
                self.add_line(&format!("let {} = egglog::Constructor::new(\"{}\", vec![{}], \"{}\".into());",
                    name, name, inputs, schema.output));
            }
            Command::Relation { name, inputs, .. } => {
                let input_str = inputs.iter().map(|s| format!("\"{}\".into()", s)).collect::<Vec<_>>().join(", ");
                self.add_line(&format!("let {} = egglog::Relation::new(\"{}\", vec![{}]);",
                    name, name, input_str));
            }
            Command::AddRuleset(_, name) => {
                self.add_line(&format!("let {} = egglog::Ruleset::new(\"{}\");", name, name));
            }
            Command::Rule { name, ruleset, rule } => {
                self.add_line(&format!("let {}_rule = {}::add_rule(\"{}\", |egraph| {{", ruleset, ruleset, name));
                self.indent();

                // Generate body facts
                for fact in &rule.body {
                    self.generate_fact(fact);
                }

                // Generate head actions
                for action in &rule.head {
                    self.generate_action(action);
                }

                self.dedent();
                self.add_line("});");
            }
            Command::Rewrite(name, rewrite, _) => {
                self.add_line(&format!("let {}_rewrite = {}::add_rewrite(\"{}\", |egraph| {{", name, name, name));
                self.indent();

                self.add_line(&format!("let lhs = {};", self.expr_to_string(&rewrite.lhs)));
                self.add_line(&format!("let rhs = {};", self.expr_to_string(&rewrite.rhs)));

                for condition in &rewrite.conditions {
                    self.generate_fact(condition);
                }

                self.add_line("egraph.union(lhs, rhs);");

                self.dedent();
                self.add_line("});");
            }
            Command::BiRewrite(name, rewrite) => {
                self.add_line(&format!("let {}_birewrite = {}::add_birewrite(\"{}\", |egraph| {{", name, name, name));
                self.indent();

                self.add_line(&format!("let lhs = {};", self.expr_to_string(&rewrite.lhs)));
                self.add_line(&format!("let rhs = {};", self.expr_to_string(&rewrite.rhs)));

                for condition in &rewrite.conditions {
                    self.generate_fact(condition);
                }

                self.add_line("egraph.union(lhs, rhs);");

                self.dedent();
                self.add_line("});");
            }
            Command::Action(action) => {
                self.generate_action(action);
            }
            Command::Check(_, facts) => {
                self.add_line("{");
                self.indent();
                for fact in facts {
                    self.generate_fact(fact);
                }
                self.add_line("assert!(egraph.check_facts());");
                self.dedent();
                self.add_line("}");
            }
            Command::Push(n) => {
                self.add_line(&format!("egraph.push({});", n));
            }
            Command::Pop(_, n) => {
                self.add_line(&format!("egraph.pop({});", n));
            }
            Command::PrintFunction(_, name, n, file) => {
                if let Some(file) = file {
                    self.add_line(&format!("{}.print_to_file(\"{}\");", name, file));
                } else if let Some(n) = n {
                    self.add_line(&format!("{}.print_first_rows({});", name, n));
                } else {
                    self.add_line(&format!("{}.print();", name));
                }
            }
            Command::Input { name, file, .. } => {
                self.add_line(&format!("{}.input_from_file(\"{}\");", name, file));
            }
            Command::Output { file, exprs, .. } => {
                let expr_strs: Vec<String> = exprs.iter().map(|e| self.expr_to_string(e)).collect();
                self.add_line(&format!("egraph.output_to_file(\"{}\", vec![{}]);", file, expr_strs.join(", ")));
            }
            Command::Include(_, file) => {
                self.add_line(&format!("// include \"{}\"", file));
            }
            Command::Fail(_, cmd) => {
                self.add_line(&format!("// fail {}", cmd));
            }
            _ => {
                self.add_line(&format!("// Unsupported command: {}", command));
            }
        }
    }

    fn generate_fact(&mut self, fact: &Fact) {
        match fact {
            Fact::Eq(_, e1, e2) => {
                self.add_line(&format!("let eq_check = egraph.check_equal({}, {});",
                    self.expr_to_string(e1), self.expr_to_string(e2)));
            }
            Fact::Fact(expr) => {
                self.add_line(&format!("let fact_check = egraph.check_fact({});",
                    self.expr_to_string(expr)));
            }
        }
    }

    fn generate_action(&mut self, action: &Action) {
        match action {
            Action::Let(_, var, expr) => {
                self.add_line(&format!("let {} = {};", var, self.expr_to_string(expr)));
            }
            Action::Set(_, func, args, value) => {
                let arg_str = args.iter().map(|a| self.expr_to_string(a)).collect::<Vec<_>>().join(", ");
                self.add_line(&format!("{}.set(egraph, vec![{}], {});",
                    func, arg_str, self.expr_to_string(value)));
            }
            Action::Union(_, e1, e2) => {
                self.add_line(&format!("egraph.union({}, {});",
                    self.expr_to_string(e1), self.expr_to_string(e2)));
            }
            Action::Expr(_, expr) => {
                self.add_line(&format!("let _ = {};", self.expr_to_string(expr)));
            }
        }
    }

    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::Lit(_, lit) => match lit {
                Literal::Int(i) => i.to_string(),
                Literal::Float(f) => f.0.to_string(),
                Literal::String(s) => format!("\"{}\".into()", s),
                Literal::Bool(b) => b.to_string(),
                Literal::Unit => "()".to_string(),
            },
            Expr::Var(_, var) => var.clone(),
            Expr::Call(_, func, args) => {
                let arg_str = args.iter().map(|a| self.expr_to_string(a)).collect::<Vec<_>>().join(", ");
                format!("{}({})", func, arg_str)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::parse::Parser;

    #[test]
    fn test_basic_codegen() {
        let program = r#"
            (datatype Math)
            (constructor Num (i64) Math)
            (let x (Num 42))
        "#;

        let mut parser = Parser::default();
        let commands = parser.get_program_from_string(None, program).unwrap();

        let mut codegen = RustCodeGenerator::new();
        let rust_code = codegen.generate(&commands);

        assert!(rust_code.contains("use egglog::prelude::*;"));
        assert!(rust_code.contains("let Math = egglog::Datatype::new"));
        assert!(rust_code.contains("let Num = egglog::Constructor::new"));
        assert!(rust_code.contains("let x = Num(42);"));
    }
}