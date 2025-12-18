# Contributing to SCAPI

First off, thank you for considering contributing to SCAPI! 🎉

The following is a set of guidelines for contributing to SCAPI. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Guidelines](#coding-guidelines)
- [Testing](#testing)

## Code of Conduct

This project and everyone participating in it is governed by a Code of Conduct. By participating, you are expected to uphold this code.

- Be respectful and inclusive
- Be open to constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples to demonstrate the steps**
- **Describe the behavior you observed after following the steps**
- **Explain which behavior you expected to see instead and why**
- **Include code snippets and error messages**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- **Use a clear and descriptive title**
- **Provide a step-by-step description of the suggested enhancement**
- **Provide specific examples to demonstrate the steps**
- **Describe the current behavior and explain the behavior you expected to see**
- **Explain why this enhancement would be useful**

### Pull Requests

- Fill in the required template
- Follow the [Coding Guidelines](#coding-guidelines)
- Include appropriate test cases
- Update documentation as needed
- End all files with a newline

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- wasm-pack (for WASM development)
- Node.js 16+ (for testing WASM bindings)

### Setup Steps

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/scapi.git
   cd scapi
   ```

2. **Install Rust dependencies**
   ```bash
   cargo build
   ```

3. **Install wasm-pack** (if working on WASM)
   ```bash
   cargo install wasm-pack
   ```

4. **Build the project**
   ```bash
   # Regular build
   cargo build
   
   # WASM build
   wasm-pack build --target web --out-dir pkg
   ```

5. **Run tests**
   ```bash
   cargo test
   ```

### Project Structure

```
SCAPI/
├── src/
│   ├── lib.rs          # Main library entry point
│   ├── main.rs         # CLI application
│   ├── sc_api.rs       # Core API implementation
│   └── wasm.rs         # WASM bindings
├── pkg/                # WASM output (after build)
├── Cargo.toml          # Project configuration
├── README.md           # Main documentation
├── EXAMPLES.md         # Usage examples
└── CONTRIBUTING.md     # This file
```

## Pull Request Process

1. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

2. **Make your changes**
   - Write clear, self-documenting code
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**
   ```bash
   # Run Rust tests
   cargo test
   
   # Check formatting
   cargo fmt --check
   
   # Run clippy (linter)
   cargo clippy -- -D warnings
   
   # Build WASM (if applicable)
   wasm-pack build --target web --out-dir pkg
   ```

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add new feature X"
   # or
   git commit -m "fix: resolve issue with Y"
   ```

   Use conventional commit format:
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `style:` - Code style changes (formatting, etc.)
   - `refactor:` - Code refactoring
   - `test:` - Adding or updating tests
   - `chore:` - Maintenance tasks

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**
   - Go to the original repository
   - Click "New Pull Request"
   - Select your branch
   - Fill in the PR template
   - Wait for review

## Coding Guidelines

### Rust Code Style

- **Follow Rust standard formatting**
  ```bash
  cargo fmt
  ```

- **Pass clippy checks**
  ```bash
  cargo clippy -- -D warnings
  ```

- **Use meaningful variable names**
  ```rust
  // Good
  let player_count = 10;
  
  // Bad
  let pc = 10;
  ```

- **Add documentation comments**
  ```rust
  /// Builds a request to query a smart contract
  ///
  /// # Arguments
  /// * `contract_index` - The index of the contract to query
  ///
  /// # Returns
  /// A `RequestDataBuilder` instance
  pub fn new() -> Self {
      // ...
  }
  ```

- **Handle errors properly**
  ```rust
  // Use Result for fallible operations
  pub fn read_bytes(&mut self, len: usize) -> Result<&[u8]> {
      self.ensure_available(len)?;
      // ...
  }
  ```

### JavaScript/WASM Code Style

- **Use clear variable names**
- **Add JSDoc comments**
  ```javascript
  /**
   * Queries a smart contract
   * @param {number} contractIndex - Contract index
   * @param {number} inputType - Function index
   * @returns {Promise<Uint8Array>} Response bytes
   */
  async function query(contractIndex, inputType) {
      // ...
  }
  ```

- **Handle promises properly**
  ```javascript
  // Good
  try {
      const response = await query(16, 1);
  } catch (error) {
      console.error('Query failed:', error);
  }
  ```

### Documentation

- Update README.md if you change public APIs
- Add examples for new features
- Keep EXAMPLES.md up to date
- Document complex algorithms

## Testing

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let builder = RequestDataBuilder::new()
            .set_contract_index(16)
            .set_input_type(1);
        
        let bytes = builder.to_bytes();
        assert!(bytes.len() > 0);
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_request_builder

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

## Questions?

Feel free to:
- Open an issue with the `question` label
- Join our community discussions
- Reach out to maintainers

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- Git commit history

Thank you for contributing to SCAPI! 🚀

---

Made with ❤️ for Qubic ecosystem
