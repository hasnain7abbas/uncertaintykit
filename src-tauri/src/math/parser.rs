use crate::math::ast::Expr;
use crate::math::tokenizer::Token;

const FUNC_NAMES: &[&str] = &[
    "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "exp", "ln", "log",
    "log10", "log2", "sqrt", "abs", "ceil", "floor",
];

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        let tok = self.advance();
        if &tok == expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, tok))
        }
    }

    /// Entry point: parse full expression
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_addition()
    }

    /// Addition and subtraction (lowest precedence)
    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplication()?;

        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expr::add(left, right);
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expr::sub(left, right);
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Multiplication and division
    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::mul(left, right);
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::div(left, right);
                }
                // Implicit multiplication: "2x", "3sin(x)", "(a)(b)"
                Token::Number(_) | Token::Ident(_) | Token::LParen => {
                    let right = self.parse_unary()?;
                    left = Expr::mul(left, right);
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Unary minus
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), Token::Minus) {
            self.advance();
            let expr = self.parse_power()?;
            Ok(Expr::neg(expr))
        } else {
            self.parse_power()
        }
    }

    /// Exponentiation (right-associative)
    fn parse_power(&mut self) -> Result<Expr, String> {
        let base = self.parse_atom()?;

        if matches!(self.peek(), Token::Caret) {
            self.advance();
            let exp = self.parse_unary()?;
            Ok(Expr::pow(base, exp))
        } else {
            Ok(base)
        }
    }

    /// Atoms: numbers, variables, constants, functions, parenthesized expressions
    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::num(n))
            }
            Token::Ident(name) => {
                self.advance();

                // Check if it's a function call: name(...)
                if matches!(self.peek(), Token::LParen) {
                    if FUNC_NAMES.contains(&name.to_lowercase().as_str()) {
                        self.advance(); // consume (
                        let arg = self.parse_expr()?;
                        self.expect(&Token::RParen)?;
                        return Ok(Expr::func(&name.to_lowercase(), arg));
                    }
                }

                // Check if it's a known constant
                match name.to_lowercase().as_str() {
                    "pi" => Ok(Expr::Constant(
                        "\u{03C0}".to_string(),
                        std::f64::consts::PI,
                    )),
                    "e" if !matches!(self.peek(), Token::LParen) => {
                        Ok(Expr::Constant("e".to_string(), std::f64::consts::E))
                    }
                    _ => Ok(Expr::var(&name)),
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            tok => Err(format!("Unexpected token: {:?}", tok)),
        }
    }
}

/// Convenience function
pub fn parse(input: &str) -> Result<Expr, String> {
    let tokens = crate::math::tokenizer::tokenize(input)?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expr()?;

    if !matches!(parser.peek(), Token::Eof) {
        return Err(format!(
            "Unexpected tokens after expression: {:?}",
            parser.peek()
        ));
    }

    Ok(expr)
}
