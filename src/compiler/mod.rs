pub mod lexer;
pub mod parser;

use std::fs;
use std::path::Path;

pub struct Compiler {
    pub source: String,
}

impl Compiler {
    pub fn new(source: String) -> Self {
        Compiler { source }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let source = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read source file: {}", e))?;
        Ok(Compiler { source })
    }

    pub fn compile(&self) -> Result<parser::ASTNode, String> {
        let mut parser = parser::Parser::new(&self.source);
        parser.parse_service()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world_service() {
        let source = r#"
            service HelloService {
                @get("/hello")
                endpoint greet(name: String): String {
                    return "Hello, World!";
                }
            }
        "#;

        let compiler = Compiler::new(source.to_string());
        let result = compiler.compile();
        assert!(result.is_ok());
    }
} 