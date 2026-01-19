# AGENTS.md

This file provides guidance for AI agents working on the freqtrade-rs codebase.

## Build, Lint, and Test Commands

### Backend (Rust)

```bash
cd src-tauri

# Check compilation
cargo check

# Build release
cargo build

# Run development server
cargo run

# Run all tests
cargo test

# Run a single test
cargo test test_name                      # By test name
cargo test module_name::test_function     # By full path

# Linting
cargo clippy

# Formatting
cargo fmt

# Verify formatting (CI check)
cargo fmt -- --check
```

### Frontend (React + TypeScript)

```bash
cd src

# Install dependencies
pnpm install

# Development server
pnpm run dev

# Production build
pnpm run build

# Linting
pnpm run lint

# Format code
pnpm run format

# Check formatting
pnpm run format:check

# Type checking
pnpm run type-check
```

### Full Stack

```bash
# Run both frontend + backend in dev mode
pnpm run tauri:dev

# Build production bundle
pnpm run tauri:build
```

## Code Style Guidelines

### Rust Backend

**Formatting:**
- Uses `rustfmt` for consistent code formatting
- Configuration: `src-tauri/rustfmt.toml`
- Toolchain: `src-tauri/rust-toolchain.toml` (Rust 1.70+)
- Run `cargo fmt` before committing

**Naming Conventions:**
- `PascalCase` for types, enums, traits
- `snake_case` for variables, functions, modules
- `SCREAMING_SNAKE_CASE` for constants
- Prefixes like `Arc<Mutex<T>>` for shared state

**Error Handling:**
- Use `crate::error::AppError` enum for all errors
- Use `crate::Result<T>` alias for return types
- Use `?` operator for error propagation
- Add `From<T>` implementations for error conversions
- Never suppress errors with `unwrap()` in production code

```rust
use crate::error::{AppError, Result};

// Return Result<T, AppError>
async fn my_function() -> Result<SomeType> {
    let data = repository.get_data().await?;
    Ok(data)
}
```

**Financial Calculations:**
- ALWAYS use `rust_decimal::Decimal` for money/financial values
- NEVER use `f64` for monetary values
- Import: `use rust_decimal::Decimal`

**DateTime:**
- ALL timestamps use `chrono::DateTime<Utc>`
- Import: `use chrono::{DateTime, Utc};`

**Async/Await:**
- Use Tokio runtime throughout (already in dependencies)
- Never block on async operations
- Use `tokio::sync::Mutex` for async locks

**Shared State:**
- Wrap pools and managers in `Arc` for thread-safe access
- Use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for shared mutable state

**Imports:**
- Use absolute imports with `crate::` prefix for internal modules
- Group imports by crate (std, external, internal)
- Example:
```rust
use std::sync::Arc;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::error::Result;
use crate::types::*;
use crate::{exchange, persistence, risk};
```

**Tauri Commands:**
- Mark with `#[tauri::command]` attribute
- Take `State<'_, AppState>` as first parameter after `self`
- Return `Result<T>` for error handling
- Must be `async` for I/O operations

```rust
#[tauri::command]
pub async fn get_data(state: State<'_, AppState>) -> Result<Data> {
    let data = state.repository.get_data().await?;
    Ok(data)
}
```

**Module Pattern:**
```
module/
├── mod.rs          # Module entry, exports
├── module.rs       # Main implementation
├── submodule1.rs   # Sub-components
└── submodule2.rs
```

**Testing:**
- Unit tests alongside modules: `module_name.rs` → `module_name_test.rs`
- Integration tests in `src-tauri/tests/` directory
- Use `#[cfg(test)]` module for unit tests

### Frontend (React + TypeScript)

**TypeScript Configuration:**
- `strict: true` enabled in `tsc`
- Path alias: `@/` maps to `src/`
- Module system: ESNext

**Naming Conventions:**
- `PascalCase` for components and types
- `camelCase` for variables, functions, hooks
- `UPPER_SNAKE_CASE` for constants
- Hooks MUST start with `use` prefix

**Imports:**
- Use `@/` alias for internal imports
- Group: React imports → external libraries → internal components/utils
- Use `import { ... } from 'library'` for named imports

```typescript
import { useState, useEffect } from 'react';
import { LucideIcon } from 'lucide-react';
import StatCard from '@/components/StatCard';
import { useTradeStore } from '@/stores/tradeStore';
```

**Components:**
- Use functional components with hooks
- Props interface with `Props` suffix
- Use TypeScript interfaces for all props
- Export default for page components, named export for shared components

```typescript
interface StatCardProps {
  title: string;
  value: string;
  icon: LucideIcon;
  trend: 'up' | 'down' | 'neutral';
}

const StatCard = ({ title, value, icon: Icon, trend }: StatCardProps) => {
  // component logic
};

export default StatCard;
```

**State Management:**
- Use Zustand for global state
- Create stores in `src/stores/`
- Pattern: `useStoreName` hook

**Styling:**
- TailwindCSS for all styling
- Use semantic color classes (e.g., `text-emerald-400`, `bg-slate-800`)
- Avoid custom CSS when Tailwind suffices

**Linting Rules:**
- `no-unused-vars`: Warn for unused variables (except `_` prefix)
- `unused-imports/no-unused-imports`: Error
- `react-hooks/exhaustive-deps`: Warn
- `react/react-in-jsx-scope`: Off (JSX runtime)

### Type Safety Non-Negotiable Rules

**NEVER use:**
- `as any`, `@ts-ignore`, `@ts-expect-error`
- Empty catch blocks: `catch(e) {}`
- `unwrap()` or `expect()` in production code
- `asyn` instead of `async` typos

### Git Workflow

```bash
# Before committing
cd src-tauri && cargo fmt && cargo clippy
cd src && pnpm run lint && pnpm run format

# Verify tests pass
cd src-tauri && cargo test
```

## Project Structure Reference

```
freqtrade-rs/
├── src-tauri/src/        # Rust backend
│   ├── bot/              # Trading bot core
│   ├── exchange/         # Exchange integrations
│   ├── strategy/         # Strategy system
│   ├── backtest/         # Backtesting engine
│   ├── risk/             # Risk management
│   ├── persistence/      # Data layer (Repository pattern)
│   ├── config/           # Configuration
│   ├── commands.rs       # Tauri commands
│   ├── error.rs          # AppError enum
│   ├── lib.rs            # Library root
│   ├── main.rs           # Entry point
│   └── types.rs          # Core types
│
├── src/                  # React frontend
│   ├── components/       # Reusable UI components
│   ├── pages/            # Route pages
│   ├── stores/           # Zustand state
│   ├── services/         # API calls
│   ├── types/            # TypeScript types
│   ├── ui/               # Base UI components
│   └── contexts/         # React contexts
│
├── config/               # Bot configuration
├── migrations/           # Database migrations
└── user_data/            # User data directory
```

## Key Dependencies

### Backend
- `tauri = "2"` - Desktop framework
- `tokio = "1.42"` - Async runtime
- `sqlx = "0.8"` - Database ORM
- `thiserror` - Error definitions
- `rust_decimal` - Financial precision
- `chrono` - DateTime handling

### Frontend
- `react = "^18.3.1"` - UI framework
- `typescript = "^5.6.0"` - Type safety
- `react-router-dom = "^7.0.0"` - Routing
- `zustand = "^5.0.0"` - State management
- `tailwindcss = "^3.4.15"` - Styling
- `recharts` - Charts
- `lucide-react` - Icons

## Common Patterns

### Backend
1. Async/await for all I/O operations
2. Repository pattern for database access
3. `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for shared state
4. `Result<T, AppError>` for error propagation
5. Tauri commands as API layer

### Frontend
1. Functional components with hooks
2. Zustand for state management
3. TailwindCSS for styling
4. TypeScript strict mode
5. Named exports for utilities, default for pages
