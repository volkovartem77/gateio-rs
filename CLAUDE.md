# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust SDK for the Gate.io cryptocurrency exchange API. The codebase implements a dual-client architecture supporting both synchronous (ureq) and asynchronous (hyper) HTTP operations.

## Common Development Commands

```bash
# Check compilation
cargo check

# Build with default features (sync client with ureq)
cargo build

# Build with async client (hyper)
cargo build --features enable-hyper --no-default-features

# Build with both clients
cargo build --all-features

# Run examples
cargo run --example sync_example
cargo run --example async_example --features enable-hyper --no-default-features

# Format code
cargo fmt

# Run clippy linter
cargo clippy
```

## Architecture & Key Concepts

### 1. Dual Client Implementation
The SDK provides two HTTP client implementations controlled by feature flags:
- **Sync Client (ureq)**: Default feature, located in `src/ureq/`
- **Async Client (hyper)**: Optional feature `enable-hyper`, located in `src/hyper/`

Both clients share a common HTTP abstraction layer in `src/http/` that handles request signing and method definitions.

### 2. API Request Pattern
Each API endpoint follows a consistent implementation pattern:
1. Request struct with optional fields for parameters (e.g., `src/api/spot/get_currency_pair_detail.rs`)
2. Builder methods for parameter configuration
3. `From<T> for Request` trait implementation to convert to generic request
4. Automatic HMAC signing for authenticated endpoints via `src/http/credentials.rs`

### 3. Module Organization
- `src/api/spot/`: All spot trading API endpoints
- `src/http/`: Core HTTP abstractions (request signing, methods, errors)
- `src/hyper/` & `src/ureq/`: Client-specific implementations
- `src/utils.rs`: Shared utilities (currently just URL generation)

### 4. Authentication
The SDK uses HMAC SHA-512 signing for authenticated requests. Credentials are managed through the `Credentials` struct, and signing logic is in `src/http/credentials.rs`.

### 5. Error Handling
Each client implementation has its own error type:
- `src/hyper/error.rs`: Async client errors
- `src/ureq/error.rs`: Sync client errors
- `src/http/error.rs`: Common HTTP-related errors

## Important Notes

1. **Feature Flags**: Always specify the correct feature when building or testing async functionality
2. **API Credentials**: Never commit API keys. Use environment variables instead (see TODO in sync_example.rs)
3. **Request Building**: Use the builder pattern for constructing API requests for better ergonomics
4. **Type Conversion**: The TODO.rs file indicates future plans to support non-String parameter types