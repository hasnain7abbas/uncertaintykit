#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
    Eof,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }

            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Caret);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }

            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Handle scientific notation: 1.5e-3
                if let Some(&'e') | Some(&'E') = chars.peek() {
                    num_str.push('e');
                    chars.next();
                    if let Some(&'+') | Some(&'-') = chars.peek() {
                        num_str.push(chars.next().unwrap());
                    }
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            num_str.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                let n = num_str
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid number: {}", num_str))?;
                tokens.push(Token::Number(n));
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(ident));
            }

            // Unicode physics symbols
            '\u{03C0}' => {
                tokens.push(Token::Ident("pi".to_string()));
                chars.next();
            }
            '\u{03B8}' => {
                tokens.push(Token::Ident("theta".to_string()));
                chars.next();
            }
            '\u{03BB}' => {
                tokens.push(Token::Ident("lambda".to_string()));
                chars.next();
            }
            '\u{03B1}' => {
                tokens.push(Token::Ident("alpha".to_string()));
                chars.next();
            }
            '\u{03B2}' => {
                tokens.push(Token::Ident("beta".to_string()));
                chars.next();
            }
            '\u{03B4}' => {
                tokens.push(Token::Ident("delta".to_string()));
                chars.next();
            }
            '\u{0394}' => {
                tokens.push(Token::Ident("Delta".to_string()));
                chars.next();
            }

            _ => return Err(format!("Unexpected character: '{}'", ch)),
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}
