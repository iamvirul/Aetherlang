use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "aeth")]
#[command(about = "Aether language runtime and development tool", long_about = None)]
#[command(version)]
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

        /// Enable hot reload
        #[arg(short, long)]
        watch: bool,
    },
    /// Initialize a new Aether project
    New {
        /// Project name
        #[arg(value_name = "NAME")]
        name: String,

        /// Project template (default: "service", options: "service", "cli", "library")
        #[arg(short, long, default_value = "service")]
        template: String,
    },
    /// Build the project
    Build {
        /// Enable optimization
        #[arg(short, long)]
        optimize: bool,

        /// Build target (wasm, native)
        #[arg(short, long, default_value = "native")]
        target: String,
    },
    /// Deploy the service
    Deploy {
        /// Target environment (dev/staging/prod)
        #[arg(short, long, default_value = "dev")]
        env: String,

        /// Cloud provider (aws, gcp, azure)
        #[arg(short, long)]
        provider: Option<String>,
    },
    /// Manage project dependencies
    Dep {
        #[command(subcommand)]
        action: DepCommands,
    },
    /// Development tools
    Dev {
        #[command(subcommand)]
        action: DevCommands,
    },
    /// Package management
    Pkg {
        #[command(subcommand)]
        action: PkgCommands,
    },
}

#[derive(Subcommand)]
enum DepCommands {
    /// Add a dependency
    Add {
        /// Package name
        name: String,
        /// Package version (optional)
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Remove a dependency
    Remove {
        /// Package name
        name: String,
    },
    /// Update dependencies
    Update {
        /// Specific package (optional)
        #[arg(value_name = "PACKAGE")]
        package: Option<String>,
    },
    /// List dependencies
    List,
}

#[derive(Subcommand)]
enum DevCommands {
    /// Start development server with hot reload
    Serve {
        /// Port number
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Run tests
    Test {
        /// Test pattern
        #[arg(value_name = "PATTERN")]
        pattern: Option<String>,
        /// Enable watch mode
        #[arg(short, long)]
        watch: bool,
    },
    /// Format code
    Fmt {
        /// Check only (don't modify files)
        #[arg(short, long)]
        check: bool,
    },
    /// Run linter
    Lint {
        /// Fix automatically
        #[arg(short, long)]
        fix: bool,
    },
}

#[derive(Subcommand)]
enum PkgCommands {
    /// Initialize package configuration
    Init,
    /// Publish package to registry
    Publish {
        /// Skip version check
        #[arg(long)]
        no_verify: bool,
    },
    /// Login to package registry
    Login,
    /// Search packages
    Search {
        /// Search query
        query: String,
    },
}

#[derive(Serialize, Deserialize)]
struct AetherConfig {
    name: String,
    version: String,
    description: Option<String>,
    authors: Vec<String>,
    dependencies: std::collections::HashMap<String, String>,
}

impl AetherConfig {
    fn new(name: &str) -> Self {
        AetherConfig {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: None,
            authors: vec![],
            dependencies: std::collections::HashMap::new(),
        }
    }

    fn save(&self, path: &PathBuf) -> std::io::Result<()> {
        let content = toml::to_string_pretty(self).unwrap();
        fs::write(path.join("aether.toml"), content)
    }

    fn load(path: &PathBuf) -> std::io::Result<Self> {
        let content = fs::read_to_string(path.join("aether.toml"))?;
        Ok(toml::from_str(&content).unwrap())
    }
}

fn create_project_structure(name: &str, template: &str) -> std::io::Result<()> {
    let project_dir = PathBuf::from(name);
    fs::create_dir_all(&project_dir)?;
    fs::create_dir_all(project_dir.join("src"))?;
    fs::create_dir_all(project_dir.join("tests"))?;

    // Create config file
    let config = AetherConfig::new(name);
    config.save(&project_dir)?;

    // Create gitignore
    fs::write(
        project_dir.join(".gitignore"),
        "/target\n/dist\n.aether\n*.log\n",
    )?;

    // Create README
    fs::write(
        project_dir.join("README.md"),
        format!("# {}\n\nAether project created with `aeth new`\n", name),
    )?;

    // Create main file based on template
    let main_content = match template {
        "service" => format!(
            "service {}Service {{\n    @get(\"/hello\")\n    endpoint greet(name: String): String {{\n        return \"Hello, \\(name) from {}!\";\n    }}\n}}\n",
            name, name
        ),
        "cli" => format!(
            "cli {name} {{\n    command main() {{\n        println(\"Hello from {name}!\");\n    }}\n}}\n"
        ),
        "library" => format!(
            "module {name} {{\n    pub fn greet(name: String): String {{\n        return \"Hello, \\(name)!\";\n    }}\n}}\n"
        ),
        _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid template")),
    };

    fs::write(project_dir.join("src/main.ath"), main_content)?;

    // Create test file
    fs::write(
        project_dir.join("tests/main_test.ath"),
        "test MainTest {\n    fn test_greet() {\n        assert_eq(greet(\"World\"), \"Hello, World!\");\n    }\n}\n",
    )?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { input, port, watch } => {
            println!("Running {} on port {}", input.display(), port);
            if watch {
                println!("Hot reload enabled");
                // TODO: Implement hot reload
            }
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
        Commands::New { name, template } => {
            println!("Creating new {} project: {}", template, name);
            match create_project_structure(&name, &template) {
                Ok(()) => {
                    println!("\n✨ Successfully created project: {}", name);
                    println!("\nNext steps:");
                    println!("  cd {}", name);
                    println!("  aeth dep add aether-std  # Add standard library");
                    println!("  aeth run src/main.ath    # Run the project");
                }
                Err(e) => {
                    eprintln!("Failed to create project: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Build { optimize, target } => {
            println!(
                "Building for {} target{}",
                target,
                if optimize { " with optimization" } else { "" }
            );
            // TODO: Implement build process
            println!("Build process not yet implemented");
        }
        Commands::Deploy { env, provider } => {
            if let Some(prov) = provider {
                println!("Deploying to {} environment on {}", env, prov);
            } else {
                println!("Deploying to {} environment", env);
            }
            // TODO: Implement deployment
            println!("Deployment not yet implemented");
        }
        Commands::Dep { action } => match action {
            DepCommands::Add { name, version } => {
                println!(
                    "Adding dependency: {}{}",
                    name,
                    version.clone().map_or("".to_string(), |v| format!("@{}", v))
                );
                
                let current_dir = std::env::current_dir().expect("Failed to get current directory");
                match AetherConfig::load(&current_dir) {
                    Ok(mut config) => {
                        config.dependencies.insert(
                            name.clone(),
                            version.unwrap_or_else(|| "latest".to_string()),
                        );
                        if let Err(e) = config.save(&current_dir) {
                            eprintln!("Failed to save configuration: {}", e);
                            std::process::exit(1);
                        }
                        println!("✨ Successfully added dependency");
                    }
                    Err(e) => {
                        eprintln!("Failed to load configuration: {}", e);
                        eprintln!("Make sure you're in an Aether project directory");
                        std::process::exit(1);
                    }
                }
            }
            DepCommands::Remove { name } => {
                println!("Removing dependency: {}", name);
                
                let current_dir = std::env::current_dir().expect("Failed to get current directory");
                match AetherConfig::load(&current_dir) {
                    Ok(mut config) => {
                        if config.dependencies.remove(&name).is_some() {
                            if let Err(e) = config.save(&current_dir) {
                                eprintln!("Failed to save configuration: {}", e);
                                std::process::exit(1);
                            }
                            println!("✨ Successfully removed dependency");
                        } else {
                            eprintln!("Dependency '{}' not found in project", name);
                            std::process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to load configuration: {}", e);
                        eprintln!("Make sure you're in an Aether project directory");
                        std::process::exit(1);
                    }
                }
            }
            DepCommands::Update { package } => {
                let current_dir = std::env::current_dir().expect("Failed to get current directory");
                match AetherConfig::load(&current_dir) {
                    Ok(mut config) => {
                        if let Some(pkg) = package {
                            if let Some(version) = config.dependencies.get_mut(&pkg) {
                                *version = "latest".to_string();
                                println!("Updating dependency: {}", pkg);
                            } else {
                                eprintln!("Dependency '{}' not found in project", pkg);
                                std::process::exit(1);
                            }
                        } else {
                            // Update all dependencies to latest
                            for (pkg, version) in config.dependencies.iter_mut() {
                                *version = "latest".to_string();
                                println!("Updating dependency: {}", pkg);
                            }
                        }
                        
                        if let Err(e) = config.save(&current_dir) {
                            eprintln!("Failed to save configuration: {}", e);
                            std::process::exit(1);
                        }
                        println!("✨ Successfully updated dependencies");
                    }
                    Err(e) => {
                        eprintln!("Failed to load configuration: {}", e);
                        eprintln!("Make sure you're in an Aether project directory");
                        std::process::exit(1);
                    }
                }
            }
            DepCommands::List => {
                let current_dir = std::env::current_dir().expect("Failed to get current directory");
                match AetherConfig::load(&current_dir) {
                    Ok(config) => {
                        println!("\nProject dependencies:");
                        println!("-------------------");
                        if config.dependencies.is_empty() {
                            println!("No dependencies installed");
                        } else {
                            for (name, version) in config.dependencies {
                                println!("{}: {}", name, version);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to load configuration: {}", e);
                        eprintln!("Make sure you're in an Aether project directory");
                        std::process::exit(1);
                    }
                }
            }
        },
        Commands::Dev { action } => match action {
            DevCommands::Serve { port } => {
                println!("Starting development server on port {}", port);
                // TODO: Implement dev server
            }
            DevCommands::Test { pattern, watch } => {
                println!(
                    "Running tests{}{}",
                    pattern.map_or("".to_string(), |p| format!(" matching '{}'", p)),
                    if watch { " in watch mode" } else { "" }
                );
                // TODO: Implement test runner
            }
            DevCommands::Fmt { check } => {
                if check {
                    println!("Checking code formatting");
                } else {
                    println!("Formatting code");
                }
                // TODO: Implement formatter
            }
            DevCommands::Lint { fix } => {
                println!(
                    "Running linter{}",
                    if fix { " with auto-fix" } else { "" }
                );
                // TODO: Implement linter
            }
        },
        Commands::Pkg { action } => match action {
            PkgCommands::Init => {
                println!("Initializing package configuration");
                // TODO: Implement package initialization
            }
            PkgCommands::Publish { no_verify } => {
                println!(
                    "Publishing package{}",
                    if no_verify { " (skipping verification)" } else { "" }
                );
                // TODO: Implement package publishing
            }
            PkgCommands::Login => {
                println!("Logging in to package registry");
                // TODO: Implement registry login
            }
            PkgCommands::Search { query } => {
                println!("Searching for packages matching: {}", query);
                // TODO: Implement package search
            }
        },
    }
} 