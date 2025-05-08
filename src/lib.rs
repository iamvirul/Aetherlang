pub mod compiler;

pub use compiler::Compiler;

/// The main entry point for the Aether language runtime
pub struct Aether {
    compiler: Compiler,
}

impl Aether {
    /// Create a new Aether runtime instance
    pub fn new(source: String) -> Self {
        Aether {
            compiler: Compiler::new(source),
        }
    }

    /// Compile and run an Aether program
    pub fn run(&self) -> Result<(), String> {
        let ast = self.compiler.compile()?;
        // TODO: Implement code generation and runtime execution
        println!("AST: {:?}", ast);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let source = r#"
            service HelloService {
                @get("/hello")
                endpoint greet(name: String): String {
                    return "Hello, World!";
                }
            }
        "#.to_string();

        let aether = Aether::new(source);
        assert!(aether.run().is_ok());
    }
} 