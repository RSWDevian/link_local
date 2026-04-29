# LinkLocal Setup Guide

## Prerequisites

### System Requirements

- **OS**: Linux, macOS, or Windows (with WSL2)
- **Rust**: 1.70+ (Install via [rustup.rs](https://rustup.rs/))
- **Cargo**: Comes with Rust

#### Linux Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora
sudo dnf install gcc pkg-config openssl-devel

# macOS
brew install pkg-config openssl
```

#### BLE Development (Optional, for real mesh implementation)

```bash
# Linux
sudo apt-get install libdbus-1-dev libudev-dev

# macOS - native support
# Windows - native support
```

## Installation

### 1. Clone the Repository

```bash
git clone <repository-url> link_local
cd link_local
```

### 2. Verify Rust Installation

```bash
rustc --version
cargo --version
```

Expected output:
```
rustc 1.70.0 (...)
cargo 1.70.0 (...)
```

### 3. Build the Project

```bash
# Check for compilation errors
cargo check --workspace

# Build all crates
cargo build --release

# This produces:
# - target/release/linklocal (CLI binary)
# - core libraries available for linking
```

### 4. Run Tests

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run tests for a specific crate
cargo test -p linklocal-protocol
cargo test -p linklocal-routing
cargo test -p linklocal-identity
```

## Project Structure

```
linklocal/
├── Cargo.toml                 # Workspace configuration
├── ARCHITECTURE.md            # System design document
├── SETUP.md                   # This file
├── README.md                  # Getting started
│
├── core/                      # Core library crates
│   ├── protocol/              # Packet encoding/decoding
│   ├── mesh/                  # Transport layer (async trait)
│   ├── routing/               # Packet forwarding & dedup
│   ├── identity/              # Cryptographic identities
│   └── sync/                  # CRDT synchronization
│
├── apps/                      # Applications
│   ├── cli/                   # Debug CLI tool
│   └── desktop/               # Tauri + React (backend)
│
├── libs/                      # Shared libraries
│   └── utils/                 # Logging, helpers
│
└── docs/                      # Documentation (future)
```

## Common Tasks

### Build CLI Tool

```bash
cargo build -p linklocal-cli --release

# Run directly
./target/release/linklocal --help

# With cargo
cargo run -p linklocal-cli -- --help
```

### Run CLI Commands

```bash
# Initialize a node
cargo run -p linklocal-cli -- init --node-id my-device

# Send a test message
cargo run -p linklocal-cli -- send --message "Hello mesh"

# Generate cryptographic identity
cargo run -p linklocal-cli -- identity

# Run tests
cargo run -p linklocal-cli -- test

# Test routing engine
cargo run -p linklocal-cli -- routing --cache-size 5000

# Create sync document
cargo run -p linklocal-cli -- sync --name "MyDocument"
```

### Run Protocol Tests

```bash
cargo test -p linklocal-protocol

# Tests verify:
# - Packet creation
# - TTL management
# - CBOR serialization roundtrip
```

### Run Routing Tests

```bash
cargo test -p linklocal-routing

# Tests verify:
# - Deduplication
# - TTL enforcement
# - Cache management
```

### Run Identity Tests

```bash
cargo test -p linklocal-identity

# Tests verify:
# - Keypair generation
# - Message signing
# - Signature verification
```

## Development Workflow

### 1. Create a New Feature

```bash
# Example: Add NAT traversal to routing

# 1. Create feature branch (or edit in workspace)
git checkout -b feature/nat-traversal

# 2. Implement in appropriate crate
# Edit core/routing/src/nat.rs

# 3. Export from lib.rs
vi core/routing/src/lib.rs

# 4. Add tests
# Add tests in core/routing/src/nat.rs

# 5. Run tests
cargo test -p linklocal-routing

# 6. Commit and push
git add .
git commit -m "feat(routing): Add NAT traversal support"
git push origin feature/nat-traversal
```

### 2. Format Code

```bash
# Format all code
cargo fmt --all

# Check formatting without changes
cargo fmt --all -- --check
```

### 3. Lint Code

```bash
# Check with Clippy
cargo clippy --workspace --all-targets

# Fix common issues
cargo clippy --workspace --all-targets --fix
```

### 4. Check Dependencies

```bash
# Outdated dependencies
cargo outdated

# Security vulnerabilities
cargo audit
```

## Building for Different Targets

### Linux

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

### macOS

```bash
cargo build --release --target x86_64-apple-darwin    # Intel
cargo build --release --target aarch64-apple-darwin   # Apple Silicon
```

### Windows

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

## Troubleshooting

### Issue: `cargo` command not found

**Solution**: Install Rust using rustup
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue: Compilation fails with "missing dependency"

**Solution**: Update dependencies
```bash
cargo update
```

### Issue: Tests fail with "mutex poisoned"

**This is expected in development**. It means a test panicked while holding a lock.

**Solution**: Ensure all tests use proper error handling
```rust
let lock = mutex.lock().expect("mutex poisoned");  // ✓ Correct
let lock = mutex.lock().unwrap(); // ✗ Can panic
```

### Issue: BLE not working on Linux

**Solution**: Ensure required packages are installed
```bash
sudo apt-get install libdbus-1-dev libudev-dev
```

## Performance Profiling

### Benchmark Protocol Encoding

```bash
cargo run --release -p linklocal-cli -- test
```

### Memory Usage

```bash
/usr/bin/time -v target/release/linklocal test
```

### Profiling (via perf on Linux)

```bash
cargo install flamegraph
cargo flamegraph -p linklocal-cli -- test
```

## Documentation

### Generate Rust Docs

```bash
cargo doc --workspace --open

# Docs open in browser
# Navigate to: crates/linklocal_protocol/index.html
```

### View Architecture Docs

```bash
cat ARCHITECTURE.md
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Test
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --workspace
```

## Next Steps

1. **Read** [ARCHITECTURE.md](./ARCHITECTURE.md) for system design
2. **Explore** [apps/cli](./apps/cli) for command examples
3. **Review** [core/protocol](./core/protocol) for packet format
4. **Implement** BLE adapter in [core/mesh](./core/mesh)
5. **Build** React frontend in [apps/desktop](./apps/desktop)

## Getting Help

- **Rust Book**: https://doc.rust-lang.org/book/
- **Tokio Guide**: https://tokio.rs/
- **Tauri Docs**: https://tauri.app/
- **Yjs Docs**: https://docs.yjs.dev/

## License

This project is licensed under the MIT License. See LICENSE file for details.
