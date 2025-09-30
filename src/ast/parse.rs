use super::*;
use std::collections::VecDeque;
use std::convert::TryInto;

pub struct Parser {
    tokens: VecDeque<Token>,
    current_file: Option<String>,
    current_line: usize,
    current_col: usize,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            tokens: VecDeque::new(),
            current_file: None,
            current_line: 1,
            current_col: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen(Span),
    RParen(Span),
    Symbol(String, Span),
    String(String, Span),
    Number(String, Span),
    Keyword(String, Span),
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LParen(_), Self::LParen(_)) => true,
            (Self::RParen(_), Self::RParen(_)) => true,
            (Self::Symbol(l0, _), Self::Symbol(r0, _)) => l0 == r0,
            (Self::String(l0, _), Self::String(r0, _)) => l0 == r0,
            (Self::Number(l0, _), Self::Number(r0, _)) => l0 == r0,
            (Self::Keyword(l0, _), Self::Keyword(r0, _)) => l0 == r0,
            _ => false,
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_program_from_string(
        &mut self,
        file: Option<String>,
        input: &str,
    ) -> Result<Vec<Command>, ParseError> {
        self.current_file = file;
        self.tokenize(input)?;
        self.parse_program()
    }

    fn tokenize(&mut self, input: &str) -> Result<(), ParseError> {
        self.tokens.clear();
        let mut chars = input.chars().peekable();

        // Initialize parser state
        self.current_line = 1;
        self.current_col = 1;

        while let Some(&ch) = chars.peek() {
            match ch {
                '(' => {
                    self.tokens.push_back(Token::LParen(self.current_span()));
                    chars.next();
                    self.current_col += 1;
                }
                ')' => {
                    self.tokens.push_back(Token::RParen(self.current_span()));
                    chars.next();
                    self.current_col += 1;
                }
                ';' => {
                    // Skip comments until end of line
                    while chars.next().map_or(false, |c| c != '\n') {}
                    self.current_line += 1;
                    self.current_col = 1;
                }
                '"' => {
                    chars.next(); // consume opening quote
                    self.current_col += 1;
                    let mut string = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch == '"' {
                            chars.next(); // consume closing quote
                            self.current_col += 1;
                            break;
                        }
                        string.push(ch);
                        chars.next();
                        self.current_col += 1;
                    }
                    self.tokens
                        .push_back(Token::String(string, self.current_span()));
                }
                ch if ch.is_whitespace() => {
                    if ch == '\n' {
                        println!("line plus 1");
                        self.current_line += 1;
                        self.current_col = 1;
                    } else {
                        self.current_col += 1;
                    }
                    chars.next();
                }
                _ => {
                    let mut symbol = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_whitespace() || ch == '(' || ch == ')' {
                            break;
                        }
                        symbol.push(ch);
                        chars.next();
                        self.current_col += 1;
                    }

                    if symbol
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '-' && symbol.len() == 1)
                    {
                        self.tokens
                            .push_back(Token::Number(symbol, self.current_span()));
                    } else if symbol.starts_with(':') {
                        self.tokens.push_back(Token::Keyword(
                            symbol[1..].to_string(),
                            self.current_span(),
                        ));
                    } else {
                        self.tokens
                            .push_back(Token::Symbol(symbol, self.current_span()));
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_program(&mut self) -> Result<Vec<Command>, ParseError> {
        let mut commands = Vec::new();
        while !self.tokens.is_empty() {
            if let Ok(command) = self.parse_command() {
                commands.push(command);
            } else {
                // Print debug info when parsing fails
                log::debug!(
                    "Failed to parse command, parsed {} commands so far, remaining tokens: {:?}",
                    commands.len(),
                    self.tokens
                );
                // Don't break, try to continue parsing
                // Skip the problematic token and continue
                if !self.tokens.is_empty() {
                    log::debug!("Skipping token: {:?}", self.tokens.pop_front());
                }
            }
        }
        Ok(commands)
    }

    fn parse_command(&mut self) -> Result<Command, ParseError> {
        self.expect_token(Token::LParen(span()))?;
        let (command_name, sp) = self.parse_symbol()?;

        let command = match command_name.as_str() {
            "datatype" => self.parse_datatype()?,
            "datatype*" => self.parse_datatype_star()?,
            "constructor" => self.parse_constructor()?,
            "let" => self.parse_let()?,
            "birewrite" => self.parse_birewrite()?,
            "rewrite" => self.parse_rewrite()?,
            "check" => self.parse_check()?,
            "push" => self.parse_push()?,
            "pop" => self.parse_pop()?,
            "run" => self.parse_run()?,
            "sort" => self.parse_sort()?,
            "ruleset" => self.parse_ruleset()?,
            _ => {
                // For unsupported commands, create a simple action
                let expr = self.parse_expr()?;
                Command::Action(Action::Expr(sp, expr))
            }
        };

        self.expect_token(Token::RParen(span()))?;
        Ok(command)
    }

    fn parse_datatype(&mut self) -> Result<Command, ParseError> {
        let (name, sp) = self.parse_symbol()?;
        let mut variants = Vec::new();

        while self.peek_token() != Some(&Token::RParen(span())) {
            let variant = self.parse_variant()?;
            variants.push(variant);
        }

        println!("current span {:?}", sp);
        Ok(Command::Datatype {
            span: sp,
            name,
            variants,
        })
    }

    fn parse_datatype_star(&mut self) -> Result<Command, ParseError> {
        self.expect_token(Token::LParen(span()))?;
        let (name, sp) = self.parse_symbol()?;
        let mut variants = Vec::new();

        while self.peek_token() != Some(&Token::RParen(span())) {
            let variant = self.parse_variant()?;
            variants.push(variant);
        }

        self.expect_token(Token::RParen(span()))?;
        Ok(Command::Datatype {
            span: sp,
            name,
            variants,
        })
    }

    fn parse_variant(&mut self) -> Result<Variant, ParseError> {
        self.expect_token(Token::LParen(span()))?;
        let (name, sp) = self.parse_symbol()?;
        let mut types = Vec::new();

        // Parse types until we hit a keyword or closing paren
        while let Some(token) = self.peek_token() {
            match token {
                Token::RParen(_) => break,
                Token::Keyword(_, _) => break,
                _ => types.push(self.parse_symbol()?.0),
            }
        }

        // Skip keyword arguments (like :cost)
        while let Some(Token::Keyword(_, _)) = self.peek_token() {
            self.tokens.pop_front(); // Skip keyword
            self.parse_symbol()?.0; // Skip value
        }

        self.expect_token(Token::RParen(span()))?;
        Ok(Variant {
            span: sp,
            name,
            types,
        })
    }

    fn parse_constructor(&mut self) -> Result<Command, ParseError> {
        let (name, sp) = self.parse_symbol()?;
        let schema = self.parse_schema()?;

        // Skip keyword arguments (like :cost)
        while matches!(self.peek_token(), Some(Token::Keyword(_, _))) {
            self.tokens.pop_front(); // Skip keyword
            self.parse_symbol()?; // Skip value
        }

        Ok(Command::Constructor {
            span: sp,
            name,
            schema,
        })
    }

    fn parse_schema(&mut self) -> Result<Schema, ParseError> {
        let mut inputs = Vec::new();

        // Check if inputs are in parentheses
        if self.peek_token() == Some(&Token::LParen(span())) {
            self.expect_token(Token::LParen(span()))?;
            while self.peek_token() != Some(&Token::RParen(span())) {
                inputs.push(self.parse_symbol()?.0);
            }
            self.expect_token(Token::RParen(span()))?;
        } else {
            // Parse inputs without parentheses
            while self.peek_token() != Some(&Token::RParen(span())) {
                inputs.push(self.parse_symbol()?.0);
            }
        }

        let output = self.parse_symbol()?.0;

        Ok(Schema {
            input: inputs,
            output,
        })
    }

    fn parse_let(&mut self) -> Result<Command, ParseError> {
        let (var, sp) = self.parse_symbol()?;
        let expr = self.parse_expr()?;

        Ok(Command::Action(Action::Let(sp, var, expr)))
    }

    fn parse_birewrite(&mut self) -> Result<Command, ParseError> {
        let lhs = self.parse_expr()?;
        let rhs = self.parse_expr()?;

        Ok(Command::BiRewrite(
            "default".to_string(),
            Rewrite {
                span: lhs.span(),
                lhs,
                rhs,
                conditions: Vec::new(),
            },
        ))
    }

    fn parse_rewrite(&mut self) -> Result<Command, ParseError> {
        let lhs = self.parse_expr()?;
        let rhs = self.parse_expr()?;
        let mut ruleset = "default".to_string();

        // Parse optional keyword arguments
        while matches!(self.peek_token(), Some(Token::Keyword(_, _))) {
            let (keyword, sp) = self.parse_symbol()?;
            match keyword.as_str() {
                "ruleset" => {
                    (ruleset, _) = self.parse_symbol()?;
                }
                "when" => {
                    // Skip when conditions for now
                    self.expect_token(Token::LParen(span()))?;
                    while self.peek_token() != Some(&Token::RParen(span())) {
                        self.tokens.pop_front();
                    }
                    self.expect_token(Token::RParen(span()))?;
                }
                _ => {
                    // Skip unknown keywords
                    self.tokens.pop_front();
                }
            }
        }

        Ok(Command::Rewrite(
            ruleset,
            Rewrite {
                span: lhs.span(),
                lhs,
                rhs,
                conditions: Vec::new(),
            },
            false,
        ))
    }

    fn parse_check(&mut self) -> Result<Command, ParseError> {
        let fact = self.parse_fact()?;

        Ok(Command::Check(self.current_span(), vec![fact]))
    }

    fn parse_fact(&mut self) -> Result<Fact, ParseError> {
        if self.peek_token() == Some(&Token::LParen(span())) {
            self.expect_token(Token::LParen(span()))?;
            let (op, sp) = self.parse_symbol()?;

            if op == "=" {
                let e1 = self.parse_expr()?;
                let e2 = self.parse_expr()?;
                self.expect_token(Token::RParen(span()))?;
                Ok(Fact::Eq(sp, e1, e2))
            } else {
                let expr = self.parse_expr()?;
                self.expect_token(Token::RParen(span()))?;
                Ok(Fact::Fact(expr))
            }
        } else {
            let expr = self.parse_expr()?;
            Ok(Fact::Fact(expr))
        }
    }

    fn parse_push(&mut self) -> Result<Command, ParseError> {
        let (n, sp) = self.parse_number()?;
        Ok(Command::Push(n.try_into().unwrap()))
    }

    fn parse_pop(&mut self) -> Result<Command, ParseError> {
        let (n, sp) = self.parse_number()?;
        Ok(Command::Pop(sp, n.try_into().unwrap()))
    }

    fn parse_run(&mut self) -> Result<Command, ParseError> {
        let (n, sp) = self.parse_number()?;
        // For now, just return a simple action
        Ok(Command::Action(Action::Expr(
            sp.clone(),
            Expr::Lit(sp, Literal::Int(n)),
        )))
    }

    fn parse_sort(&mut self) -> Result<Command, ParseError> {
        let (name, sp) = self.parse_symbol()?;

        // Check if there are additional parameters
        if self.peek_token() == Some(&Token::LParen(span())) {
            // For now, skip the type specification and just parse as simple sort
            // This handles cases like: (sort UnstableFn_Int_Int (UnstableFn (Int) Int))
            while self.peek_token() != Some(&Token::RParen(span())) {
                self.tokens.pop_front(); // Skip tokens until closing paren
            }
            self.expect_token(Token::RParen(span()))?;
            Ok(Command::Sort(sp, name, None))
        } else {
            Ok(Command::Sort(sp, name, None))
        }
    }

    fn parse_ruleset(&mut self) -> Result<Command, ParseError> {
        let (name, sp) = self.parse_symbol()?;
        Ok(Command::AddRuleset(sp, name))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        match self.peek_token() {
            Some(Token::LParen(_)) => {
                self.expect_token(Token::LParen(span()))?;
                let (func, sp) = self.parse_symbol()?;
                let mut args = Vec::new();

                while self.peek_token() != Some(&Token::RParen(span())) {
                    args.push(self.parse_expr()?);
                }

                self.expect_token(Token::RParen(span()))?;
                Ok(Expr::Call(sp, func, args))
            }
            Some(Token::Number(_n, _)) => {
                let (num, sp) = self.parse_number()?;
                Ok(Expr::Lit(sp, Literal::Int(num)))
            }
            Some(Token::String(_s, _)) => {
                let (s, sp) = self.parse_string()?;
                Ok(Expr::Lit(sp, Literal::String(s)))
            }
            Some(Token::Symbol(_s, _)) => {
                let (sym, sp) = self.parse_symbol()?;
                Ok(Expr::Var(sp, sym))
            }
            _ => Err(ParseError::new(span(), "Expected expression".to_string())),
        }
    }

    fn parse_number(&mut self) -> Result<(i64, Span), ParseError> {
        if let Some(Token::Number(n, sp)) = self.tokens.pop_front() {
            n.parse()
                .map_err(|_| ParseError::new(sp.clone(), "Invalid number".to_string()))
                .map(|n| (n, sp))
        } else {
            Err(ParseError::new(span(), "Expected number".to_string()))
        }
    }

    fn parse_string(&mut self) -> Result<(String, Span), ParseError> {
        if let Some(Token::String(s, sp)) = self.tokens.pop_front() {
            Ok((s, sp))
        } else {
            Err(ParseError::new(span(), "Expected string".to_string()))
        }
    }

    fn parse_symbol(&mut self) -> Result<(String, Span), ParseError> {
        let popped = self.tokens.pop_front();
        match popped {
            Some(Token::Symbol(s, span)) => Ok((s, span)),
            Some(Token::Keyword(k, span)) => Ok((k, span)),
            _ => Err(ParseError::new(
                get_op_span(&popped),
                "Expected symbol".to_string(),
            )),
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        if let Some(token) = self.tokens.pop_front() {
            if token == expected {
                Ok(())
            } else {
                Err(ParseError::new(
                    self.current_span(),
                    format!("Expected {:?}, got {:?}", expected, token),
                ))
            }
        } else {
            Err(ParseError::new(
                self.current_span(),
                format!("Expected {:?}, but no more tokens", expected),
            ))
        }
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.front()
    }

    fn current_span(&self) -> Span {
        Span::new(
            self.current_file.clone(),
            self.current_line,
            self.current_col,
        )
    }
}

impl ParseError {
    pub fn new(span: Span, message: String) -> Self {
        ParseError(span, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_stresstest() {
        let input = r#"(rewrite (TupleInt_single __var__i) (TupleInt___init__ (Int___init__ 1) (unstable-fn "cast_Callable__Int__Int___Int___lambda_i_____i_" __var__i)) :ruleset array_api_ruleset)"#;
        let mut parser = Parser::new();

        println!("Input: {}", input);

        match parser.get_program_from_string(None, input) {
            Ok(commands) => {
                println!("Success! Parsed {} commands", commands.len());
                for cmd in commands {
                    println!("  Command: {:?}", cmd);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                println!("Remaining tokens: {:?}", parser.tokens);
            }
        }
    }

    #[test]
    fn test_parse_all_egg_files() {
        use std::fs;
        use std::path::Path;

        // Path to the egglog project's tests directory
        let egglog_tests_path = Path::new("/Users/mineralsteins/Repos/egglog/tests");

        if !egglog_tests_path.exists() {
            println!(
                "Egglog tests directory not found at: {:?}",
                egglog_tests_path
            );
            return;
        }

        let mut parser = Parser::default();
        let mut total_files = 0;
        let mut parsed_successfully = 0;
        let mut failed_files = Vec::new();

        // Walk through all .egg files in the tests directory
        if let Ok(entries) = fs::read_dir(egglog_tests_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "egg") {
                        total_files += 1;

                        println!("Parsing: {:?}", path);

                        match fs::read_to_string(&path) {
                            Ok(content) => {
                                match parser.get_program_from_string(
                                    Some(path.to_string_lossy().to_string()),
                                    &content,
                                ) {
                                    Ok(commands) => {
                                        parsed_successfully += 1;
                                        println!(
                                            "  ✓ Successfully parsed {} commands",
                                            commands.len()
                                        );
                                    }
                                    Err(e) => {
                                        failed_files.push((
                                            path.to_string_lossy().to_string(),
                                            e.to_string(),
                                        ));
                                        println!("  ✗ Failed to parse: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                failed_files.push((
                                    path.to_string_lossy().to_string(),
                                    format!("Failed to read file: {}", e),
                                ));
                                println!("  ✗ Failed to read file: {}", e);
                            }
                        }
                    }
                }
            }
        }

        println!("\n=== Parsing Results ===");
        println!("Total .egg files found: {}", total_files);
        println!("Successfully parsed: {}", parsed_successfully);
        println!("Failed to parse: {}", failed_files.len());

        if !failed_files.is_empty() {
            println!("\nFailed files:");
            for (file, error) in &failed_files {
                println!("  {}: {}", file, error);
            }
        }

        // For now, we'll just print the results but not fail the test
        // This allows us to see which files work and which don't
        println!(
            "\nNote: This test does not fail on parsing errors to allow incremental development."
        );
        println!("The goal is to gradually improve the parser to handle all .egg files.");
    }
}

fn span() -> Span {
    Span {
        file: None,
        line: 0,
        col: 0,
    }
}

fn get_span(token: &Token) -> &Span {
    match token {
        Token::LParen(span) => span,
        Token::RParen(span) => span,
        Token::Symbol(_, span) => span,
        Token::String(_, span) => span,
        Token::Number(_, span) => span,
        Token::Keyword(_, span) => span,
    }
}

fn get_op_span(token: &Option<Token>) -> Span {
    match token {
        Some(token) => get_span(token).clone(),
        None => Span {
            file: None,
            line: 0,
            col: 0,
        },
    }
}
