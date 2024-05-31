#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Lparen,
    Rparen,
    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token(pub TokenType, pub String);

impl Token {
    pub fn get_type(&self) -> &TokenType {
        &self.0
    }

    pub fn get_value(&self) -> &String {
        &self.1
    }
}

pub fn tokenize(string: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut position = 0;
    let size = string.len();
    let chars: Vec<char> = string.chars().collect();

    while size > position {
        let char = chars[position];

        if char.is_numeric() {
            let token = tokenize_number(char, &mut position, size, &chars);
            tokens.push(token);
        } else if let Some(token) = tokenize_operator(char) {
            tokens.push(token);
            position += 1;
        } else {
            position += 1;
        }
    }
    tokens
}

fn tokenize_number(char: char, position: &mut usize, size: usize, chars: &[char]) -> Token {
    let mut buffer = String::new();
    println!("char: {}", chars[*position]);
    buffer.push(char);
    *position += 1;

    while size > *position {
        if chars[*position].is_numeric() || chars[*position] == '.' {
            if chars[*position] == '.' && buffer.contains('.') {
                panic!("Incorrect float number");
            }
            println!("char: {}", chars[*position]);
            buffer.push(chars[*position]);
            *position += 1;
        } else {
            break;
        }
    }
    Token(TokenType::Number, buffer)
}

fn tokenize_operator(char: char) -> Option<Token> {
    let token_type = match char {
        '+' => TokenType::Plus,
        '*' => TokenType::Star,
        '-' => TokenType::Minus,
        '/' => TokenType::Slash,
        '(' => TokenType::Lparen,
        ')' => TokenType::Rparen,
        _ => return None,
    };
    Some(Token(token_type, String::new()))
}
