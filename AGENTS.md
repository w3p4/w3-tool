# AGENTS.md

This document contains essential information for agentic coding agents working in this Rust workspace repository.

## Project Structure

This is a Rust workspace with two crates:
- `token/` - Core library crate containing token management functionality
- `weld/` - CLI application crate that uses the token library

The project uses Rust 2024 edition and focuses on ERC20 token information retrieval using the Alloy blockchain library.

## Essential Commands

### Building
```bash
# Build all workspace members
cargo build

# Build with optimizations
cargo build --release

# Build specific crate
cargo build -p token
cargo build -p weld
```

### Running
```bash
# Run the CLI application
cargo run -p weld -- <args>

# Example: Get token info
cargo run -p weld -- tokens-info -a '["0x123..."]' -r https://rpc.ankr.com/eth -w
```

### Testing
```bash
# Run all tests across workspace
cargo test

# Run tests for specific crate
cargo test -p token
cargo test -p weld

# Run a single test (when tests exist)
cargo test test_name
cargo test -p token test_name
```

### Linting and Formatting
```bash
# Check code formatting
cargo fmt --check

# Format code
cargo fmt

# Run Clippy lints
cargo clippy

# Check for unused dependencies
cargo machete
```

### Development
```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

## Code Style Guidelines

### Import Organization
1. Standard library imports first (`use std::...`)
2. External crate imports second (grouped alphabetically)
3. Local module imports third
4. Use consistent import style:
   ```rust
   use alloy::primitives::Address;
   use eyre::Result;
   use serde_json::{json, to_string_pretty};
   
   use crate::client::TokenClient;
   use crate::types::TokenInfo;
   ```

### Naming Conventions
- **Structs**: PascalCase (e.g., `TokenManager`, `TokenClient`)
- **Functions**: snake_case (e.g., `get_tokens_info`, `parse_addresses`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `DEFAULT_FILE_NAME`)
- **Modules**: lowercase (e.g., `client`, `service`, `types`)
- **Fields**: snake_case (e.g., `token_infos`, `rpc_url`)

### Error Handling
- Use `eyre::Result<T>` as the standard error type
- Propagate errors using the `?` operator
- Avoid `.unwrap()` except in cases where failure is truly impossible
- For file operations that should not fail in normal operation, `.unwrap()` is acceptable

### Module Structure
- Each major component has its own module directory
- `mod.rs` files contain the main implementation
- Use `pub use` to re-export commonly used types and functions
- Keep modules focused and cohesive

### Async Code
- Use `#[tokio::main]` for async main functions
- Async functions should return `Result<T>`
- Use `.await` consistently and properly

### Struct Design
- Use `#[derive(Debug, Clone)]` for data structures
- Keep fields private when appropriate, provide getter methods
- Implement `new()` functions for construction

### Documentation
- Add doc comments to public APIs
- Use `///` for documentation comments
- Include examples in doc comments when helpful
- Use `#[allow(missing_docs)]` for generated code (e.g., sol! macros)

### CLI Structure
- Use `clap` with derive macros
- Include help text for commands and arguments
- Use `Subcommand` enum for multiple commands
- Provide clear argument descriptions

### Constants and Configuration
- Define constants for default values (e.g., `DEFAULT_FILE_NAME`)
- Use string literals for simple configuration
- Consider environment variables for deployment configuration

### Testing Guidelines
- Write unit tests in the same modules using `#[cfg(test)]`
- Write integration tests in `tests/` directory when needed
- Test both success and error paths
- Use descriptive test names

## Dependencies Management
- Use workspace dependencies in root `Cargo.toml`
- Reference workspace dependencies in individual crates
- Keep dependency versions aligned across workspace
- Prefer using features rather than version mismatches

## Blockchain Interactions
- Use Alloy library for Ethereum interactions
- Use `sol!` macro for smart contract bindings
- Prefer multicall for batch operations
- Handle RPC errors gracefully

## File Organization
- Keep `lib.rs` as the main library entry point
- Organize code into logical modules
- Use module directories for larger components
- Keep main.rs focused on CLI parsing and orchestration