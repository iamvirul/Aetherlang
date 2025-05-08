# Aether Programming Language

Aether is a modern, cloud-native programming language designed for building scalable, distributed systems with elegant syntax and robust safety features.

## Features

- **Cloud-First Architecture**: Native support for cloud APIs, service discovery, and containerized deployments
- **Object-Oriented Core**: First-class support for classes, inheritance, interfaces, and polymorphism
- **Elegant Syntax**: Clean, minimal syntax inspired by modern languages
- **Hybrid Type System**: Static typing by default with optional dynamic typing
- **Built-in Cloud Integration**: Direct cloud provider SDK integration
- **Modern Concurrency**: Async/await, actors, and reactive streams
- **Security-Focused**: Language-level constructs for sandboxing and capability-based security

## Getting Started

### Installation

```bash
# Coming soon
curl -fsSL https://install.aetherlang.dev | sh
```

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

## Tools

- `aethc` - The Aether compiler
- `aeth` - The Aether runtime and development tool
- `aethpkg` - Package manager
- `aethctl` - Deployment and cloud control tool

## Documentation

Visit [docs.aetherlang.dev](https://docs.aetherlang.dev) for comprehensive documentation.

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

Aether is released under the MIT License. See [LICENSE](LICENSE) for details. 