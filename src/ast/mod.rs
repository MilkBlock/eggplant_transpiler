pub mod expr;
pub mod parse;

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    pub file: Option<String>,
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(file: Option<String>, line: usize, col: usize) -> Self {
        Self { file, line, col }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.file {
            Some(filename) => write!(f, "{}:{}:{}", filename, self.line, self.col),
            None => write!(f, "{}:{}", self.line, self.col),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError(pub Span, pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: parse error: {}", self.0, self.1)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Int(i64),
    Float(ordered_float::OrderedFloat<f64>),
    String(String),
    Bool(bool),
    Unit,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Literal::Int(i) => Display::fmt(i, f),
            Literal::Float(n) => write!(f, "{}", n.0),
            Literal::Bool(b) => Display::fmt(b, f),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Unit => write!(f, "()"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericExpr<Head, Leaf> {
    Lit(Span, Literal),
    Var(Span, Leaf),
    Call(Span, Head, Vec<Self>),
}

impl<Head: Display, Leaf: Display> Display for GenericExpr<Head, Leaf> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GenericExpr::Lit(_ann, lit) => write!(f, "{}", lit),
            GenericExpr::Var(_ann, var) => write!(f, "{}", var),
            GenericExpr::Call(_ann, op, children) => {
                write!(f, "({} {})", op, ListDisplay(children, " "))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericFact<Head, Leaf> {
    Eq(Span, GenericExpr<Head, Leaf>, GenericExpr<Head, Leaf>),
    Fact(GenericExpr<Head, Leaf>),
}

impl<Head: Display, Leaf: Display> Display for GenericFact<Head, Leaf> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GenericFact::Eq(_, e1, e2) => write!(f, "(= {} {})", e1, e2),
            GenericFact::Fact(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericAction<Head, Leaf> {
    Let(Span, Leaf, GenericExpr<Head, Leaf>),
    Set(Span, Head, Vec<GenericExpr<Head, Leaf>>, GenericExpr<Head, Leaf>),
    Union(Span, GenericExpr<Head, Leaf>, GenericExpr<Head, Leaf>),
    Expr(Span, GenericExpr<Head, Leaf>),
}

impl<Head: Display, Leaf: Display> Display for GenericAction<Head, Leaf> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GenericAction::Let(_ann, lhs, rhs) => write!(f, "(let {} {})", lhs, rhs),
            GenericAction::Set(_ann, lhs, args, rhs) => {
                write!(f, "(set ({} {}) {})", lhs, ListDisplay(args, " "), rhs)
            }
            GenericAction::Union(_ann, lhs, rhs) => write!(f, "(union {} {})", lhs, rhs),
            GenericAction::Expr(_ann, e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericRule<Head, Leaf> {
    pub span: Span,
    pub head: Vec<GenericAction<Head, Leaf>>,
    pub body: Vec<GenericFact<Head, Leaf>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Schema {
    pub input: Vec<String>,
    pub output: String,
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {}", ListDisplay(&self.input, " "), self.output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub span: Span,
    pub name: String,
    pub types: Vec<String>,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", self.name)?;
        if !self.types.is_empty() {
            write!(f, " {}", ListDisplay(&self.types, " "))?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone)]
pub enum GenericCommand<Head, Leaf> {
    Sort(Span, String, Option<(String, Vec<GenericExpr<Head, Leaf>>)>),
    Datatype {
        span: Span,
        name: String,
        variants: Vec<Variant>,
    },
    Function {
        span: Span,
        name: String,
        schema: Schema,
        merge: Option<GenericExpr<Head, Leaf>>,
    },
    Constructor {
        span: Span,
        name: String,
        schema: Schema,
    },
    Relation {
        span: Span,
        name: String,
        inputs: Vec<String>,
    },
    AddRuleset(Span, String),
    Rule {
        name: String,
        ruleset: String,
        rule: GenericRule<Head, Leaf>,
    },
    Rewrite(String, GenericRewrite<Head, Leaf>, bool),
    BiRewrite(String, GenericRewrite<Head, Leaf>),
    Action(GenericAction<Head, Leaf>),
    Check(Span, Vec<GenericFact<Head, Leaf>>),
    Push(usize),
    Pop(Span, usize),
    PrintFunction(Span, String, Option<usize>, Option<String>),
    Input {
        span: Span,
        name: String,
        file: String,
    },
    Output {
        span: Span,
        file: String,
        exprs: Vec<GenericExpr<Head, Leaf>>,
    },
    Include(Span, String),
    Fail(Span, Box<GenericCommand<Head, Leaf>>),
}

impl<Head: Display, Leaf: Display> Display for GenericCommand<Head, Leaf> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GenericCommand::Sort(_span, name, None) => write!(f, "(sort {})", name),
            GenericCommand::Sort(_span, name, Some((name2, args))) => {
                write!(f, "(sort {} ({} {}))", name, name2, ListDisplay(args, " "))
            }
            GenericCommand::Datatype {
                span: _,
                name,
                variants,
            } => write!(f, "(datatype {} {})", name, ListDisplay(variants, " ")),
            GenericCommand::Function {
                span: _,
                name,
                schema,
                merge,
            } => {
                write!(f, "(function {} {}", name, schema)?;
                if let Some(merge) = merge {
                    write!(f, " :merge {}", merge)?;
                }
                write!(f, ")")
            }
            GenericCommand::Constructor {
                span: _,
                name,
                schema,
            } => write!(f, "(constructor {} {})", name, schema),
            GenericCommand::Relation {
                span: _,
                name,
                inputs,
            } => write!(f, "(relation {} ({}))", name, ListDisplay(inputs, " ")),
            GenericCommand::AddRuleset(_span, name) => write!(f, "(ruleset {})", name),
            GenericCommand::Rule {
                ruleset,
                name,
                rule,
            } => {
                write!(f, "(rule (")?;
                for (i, fact) in rule.body.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", fact)?;
                }
                write!(f, ") (")?;
                for (i, action) in rule.head.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", action)?;
                }
                write!(f, ") :ruleset {} :name \"{}\")", ruleset, name)
            }
            GenericCommand::Rewrite(name, rewrite, subsume) => {
                write!(f, "(rewrite {} {}", rewrite.lhs, rewrite.rhs)?;
                if *subsume {
                    write!(f, " :subsume")?;
                }
                if !rewrite.conditions.is_empty() {
                    write!(f, " :when ({})", ListDisplay(&rewrite.conditions, " "))?;
                }
                write!(f, " :ruleset {})", name)
            }
            GenericCommand::BiRewrite(name, rewrite) => {
                write!(f, "(birewrite {} {}", rewrite.lhs, rewrite.rhs)?;
                if !rewrite.conditions.is_empty() {
                    write!(f, " :when ({})", ListDisplay(&rewrite.conditions, " "))?;
                }
                write!(f, " :ruleset {})", name)
            }
            GenericCommand::Action(a) => write!(f, "{}", a),
            GenericCommand::Check(_ann, facts) => {
                write!(f, "(check {})", ListDisplay(facts, " "))
            }
            GenericCommand::Push(n) => write!(f, "(push {})", n),
            GenericCommand::Pop(_span, n) => write!(f, "(pop {})", n),
            GenericCommand::PrintFunction(_span, name, n, file) => {
                write!(f, "(print-function {}", name)?;
                if let Some(n) = n {
                    write!(f, " {}", n)?;
                }
                if let Some(file) = file {
                    write!(f, " :file \"{}\"", file)?;
                }
                write!(f, ")")
            }
            GenericCommand::Input {
                span: _,
                name,
                file,
            } => write!(f, "(input {} \"{}\")", name, file),
            GenericCommand::Output {
                span: _,
                file,
                exprs,
            } => write!(f, "(output \"{}\" {})", file, ListDisplay(exprs, " ")),
            GenericCommand::Include(_span, file) => write!(f, "(include \"{}\")", file),
            GenericCommand::Fail(_span, cmd) => write!(f, "(fail {})", cmd),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericRewrite<Head, Leaf> {
    pub span: Span,
    pub lhs: GenericExpr<Head, Leaf>,
    pub rhs: GenericExpr<Head, Leaf>,
    pub conditions: Vec<GenericFact<Head, Leaf>>,
}

pub type Expr = GenericExpr<String, String>;
pub type Fact = GenericFact<String, String>;
pub type Action = GenericAction<String, String>;
pub type Rule = GenericRule<String, String>;
pub type Command = GenericCommand<String, String>;
pub type Rewrite = GenericRewrite<String, String>;

struct ListDisplay<'a, T>(&'a [T], &'a str);

impl<'a, T: Display> Display for ListDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "{}", self.1)?;
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}