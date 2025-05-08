use super::lexer::{Lexer, Token, TokenType};

#[derive(Debug)]
pub enum ASTNode {
    Service {
        name: String,
        endpoints: Vec<ASTNode>,
    },
    Endpoint {
        path: String,
        method: String,
        params: Vec<Parameter>,
        return_type: String,
        body: Box<ASTNode>,
    },
    Block {
        statements: Vec<ASTNode>,
    },
    ReturnStatement {
        expression: Box<ASTNode>,
    },
    StringLiteral {
        value: String,
    },
    Identifier {
        name: String,
    },
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, token_type: TokenType) -> Result<(), String> {
        if std::mem::discriminant(&self.current_token.token_type) == std::mem::discriminant(&token_type) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, got {:?} at line {} column {}",
                token_type,
                self.current_token.token_type,
                self.current_token.line,
                self.current_token.column
            ))
        }
    }

    pub fn parse_service(&mut self) -> Result<ASTNode, String> {
        self.expect(TokenType::Service)?;
        
        let name = match &self.current_token.token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected service name".to_string()),
        };
        self.advance();

        self.expect(TokenType::LeftBrace)?;
        
        let mut endpoints = Vec::new();
        while let TokenType::At = self.current_token.token_type {
            endpoints.push(self.parse_endpoint()?);
        }

        self.expect(TokenType::RightBrace)?;

        Ok(ASTNode::Service {
            name,
            endpoints,
        })
    }

    fn parse_endpoint(&mut self) -> Result<ASTNode, String> {
        // Parse @get decorator
        self.expect(TokenType::At)?;
        let method = match &self.current_token.token_type {
            TokenType::Identifier(method) => method.clone().to_lowercase(),
            _ => return Err("Expected HTTP method".to_string()),
        };
        self.advance();

        // Parse path
        self.expect(TokenType::LeftParen)?;
        let path = match &self.current_token.token_type {
            TokenType::StringLiteral(path) => path.clone(),
            _ => return Err("Expected path string".to_string()),
        };
        self.advance();
        self.expect(TokenType::RightParen)?;

        // Parse endpoint keyword and name
        self.expect(TokenType::Endpoint)?;
        let _name = match &self.current_token.token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected endpoint name".to_string()),
        };
        self.advance();

        // Parse parameters
        self.expect(TokenType::LeftParen)?;
        let mut params = Vec::new();
        while let TokenType::Identifier(_) = &self.current_token.token_type {
            let param_name = match &self.current_token.token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => break,
            };
            self.advance();

            self.expect(TokenType::Colon)?;

            let param_type = match &self.current_token.token_type {
                TokenType::Identifier(type_name) => type_name.clone(),
                _ => return Err("Expected parameter type".to_string()),
            };
            self.advance();

            params.push(Parameter {
                name: param_name,
                param_type,
            });

            if let TokenType::RightParen = self.current_token.token_type {
                break;
            }
        }
        self.expect(TokenType::RightParen)?;

        // Parse return type
        self.expect(TokenType::Colon)?;
        let return_type = match &self.current_token.token_type {
            TokenType::Identifier(type_name) => type_name.clone(),
            _ => return Err("Expected return type".to_string()),
        };
        self.advance();

        // Parse body
        self.expect(TokenType::LeftBrace)?;
        let body = self.parse_block()?;
        self.expect(TokenType::RightBrace)?;

        Ok(ASTNode::Endpoint {
            path,
            method,
            params,
            return_type,
            body: Box::new(body),
        })
    }

    fn parse_block(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();
        
        while let TokenType::Return = self.current_token.token_type {
            statements.push(self.parse_return_statement()?);
        }

        Ok(ASTNode::Block { statements })
    }

    fn parse_return_statement(&mut self) -> Result<ASTNode, String> {
        self.expect(TokenType::Return)?;
        
        let expr = match &self.current_token.token_type.clone() {
            TokenType::StringLiteral(s) => {
                let value = s.clone();
                self.advance();
                ASTNode::StringLiteral { value }
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                ASTNode::Identifier { name }
            }
            _ => return Err("Expected expression after return".to_string()),
        };

        self.expect(TokenType::Semicolon)?;

        Ok(ASTNode::ReturnStatement {
            expression: Box::new(expr),
        })
    }
} 