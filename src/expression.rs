mod ast;
mod lexer;
use ast::{BinaryExpression, Expression, NumberExpression, UnaryExpression};
use lexer::{Token, TokenType};

static EOF: Token = Token(TokenType::Eof, String::new());

pub fn tokenize(string: &str) -> Vec<Token> {
    lexer::tokenize(string)
}

pub fn parser(tokens: &[Token]) -> Vec<Box<dyn Expression>> {
    let mut result: Vec<Box<dyn Expression>> = Vec::new();
    let mut position = 0;

    while *get(&mut position, &tokens).get_type() != TokenType::Eof {
        let expr = expression(&mut position, &tokens);
        result.push(expr);
    }

    result
}

fn expression(position: &mut usize, tokens: &[Token]) -> Box<dyn Expression> {
    addictive(position, tokens)
}

fn addictive(position: &mut usize, tokens: &[Token]) -> Box<dyn Expression> {
    let mut result: Box<dyn Expression> = multiplicative(position, tokens);

    loop {
        match get(position, tokens).get_type() {
            TokenType::Plus => {
                *position += 1;
                result = Box::new(BinaryExpression(
                    '+',
                    result,
                    multiplicative(position, tokens),
                ));
            }
            TokenType::Minus => {
                *position += 1;
                result = Box::new(BinaryExpression(
                    '-',
                    result,
                    multiplicative(position, tokens),
                ));
            }
            _ => break,
        }
    }

    result
}

fn multiplicative(position: &mut usize, tokens: &[Token]) -> Box<dyn Expression> {
    let mut result: Box<dyn Expression> = unary(position, tokens);

    loop {
        match get(position, tokens).get_type() {
            TokenType::Star => {
                *position += 1;
                result = Box::new(BinaryExpression('*', result, unary(position, tokens)));
            }
            TokenType::Slash => {
                *position += 1;
                result = Box::new(BinaryExpression('/', result, unary(position, tokens)));
            }
            _ => break,
        }
    }
    result
}

fn unary(position: &mut usize, tokens: &[Token]) -> Box<dyn Expression> {
    if *get(position, tokens).get_type() == TokenType::Minus {
        *position += 1;
        Box::new(UnaryExpression('-', primary(position, tokens)))
    } else {
        primary(position, tokens)
    }
}

fn primary(position: &mut usize, tokens: &[Token]) -> Box<dyn Expression> {
    let token = get(position, tokens);
    match token.get_type() {
        TokenType::Number => {
            let expr = Box::new(NumberExpression(token.get_value().parse().unwrap()));
            *position += 1;
            expr
        }
        TokenType::Lparen => {
            *position += 1;
            let result = expression(position, tokens);
            if *get(position, tokens).get_type() != TokenType::Rparen {
                panic!("Excpected closing parenthesis");
            }
            *position += 1;
            result
        }
        _ => panic!("Unexpecteed token: {:?}", token),
    }
}

fn get<'a>(position: &'a mut usize, tokens: &'a [Token]) -> &'a Token {
    tokens.get(*position).unwrap_or(&EOF)
}
