# Contributing to freqtrade-rs

> Thank you for considering contributing to freqtrade-rs! This document outlines the process for contributing to this project.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Coding Standards](#coding-standards)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Documentation](#documentation)
- [Questions](#questions)

---

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: 1.70 or later
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Node.js**: 18 or later
  ```bash
  # Using nvm (recommended)
  nvm install 18
  nvm use 18
  ```

- **pnpm**: 8 or later
  ```bash
  npm install -g pnpm
  ```

- **Tauri CLI**
  ```bash
  cargo install tauri-cli
  ```

### Setting Up Your Development Environment

1. **Fork the repository**
   
   Click the "Fork" button on GitHub, then clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/freqtrade-rs.git
   cd freqtrade-rs
   ```

2. **Set up upstream remote**
   ```bash
   git remote add upstream https://github.com/code-yeongyu/freqtrade-rs.git
   ```

3. **Install dependencies**
   ```bash
   # Install Rust dependencies
   cd src-tauri
   cargo fetch
   cargo build

   # Install frontend dependencies
   cd ../src
   pnpm install
   ```

4. **Verify your setup**
   ```bash
   # Should run without errors
   cd src-tauri
   cargo check

   cd ../src
   pnpm run build
   ```

## Development Process

### 1. Create a Feature Branch

```bash
# Ensure you're on the latest main branch
git checkout main
git pull upstream main

# Create a new feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

### 2. Make Your Changes

Follow the [coding standards](#coding-standards) and make your changes.

### 3. Test Your Changes

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run frontend tests
cd ../src
pnpm run test
```

### 4. Commit Your Changes

Follow the [commit message guidelines](#commit-messages):
```bash
git add .
git commit -m "feat: add new risk protection mechanism"
```

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then open a Pull Request on GitHub.

## Coding Standards

### Rust (Backend)

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
// ✅ Good: Clear naming, documentation, error handling
/// Creates a new risk protection mechanism.
///
/// # Arguments
///
/// * `config` - Configuration for the protection mechanism
///
/// # Returns
///
/// A new `CooldownPeriod` instance or an error if config is invalid.
pub fn new(config: CooldownPeriodConfig) -> Result<Self, ConfigError> {
    if config.stop_duration <= 0 {
        return Err(ConfigError::InvalidDuration(config.stop_duration));
    }
    Ok(Self { config })
}

// ❌ Bad: No documentation, unclear naming
pub fn create_protection(cfg: &Config) -> CooldownPeriod {
    // ...
}
```

**Key Rules**:
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Write doc comments for all public items
- Use `Result<T, AppError>` for error handling
- Never suppress type errors with `as any`, `@ts-ignore`, etc.

### TypeScript/React (Frontend)

```typescript
// ✅ Good: TypeScript interfaces, clear naming
interface Trade {
  id: string;
  pair: string;
  openRate: number;
  closeRate?: number;
  profit: number;
}

// ❌ Bad: No type safety, unclear naming
interface TradeData {
  id: string;
  p: string;
  o: number;
  c?: number;
}
```

**Key Rules**:
- Use TypeScript for all new code
- Use functional components with hooks
- Follow the existing component structure
- Use TailwindCSS for styling
- Run `pnpm run lint` before committing

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types**:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools

**Examples**:
```
feat(risk): add cooldown protection mechanism

Implement cooldown protection that prevents trading after a specified
number of losing trades.

Closes #123
```

```
fix(exchange): handle API rate limiting gracefully

Add exponential backoff for Binance API calls to prevent rate limit
errors.

Closes #456
```

```
docs: update API documentation

Add examples for all Tauri commands.
```

### Pull Request Process

1. **Fill in the PR template** - Provide a clear description of your changes
2. **Link related issues** - Use `Closes #123` or `Fixes #456`
3. **Ensure tests pass** - All tests must pass before merging
4. **Update documentation** - Update relevant docs if needed
5. **Get review** - At least one approval required

#### PR Title Convention

Use the same conventional commit format:
```
feat(risk): add new protection mechanism
fix(bot): resolve trade execution deadlock
docs(api): update command documentation
```

#### Review Checklist

- [ ] Code follows project conventions
- [ ] Tests added/updated for new functionality
- [ ] Documentation updated
- [ ] No linting errors
- [ ] TypeScript types correct
- [ ] No commented-out code

## Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run tests for specific module
cargo test --package freqtrade-rs --lib risk

# Run tests with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

### Frontend Tests

```bash
# Run all tests
pnpm run test

# Run in watch mode
pnpm run test:watch

# Run with coverage
pnpm run test:coverage
```

### Writing Tests

**Rust**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cooldown_protection() {
        let protection = CooldownPeriod::new(CooldownPeriodConfig {
            stop_duration: 60,
            lookback_period: 1440,
            stop_after_losses: 2,
        });

        // Test logic
        assert!(protection.is_locked());
    }
}
```

**TypeScript**:
```typescript
describe('Trade', () => {
  it('should calculate profit correctly', () => {
    const trade = new Trade({
      openRate: 50000,
      closeRate: 55000,
      amount: 0.1,
    });
    expect(trade.profit).toBe(500);
  });
});
```

## Documentation

### Updating Docs

- Update relevant documentation for user-facing changes
- Add code comments for complex logic
- Update API documentation in `docs/api/`

### Writing Documentation

Follow the existing documentation style:
```markdown
# Document Title

## Overview
Brief description of what this document covers.

## Usage
Code examples and instructions.

## Configuration
Configuration options and examples.

## Related
Links to related documentation.
```

## Questions

### I want to contribute but don't know where to start

Check out the [Good First Issues](https://github.com/code-yeongyu/freqtrade-rs/issues?q=label:good+first+issue) label.

### I have a question about the project

Open a [GitHub Discussion](https://github.com/code-yeongyu/freqtrade-rs/discussions) or ask in the Discord server.

### I found a security vulnerability

**DO NOT** open a public issue. Email security concerns to the maintainers directly.

---

## 🙏 Thank You!

Your contributions make this project better for everyone. We appreciate your time and effort!
