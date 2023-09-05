use std::iter::Peekable;

enum Token {
    Number(f64),
    Operator(String),
    Function(String),
    Constant(String),
}

fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut number = String::new();
    let mut chars = expression.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_digit(10) || c == '.' {
            number.push(c);
        } else {
            if !number.is_empty() {
                tokens.push(Token::Number(number.parse().unwrap()));
                number.clear();
            }
            match c {
                '+' | '-' | '*' | '/' | '^' | '(' | ')' | '%' | '!' => {
                    tokens.push(Token::Operator(c.to_string()));
                }
                'l' => {
                    if let Some(&'o') = chars.peek() {
                        chars.next();
                        if let Some(&'g') = chars.peek() {
                            chars.next();
                            tokens.push(Token::Function("log".to_string()));
                        }
                    } else if let Some(&'n') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Function("ln".to_string()));
                    }
                }
                'e' => {
                    tokens.push(Token::Constant("e".to_string()));
                }
                'p' => {
                    if let Some(&'i') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Constant("pi".to_string()));
                    }
                }
                'P' => {
                    if let Some(&'I') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Constant("PI".to_string()));
                    }
                }
                _ => {}
            }
        }
    }
    
    if !number.is_empty() {
        tokens.push(Token::Number(number.parse().unwrap()));
    }

    tokens
}

fn factorial(n: f64) -> f64 {
    if n == 0.0 {
        return 1.0;
    }
    n * factorial(n - 1.0)
}

fn parse_base(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> f64 {
    match tokens.next() {
        Some(Token::Number(val)) => val,
        Some(Token::Operator(op)) if op == "-" => -parse_base(tokens),
        Some(Token::Operator(op)) if op == "(" => {
            let val = parse_expression(tokens);
            if let Some(Token::Operator(op)) = tokens.next() {
                if op == ")" {
                    val
                } else {
                    0.0
                }
            } else {
                0.0
            }
        },
        Some(Token::Function(func)) => {
            if func == "log" {
                10.0_f64.powf(parse_base(tokens))
            } else if func == "ln" {
                parse_base(tokens).ln()
            } else {
                0.0
            }
        },
        Some(Token::Constant(c)) if c == "e" => std::f64::consts::E,
        Some(Token::Constant(c)) if c == "pi" || c == "PI" => std::f64::consts::PI,
        _ => 0.0
    }
}


fn parse_factor(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> f64 {
    let mut base = parse_base(tokens);
    while let Some(&Token::Operator(ref op)) = tokens.peek() {
        match op.as_str() {
            "^" => {
                tokens.next();
                base = base.powf(parse_factor(tokens));
            }
            "<<" => {
                tokens.next();
                base = ((base as i64) << (parse_factor(tokens) as i64)) as f64;
            }
            ">>" => {
                tokens.next();
                base = ((base as i64) >> (parse_factor(tokens) as i64)) as f64;
            }
            "!" => {
                tokens.next();
                base = factorial(base);
            }
            _ => break,
        }
    }
    base
}



fn parse_term(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> f64 {
    let mut factor = parse_factor(tokens);
    while let Some(&Token::Operator(ref op)) = tokens.peek() {
        match op.as_str() {
            "*" => {
                tokens.next();
                factor *= parse_factor(tokens);
            },
            "/" => {
                tokens.next();
                factor /= parse_factor(tokens);
            },
            "%" => {
                tokens.next();
                factor %= parse_factor(tokens);
            },
            _ => break,
        }
    }
    factor
}

fn parse_expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> f64 {
    let mut term = parse_term(tokens);
    while let Some(&Token::Operator(ref op)) = tokens.peek() {
        match op.as_str() {
            "+" => {
                tokens.next();
                term += parse_term(tokens);
            },
            "-" => {
                tokens.next();
                term -= parse_term(tokens);
            },
            _ => break,
        }
    }
    term
}

pub fn evaluate(expression: &str) -> f64 {
    let tokens = tokenize(expression);
    let mut tokens = tokens.into_iter().peekable();
    parse_expression(&mut tokens)
}
