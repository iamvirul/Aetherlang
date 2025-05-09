# Aether Programming Language

<div align="center">
  <img src="./aether-logo.png" alt="Aetherlang Logo" width="200" style="border-radius:15px;">
</div>

Aether is a modern, cloud-native programming language designed for building scalable, distributed systems with elegant syntax and robust safety features.

## Features

- **Cloud-First Architecture**: Native support for cloud APIs, service discovery, and containerized deployments
- **Object-Oriented Core**: First-class support for classes, inheritance, interfaces, and polymorphism
- **Elegant Syntax**: Clean, minimal syntax inspired by modern languages
- **Hybrid Type System**: Static typing by default with optional dynamic typing
- **Built-in Cloud Integration**: Direct cloud provider SDK integration
- **Modern Concurrency**: Async/await, actors, and reactive streams
- **Security-Focused**: Language-level constructs for sandboxing and capability-based security

## Installation

### Pre-built Binaries

You can download pre-built binaries for your operating system from our [GitHub Releases](https://github.com/iamvirul/Aetherlang/releases) page.

#### Windows
1. Download `aether-windows-x86_64.exe` from the latest release
2. Rename it to `aeth.exe` (optional)
3. Add the binary location to your PATH environment variable
4. Open Command Prompt or PowerShell and run `aeth --version` to verify the installation

#### macOS
1. Download `aether-macos-x86_64` from the latest release
2. Make it executable:
   ```bash
   chmod +x aether-macos-x86_64
   ```
3. Move it to a location in your PATH:
   ```bash
   sudo mv aether-macos-x86_64 /usr/local/bin/aeth
   ```
4. Verify the installation:
   ```bash
   aeth --version
   ```

#### Linux
1. Download `aether-linux-x86_64` from the latest release
2. Make it executable:
   ```bash
   chmod +x aether-linux-x86_64
   ```
3. Move it to a location in your PATH:
   ```bash
   sudo mv aether-linux-x86_64 /usr/local/bin/aeth
   ```
4. Verify the installation:
   ```bash
   aeth --version
   ```

### Building from Source

If you prefer to build from source, you'll need Rust installed on your system:

1. Install Rust from [https://rustup.rs/](https://rustup.rs/)
2. Clone the repository:
   ```bash
   git clone https://github.com/iamvirul/Aetherlang.git
   cd Aetherlang
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The compiled binaries will be in `target/release/`

## Getting Started

### Hello World

```aether
service HelloService {
  @get("/hello")
  endpoint greet(name: String): String {
    return "Hello, \(name) from Aether!";
  }
}
```

### Running the Service

```bash
aeth run hello.ath
```

## VS Code Extension

Enhance your Aetherlang development experience with our dedicated VS Code extension. It provides syntax highlighting and other language features.

- **Download the latest version:** [Aetherlang VS Code Extension (aetherlang-1.0.2.vsix)](./aetherlang-vscode-extension/aetherlang-1.0.2.vsix)
- For installation instructions and more details, see the [extension's README](./aetherlang-vscode-extension/README.md).

## Tools

- `aethc` - The Aether compiler
- `aeth` - The Aether runtime and development tool
- `aethpkg` - Package manager
- `aethctl` - Deployment and cloud control tool

## Documentation

Documentation is coming soon.
We're working hard to provide you with comprehensive guides and references.

Stay tuned for updates!

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

Aether is released under the MIT License. See [LICENSE](LICENSE) for details. 