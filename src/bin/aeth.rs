use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aeth")]
#[command(about = "Aether language runtime and development tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run an Aether program
    Run {
        /// Input file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Port to run the service on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Initialize a new Aether project
    Init {
        /// Project name
        #[arg(value_name = "NAME")]
        name: String,
    },
    /// Build the project
    Build {
        /// Enable optimization
        #[arg(short, long)]
        optimize: bool,
    },
    /// Deploy the service
    Deploy {
        /// Target environment (dev/staging/prod)
        #[arg(short, long, default_value = "dev")]
        env: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { input, port } => {
            println!("Running {} on port {}", input.display(), port);
            match aether::Compiler::from_file(&input) {
                Ok(compiler) => {
                    let aether = aether::Aether::new(compiler.source).with_port(port);
                    if let Err(e) = aether.run().await {
                        eprintln!("Runtime error: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Init { name } => {
            println!("Initializing new project: {}", name);
            // Create project structure
            let project_dir = PathBuf::from(&name);
            if let Err(e) = std::fs::create_dir_all(&project_dir) {
                eprintln!("Failed to create project directory: {}", e);
                std::process::exit(1);
            }

            // Create example service
            let example_service = format!(
                "service {}Service {{\n    @get(\"/hello\")\n    endpoint greet(name: String): String {{\n        return \"Hello, \\(name) from {}!\";\n    }}\n}}\n",
                name, name
            );
            if let Err(e) = std::fs::write(project_dir.join("main.ath"), example_service) {
                eprintln!("Failed to create example service: {}", e);
                std::process::exit(1);
            }

            println!("✨ Created new Aether project: {}", name);
            println!("To get started:");
            println!("  cd {}", name);
            println!("  aeth run main.ath");
        }
        Commands::Build { optimize } => {
            println!("Building project{}", if optimize { " with optimization" } else { "" });
            // TODO: Implement build process
            println!("Build process not yet implemented");
        }
        Commands::Deploy { env } => {
            println!("Deploying to {} environment", env);
            // TODO: Implement deployment
            println!("Deployment not yet implemented");
        }
    }
} 