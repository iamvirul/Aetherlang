use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aethc")]
#[command(about = "Aether language compiler", long_about = None)]
struct Cli {
    /// Input file
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output file
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Enable optimization
    #[arg(short, long)]
    optimize: bool,

    /// Generate WebAssembly output
    #[arg(long)]
    wasm: bool,
}

fn main() {
    let cli = Cli::parse();

    // Read the input file
    let compiler = match aether::Compiler::from_file(&cli.input) {
        Ok(compiler) => compiler,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            std::process::exit(1);
        }
    };

    // Compile the input
    match compiler.compile() {
        Ok(ast) => {
            println!("Successfully compiled {:?}", cli.input);
            println!("AST: {:?}", ast);
            // TODO: Implement code generation
            if cli.wasm {
                println!("WebAssembly output not yet implemented");
            }
        }
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            std::process::exit(1);
        }
    }
} 