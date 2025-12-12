# Aetherlang Complete Documentation

## Table of Contents
1. [Project Overview](#project-overview)
2. [Lexer (Tokenization)](#lexer-tokenization)
3. [Parser (AST Generation)](#parser-ast-generation)
4. [Runtime & Module System](#runtime--module-system)
5. [Keywords Reference](#keywords-reference)
6. [Language Features](#language-features)
7. [Syntax Guide](#syntax-guide)
8. [Complete A-Z Reference](#complete-a-z-reference)

---

## Project Overview

**Aetherlang** (also known as **Aether**) is a modern, cloud-native programming language designed for building scalable, distributed systems. It features elegant syntax, robust safety features, and native cloud integration capabilities.

### Core Philosophy
- **Cloud-First Architecture**: Built with cloud-native principles in mind
- **Service-Oriented**: Services and endpoints are first-class citizens
- **Type-Safe**: Static typing by default with optional dynamic typing
- **Modern Concurrency**: Async/await support for concurrent operations
- **Security-Focused**: Language-level security constructs

### Project Structure
```
Aetherlang/
├── src/
│   ├── compiler/          # Compilation pipeline
│   │   ├── lexer.rs       # Tokenization
│   │   ├── parser.rs      # AST generation
│   │   └── mod.rs         # Compiler interface
│   ├── runtime/           # Runtime execution
│   │   └── mod.rs         # HTTP server & endpoint handling
│   ├── core/              # Core library modules
│   └── bin/               # CLI tools
│       ├── aeth.rs        # Runtime tool
│       └── aethc.rs       # Compiler tool
└── examples/              # Example code
```

---

## Lexer (Tokenization)

The **Lexer** is responsible for breaking down source code into tokens. It reads the input character by character and produces a stream of tokens that the parser can understand.

### Token Types

#### 1. Keywords
The lexer recognizes the following reserved keywords:

| Keyword | Token Type | Purpose |
|---------|------------|---------|
| `service` | `Service` | Declares a service definition |
| `endpoint` | `Endpoint` | Declares an HTTP endpoint |
| `class` | `Class` | Declares a class (OOP support) |
| `interface` | `Interface` | Declares an interface |
| `async` | `Async` | Marks asynchronous operations |
| `await` | `Await` | Waits for async operations |
| `cloud` | `Cloud` | Cloud-specific constructs |
| `import` | `Import` | Imports modules/packages |
| `export` | `Export` | Exports symbols |
| `try` | `Try` | Error handling block |
| `catch` | `Catch` | Catches exceptions |
| `defer` | `Defer` | Defers execution |
| `return` | `Return` | Returns a value |

#### 2. Symbols (Operators & Punctuation)

| Symbol | Token Type | Usage |
|--------|------------|-------|
| `{` | `LeftBrace` | Block/scope opening |
| `}` | `RightBrace` | Block/scope closing |
| `(` | `LeftParen` | Function/expression grouping |
| `)` | `RightParen` | Function/expression closing |
| `->` | `Arrow` | Function return type or lambda |
| `:` | `Colon` | Type annotation separator |
| `;` | `Semicolon` | Statement terminator |
| `@` | `At` | Decorator/annotation marker |

#### 3. Literals

| Literal Type | Token Type | Example |
|--------------|------------|---------|
| Identifier | `Identifier(String)` | `myVariable`, `ServiceName` |
| String | `StringLiteral(String)` | `"Hello, World!"` |
| Number | `NumberLiteral(f64)` | `42`, `3.14` |

#### 4. Special Tokens

| Token | Token Type | Purpose |
|-------|------------|---------|
| End of File | `EOF` | Marks end of input |

### Lexer Implementation Details

#### Character Processing
- **Whitespace**: Spaces and tabs are skipped
- **Newlines**: Increment line counter, reset column
- **Comments**: Block comments `{- ... -}` (planned)
- **String Escaping**: Supports `\n`, `\t`, `\r`, `\\`, `\"`

#### Token Recognition Rules

1. **Identifiers**: 
   - Start with alphabetic character or underscore
   - Can contain alphanumeric characters and underscores
   - Case-sensitive

2. **Numbers**:
   - Integer: `123`
   - Float: `123.45`
   - Supports decimal notation

3. **Strings**:
   - Double-quoted: `"text"`
   - Supports escape sequences
   - String interpolation: `\(variableName)` (in runtime)

4. **Arrow Operator**:
   - Two characters: `->`
   - Used for function types and returns

### Lexer Code Structure

```rust
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,  // Character iterator
    line: usize,                  // Current line number
    column: usize,                // Current column number
}
```

**Key Methods:**
- `new(input: &str)` - Creates a new lexer
- `next_token()` - Returns the next token
- `read_identifier()` - Reads identifier or keyword
- `read_string()` - Reads string literal with escaping
- `read_number()` - Reads numeric literal
- `skip_whitespace()` - Skips whitespace characters
- `handle_arrow()` - Processes `->` operator

---

## Parser (AST Generation)

The **Parser** takes tokens from the lexer and builds an Abstract Syntax Tree (AST) that represents the program's structure.

### AST Node Types

#### 1. Service Node
Represents a service definition containing multiple endpoints.

```rust
ASTNode::Service {
    name: String,           // Service name
    endpoints: Vec<ASTNode> // List of endpoint nodes
}
```

**Syntax:**
```aether
service ServiceName {
    // endpoints here
}
```

#### 2. Endpoint Node
Represents an HTTP endpoint with method, path, parameters, and body.

```rust
ASTNode::Endpoint {
    path: String,              // HTTP path (e.g., "/hello")
    method: String,            // HTTP method (get, post, put, delete)
    params: Vec<Parameter>,    // Function parameters
    return_type: String,       // Return type
    body: Box<ASTNode>         // Endpoint body (block)
}
```

**Syntax:**
```aether
@get("/path")
endpoint endpointName(param1: Type1, param2: Type2): ReturnType {
    // body
}
```

#### 3. Block Node
Represents a block of statements.

```rust
ASTNode::Block {
    statements: Vec<ASTNode>  // List of statements
}
```

#### 4. Return Statement Node
Represents a return statement.

```rust
ASTNode::ReturnStatement {
    expression: Box<ASTNode>  // Expression to return
}
```

**Syntax:**
```aether
return expression;
```

#### 5. String Literal Node
Represents a string literal value.

```rust
ASTNode::StringLiteral {
    value: String
}
```

#### 6. Identifier Node
Represents an identifier (variable name, etc.).

```rust
ASTNode::Identifier {
    name: String
}
```

### Parameter Structure

```rust
pub struct Parameter {
    pub name: String,      // Parameter name
    pub param_type: String // Parameter type
}
```

### Parser Implementation Details

#### Parsing Flow

1. **Service Parsing** (`parse_service`):
   - Expects `service` keyword
   - Reads service name (identifier)
   - Expects `{`
   - Parses zero or more endpoints (starting with `@`)
   - Expects `}`

2. **Endpoint Parsing** (`parse_endpoint`):
   - Expects `@` decorator
   - Reads HTTP method (identifier)
   - Expects `(` and reads path (string literal)
   - Expects `)` and `endpoint` keyword
   - Reads endpoint name (identifier)
   - Parses parameters: `(name: Type, ...)`
   - Parses return type: `: ReturnType`
   - Parses body block: `{ ... }`

3. **Block Parsing** (`parse_block`):
   - Parses statements until end of block
   - Currently supports return statements

4. **Return Statement Parsing** (`parse_return_statement`):
   - Expects `return` keyword
   - Parses expression (string literal or identifier)
   - Expects `;`

#### Parser Code Structure

```rust
pub struct Parser<'a> {
    lexer: Lexer<'a>,        // Token source
    current_token: Token,     // Current token being processed
}
```

**Key Methods:**
- `new(input: &str)` - Creates a new parser
- `advance()` - Moves to next token
- `expect(token_type)` - Validates and consumes expected token
- `parse_service()` - Parses service definition
- `parse_endpoint()` - Parses endpoint definition
- `parse_block()` - Parses code block
- `parse_return_statement()` - Parses return statement

---

## Runtime & Module System

The **Runtime** executes the compiled AST and serves HTTP endpoints using the Axum web framework.

### Runtime Architecture

#### Runtime Structure

```rust
pub struct Runtime {
    ast: ASTNode,    // Compiled AST
    port: u16,       // HTTP server port
}
```

#### Key Features

1. **HTTP Server**:
   - Built on Axum framework
   - Supports GET, POST, PUT, DELETE methods
   - Automatic route registration from AST

2. **Endpoint Execution**:
   - Maps Aetherlang endpoints to HTTP routes
   - Handles query parameters
   - Processes string interpolation: `\(paramName)`
   - Returns JSON responses

3. **System Endpoints**:
   - `/health` - Health check endpoint
   - `/system/info` - System information
   - `/system/routes` - Available routes

### Runtime Implementation Details

#### Router Building

The runtime builds an Axum router from the AST:

1. **Service Processing**:
   - Extracts all endpoints from Service node
   - Creates routes for each endpoint

2. **Endpoint Handling**:
   - GET endpoints: Extracts query parameters
   - String interpolation: Replaces `\(paramName)` with actual values
   - Returns JSON: `{ "data": value }` or `{ "error": message }`

3. **Default Endpoints**:
   - `/health`: Returns status and timestamp
   - `/system/info`: Returns version and metadata
   - `/system/routes`: Lists available routes

#### Response Format

**Success Response:**
```json
{
  "data": "response value"
}
```

**Error Response:**
```json
{
  "error": "error message"
}
```

### Module System

Currently, the module system is in development. Planned features:

- **Import/Export**: Module import/export keywords exist in lexer
- **Package Management**: `aethpkg` tool (planned)
- **Cloud Modules**: Cloud-specific module support

---

## Keywords Reference

### Complete Keyword List

| Keyword | Category | Status | Description |
|---------|----------|--------|-------------|
| `service` | Declaration | ✅ Implemented | Declares a service |
| `endpoint` | Declaration | ✅ Implemented | Declares an HTTP endpoint |
| `class` | OOP | 🔄 Planned | Declares a class |
| `interface` | OOP | 🔄 Planned | Declares an interface |
| `async` | Concurrency | 🔄 Planned | Marks async function |
| `await` | Concurrency | 🔄 Planned | Waits for async result |
| `cloud` | Cloud | 🔄 Planned | Cloud-specific construct |
| `import` | Module | 🔄 Planned | Imports module |
| `export` | Module | 🔄 Planned | Exports symbol |
| `try` | Error Handling | 🔄 Planned | Error handling block |
| `catch` | Error Handling | 🔄 Planned | Catches exceptions |
| `defer` | Control Flow | 🔄 Planned | Defers execution |
| `return` | Control Flow | ✅ Implemented | Returns value |

**Legend:**
- ✅ Implemented - Currently working
- 🔄 Planned - Keyword exists but not fully implemented

---

## Language Features

### Currently Implemented Features

#### 1. Service Definitions
```aether
service MyService {
    // endpoints
}
```

#### 2. HTTP Endpoints
```aether
@get("/path")
endpoint handler(): String {
    return "response";
}
```

#### 3. Parameters
```aether
endpoint greet(name: String): String {
    return "Hello, \(name)";
}
```

#### 4. String Interpolation
```aether
return "Hello, \(name) from Aether!";
```

#### 5. Return Statements
```aether
return expression;
```

#### 6. Type Annotations
```aether
param: Type
```

### Planned Features

#### 1. Object-Oriented Programming
- Classes and inheritance
- Interfaces and polymorphism
- Method definitions

#### 2. Asynchronous Programming
- Async/await syntax
- Promise/future handling
- Concurrent execution

#### 3. Error Handling
- Try-catch blocks
- Exception types
- Error propagation

#### 4. Module System
- Import/export statements
- Package management
- Module resolution

#### 5. Cloud Integration
- Cloud provider SDKs
- Service discovery
- Container deployment

#### 6. Advanced Types
- Generics
- Union types
- Optional types

---

## Syntax Guide

### Basic Syntax Rules

#### 1. Service Declaration
```aether
service ServiceName {
    // endpoints
}
```

#### 2. Endpoint Declaration
```aether
@method("/path")
endpoint endpointName(param1: Type1, param2: Type2): ReturnType {
    // statements
    return value;
}
```

#### 3. HTTP Method Decorators
```aether
@get("/path")      // HTTP GET
@post("/path")     // HTTP POST
@put("/path")      // HTTP PUT
@delete("/path")   // HTTP DELETE
```

#### 4. Parameters
```aether
// Single parameter
endpoint handler(name: String): String { }

// Multiple parameters
endpoint handler(name: String, age: Number): String { }
```

#### 5. Return Statement
```aether
return "Hello";
return variableName;
```

#### 6. String Literals
```aether
"Simple string"
"String with \(interpolation)"
"Escaped: \"quote\""
"Newline: \n"
```

#### 7. Identifiers
```aether
// Valid identifiers
myVariable
ServiceName
_private
var123
```

### Complete Example

```aether
service HelloService {
    @get("/hello")
    endpoint greet(name: String): String {
        return "Hello, \(name) from Aether!";
    }

    @get("/health")
    endpoint healthCheck(): String {
        return "OK";
    }

    @get("/version")
    endpoint version(): String {
        return "Aether v1.1.0";
    }
}
```

---

## Complete A-Z Reference

### A

- **AST (Abstract Syntax Tree)**: Tree representation of program structure
- **Async**: Keyword for asynchronous operations (planned)
- **Await**: Keyword for waiting on async operations (planned)
- **@ (At symbol)**: Decorator/annotation marker

### B

- **Block**: Code block enclosed in `{ }`
- **Body**: Endpoint body containing statements

### C

- **Catch**: Error handling keyword (planned)
- **Class**: Object-oriented class declaration (planned)
- **Cloud**: Cloud-specific keyword (planned)
- **Colon (`:`)**: Type annotation separator
- **Compiler**: Transforms source code to AST

### D

- **Defer**: Deferred execution keyword (planned)
- **DELETE**: HTTP DELETE method decorator

### E

- **Endpoint**: HTTP endpoint declaration
- **EOF**: End of File token
- **Export**: Module export keyword (planned)
- **Expression**: Code that evaluates to a value

### F

- **Float**: Floating-point number type (NumberLiteral)

### G

- **GET**: HTTP GET method decorator

### H

- **HTTP**: Hypertext Transfer Protocol (endpoints use HTTP)

### I

- **Identifier**: Variable, function, or type name
- **Import**: Module import keyword (planned)
- **Interface**: Interface declaration (planned)
- **Interpolation**: String variable substitution `\(name)`

### K

- **Keyword**: Reserved word in the language

### L

- **LeftBrace (`{`)**: Opening brace
- **LeftParen (`(`)**: Opening parenthesis
- **Lexer**: Tokenizer that breaks code into tokens
- **Literal**: Constant value (string, number)

### M

- **Method**: HTTP method (GET, POST, PUT, DELETE)
- **Module**: Code organization unit (planned)

### N

- **NumberLiteral**: Numeric literal (integer or float)

### O

- **Operator**: Symbol that performs operation (e.g., `->`)

### P

- **Parameter**: Function/endpoint input
- **Parser**: Builds AST from tokens
- **Path**: HTTP route path (e.g., "/hello")
- **POST**: HTTP POST method decorator

### R

- **Return**: Returns value from endpoint
- **ReturnType**: Type of return value
- **RightBrace (`}`)**: Closing brace
- **RightParen (`)`)**: Closing parenthesis
- **Runtime**: Executes compiled AST

### S

- **Semicolon (`;`)**: Statement terminator
- **Service**: Top-level service declaration
- **StringLiteral**: String value enclosed in quotes
- **Symbol**: Operator or punctuation token

### T

- **Token**: Smallest unit of code recognized by lexer
- **Try**: Error handling keyword (planned)
- **Type**: Data type annotation (e.g., String, Number)

### U

- **Underscore (`_`)**: Allowed in identifiers

### V

- **Variable**: Named storage location (identifier)

### W

- **Whitespace**: Spaces, tabs, newlines (ignored by lexer)

---

## Data Types

### Currently Supported

1. **String**: Text data
   ```aether
   name: String
   ```

2. **Number**: Numeric data (integer or float)
   ```aether
   age: Number
   price: Number
   ```

### Planned Types

- Boolean
- Array/List
- Map/Dictionary
- Custom types (classes)
- Optional types
- Union types

---

## Operators

### Currently Supported

| Operator | Description | Example |
|----------|-------------|---------|
| `->` | Arrow (function type) | `endpoint(): String` |
| `:` | Type annotation | `name: String` |
| `;` | Statement terminator | `return value;` |
| `@` | Decorator | `@get("/path")` |

### Planned Operators

- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`
- Assignment: `=`, `+=`, `-=`, etc.

---

## Error Handling

### Current Status

Error handling is planned but not yet implemented. The lexer and parser can detect syntax errors and report them with line and column information.

### Planned Features

- Try-catch blocks
- Exception types
- Error propagation
- Custom error handling

---

## Best Practices

### Service Organization

1. **Naming**: Use PascalCase for service names
   ```aether
   service UserService { }
   ```

2. **Endpoints**: Use camelCase for endpoint names
   ```aether
   endpoint getUserById() { }
   ```

3. **Paths**: Use RESTful conventions
   ```aether
   @get("/users")
   @get("/users/:id")
   ```

### Code Style

1. **Indentation**: Use consistent indentation (spaces or tabs)
2. **Spacing**: Add spaces around operators and after commas
3. **Comments**: Use block comments `{- ... -}` (when implemented)

---

## Tools & Commands

### CLI Tools

1. **aeth**: Runtime and development tool
   ```bash
   aeth run file.ath
   aeth run file.ath --port 3000
   ```

2. **aethc**: Compiler tool
   ```bash
   aethc compile file.ath
   ```

3. **aethpkg**: Package manager (planned)
4. **aethctl**: Deployment tool (planned)

---

## Examples

### Example 1: Hello World
```aether
service HelloService {
    @get("/hello")
    endpoint greet(): String {
        return "Hello, World!";
    }
}
```

### Example 2: With Parameters
```aether
service GreetingService {
    @get("/greet")
    endpoint greet(name: String): String {
        return "Hello, \(name)!";
    }
}
```

### Example 3: Multiple Endpoints
```aether
service ApiService {
    @get("/health")
    endpoint health(): String {
        return "OK";
    }

    @get("/version")
    endpoint version(): String {
        return "v1.0.0";
    }

    @get("/info")
    endpoint info(): String {
        return "Aetherlang API";
    }
}
```

---

## Version Information

- **Current Version**: 1.1.0
- **Language**: Rust
- **Runtime**: Axum (HTTP framework)
- **License**: Aetherlang License (Non-Commercial)

---

## Contributing

For contribution guidelines, see [CONTRIBUTING.md](./CONTRIBUTING.md)

---

## License

Licensed under the Aetherlang License (Non-Commercial) — see [LICENSE](./LICENSE) for details.

---

**Last Updated**: Based on codebase analysis
**Documentation Version**: 1.0
