use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Service,
    Endpoint,
    Class,
    Interface,
    Async,
    Await,
    Cloud,
    Import,
    Export,
    Try,
    Catch,
    Defer,
    
    // Symbols
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Arrow,
    Colon,
    Semicolon,
    At,
    
    // Literals
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    
    // Special
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(&c) = self.input.peek() {
            match c {
                '{' => self.single_char_token(TokenType::LeftBrace),
                '}' => self.single_char_token(TokenType::RightBrace),
                '(' => self.single_char_token(TokenType::LeftParen),
                ')' => self.single_char_token(TokenType::RightParen),
                ':' => self.single_char_token(TokenType::Colon),
                ';' => self.single_char_token(TokenType::Semicolon),
                '@' => self.single_char_token(TokenType::At),
                '-' => self.handle_arrow(),
                '"' => self.read_string(),
                c if c.is_alphabetic() => self.read_identifier(),
                c if c.is_digit(10) => self.read_number(),
                _ => {
                    self.input.next();
                    Token {
                        token_type: TokenType::EOF,
                        line: self.line,
                        column: self.column,
                    }
                }
            }
        } else {
            Token {
                token_type: TokenType::EOF,
                line: self.line,
                column: self.column,
            }
        }
    }

    fn single_char_token(&mut self, token_type: TokenType) -> Token {
        self.input.next();
        self.column += 1;
        Token {
            token_type,
            line: self.line,
            column: self.column - 1,
        }
    }

    fn handle_arrow(&mut self) -> Token {
        self.input.next();
        self.column += 1;
        
        if let Some(&'>') = self.input.peek() {
            self.input.next();
            self.column += 1;
            Token {
                token_type: TokenType::Arrow,
                line: self.line,
                column: self.column - 2,
            }
        } else {
            // Handle single minus if needed
            Token {
                token_type: TokenType::EOF, // Placeholder
                line: self.line,
                column: self.column - 1,
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let start_column = self.column;

        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.input.next();
                self.column += 1;
            } else {
                break;
            }
        }

        let token_type = match identifier.as_str() {
            "service" => TokenType::Service,
            "endpoint" => TokenType::Endpoint,
            "class" => TokenType::Class,
            "interface" => TokenType::Interface,
            "async" => TokenType::Async,
            "await" => TokenType::Await,
            "cloud" => TokenType::Cloud,
            "import" => TokenType::Import,
            "export" => TokenType::Export,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "defer" => TokenType::Defer,
            _ => TokenType::Identifier(identifier),
        };

        Token {
            token_type,
            line: self.line,
            column: start_column,
        }
    }

    fn read_string(&mut self) -> Token {
        self.input.next(); // Skip opening quote
        self.column += 1;
        let start_column = self.column;
        let mut string = String::new();

        while let Some(&c) = self.input.peek() {
            match c {
                '"' => {
                    self.input.next();
                    self.column += 1;
                    break;
                }
                '\\' => {
                    self.input.next();
                    self.column += 1;
                    if let Some(next) = self.input.next() {
                        string.push(match next {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '\\' => '\\',
                            '"' => '"',
                            _ => next,
                        });
                        self.column += 1;
                    }
                }
                _ => {
                    string.push(c);
                    self.input.next();
                    self.column += 1;
                }
            }
        }

        Token {
            token_type: TokenType::StringLiteral(string),
            line: self.line,
            column: start_column,
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        let start_column = self.column;

        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) || c == '.' {
                number.push(c);
                self.input.next();
                self.column += 1;
            } else {
                break;
            }
        }

        Token {
            token_type: TokenType::NumberLiteral(number.parse().unwrap_or(0.0)),
            line: self.line,
            column: start_column,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            match c {
                ' ' | '\t' => {
                    self.input.next();
                    self.column += 1;
                }
                '\n' => {
                    self.input.next();
                    self.line += 1;
                    self.column = 0;
                }
                _ => break,
            }
        }
    }
} 