pub mod compiler;
pub mod runtime;
pub mod core;

pub use compiler::Compiler;
pub use runtime::Runtime;

/// The main entry point for the Aether language runtime
pub struct Aether {
    compiler: Compiler,
    port: u16,
}

impl Aether {
    /// Create a new Aether runtime instance
    pub fn new(source: String) -> Self {
        Aether {
            compiler: Compiler::new(source),
            port: 8080,
        }
    }

    /// Set the port for the service
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Compile and run an Aether program
    pub async fn run(&self) -> Result<(), String> {
        let ast = self.compiler.compile()?;
        let runtime = Runtime::new(ast, self.port);
        runtime.start().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;
    use serde_json::Value;

    #[tokio::test]
    async fn test_hello_world() {
        let source = r#"
            service HelloService {
                @get("/hello")
                endpoint greet(name: String): String {
                    return "Hello, World!";
                }
            }
        "#.to_string();

        let aether = Aether::new(source).with_port(8081);
        let server_handle = tokio::spawn(async move {
            aether.run().await.unwrap();
        });

        // Send a request to the /hello endpoint
        let response = reqwest::get("http://localhost:8081/hello").await.unwrap();
        let body: Value = response.json().await.unwrap();

        // Extract the "data" field from the response
        let message = body["data"].as_str().unwrap_or("");

        // Check if the response body is "Hello, World!"
        if message == "Hello, World!" {
            // If the response is "Hello, World!", shutdown the server
            server_handle.abort();
        }

        assert_eq!(message, "Hello, World!");
    }
}