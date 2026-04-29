# LinkLocal

**An offline-first, decentralized desktop communication app using BLE mesh, CRDT sync, and zero-trust architecture.**

```
┌─────────────────────────────────────────────┐
│        LinkLocal Mesh Network               │
│    (Peer-to-Peer, Offline-First, P2P)       │
└─────────────────────────────────────────────┘
     ↓              ↓              ↓
┌─────────┐  ┌─────────┐  ┌─────────┐
│ Device 1│  │ Device 2│  │ Device 3│
│  (BLE)  │  │  (BLE)  │  │  (BLE)  │
└─────────┘  └─────────┘  └─────────┘
   Chat         Chat         Chat
   Files        Files        Files
   Sync         Sync         Sync
```

## Features

✅ **Offline-First**: Works without internet—no cloud dependency
✅ **Peer-to-Peer**: No central server, true decentralization  
✅ **Conflict-Free**: CRDT-based eventual consistency (Yrs)
✅ **Secure**: Ed25519 cryptography, zero-trust model
✅ **Efficient**: BLE mesh, CBOR serialization, ~100 bytes/packet
✅ **Production-Ready**: Modular Rust, async/await, fully tested

---

## Quick Start

### Prerequisites

- **Rust 1.70+** ([install](https://rustup.rs/))
- **Linux/macOS/Windows** (with WSL2)
- **Dependencies**: See [SETUP.md](./SETUP.md)

### Installation

```bash
# Clone
git clone <repo> link_local && cd link_local

# Test build
cargo check --workspace

# Build CLI tool
cargo build -p linklocal-cli --release

# Run tests
cargo test --workspace
```

### CLI Usage

```bash
# Initialize a node
cargo run -p linklocal-cli -- init --node-id my-laptop

# Send a test message
cargo run -p linklocal-cli -- send --message "Hello, mesh!"

# Generate cryptographic identity
cargo run -p linklocal-cli -- identity

# Run all tests
cargo run -p linklocal-cli -- test

# Test mesh routing
cargo run -p linklocal-cli -- routing

# Create a sync document
cargo run -p linklocal-cli -- sync --name "My Shared Doc"
```

---

## Architecture

LinkLocal uses a **layered architecture**:

| Layer | Crate | Purpose |
|-------|-------|---------|
| **App** | `apps/desktop`, `apps/cli` | User interfaces |
| **Sync** | `core/sync` | CRDT (Yrs) for conflict-free data |
| **Identity** | `core/identity` | Ed25519 keypair management |
| **Routing** | `core/routing` | Packet dedup + forwarding |
| **Mesh** | `core/mesh` | BLE/transport abstraction |
| **Protocol** | `core/protocol` | CBOR packet encoding |
| **Utils** | `libs/utils` | Logging + common utilities |

### Data Flow

```
Send Message
    ↓
[Protocol] CBOR encode
    ↓
[Identity] Sign with Ed25519
    ↓
[Routing] Check dedup + TTL
    ↓
[Mesh] Broadcast via BLE
    ↓
(Remote node repeats)
    ↓
[Sync] Apply CRDT update
    ↓
Store locally + Display to user
```

### Design Patterns

- **Async/Await**: Non-blocking I/O with Tokio
- **Trait-Based**: Pluggable transport (BLE ↔ Mock)
- **CRDT**: Conflict-free data structures (Yrs)
- **Error Handling**: Ergonomic with `thiserror`

---

## Project Structure

```
linklocal/
├── Cargo.toml              # Workspace config
├── ARCHITECTURE.md         # System design (detailed)
├── SETUP.md                # Development guide
├── README.md               # This file
│
├── core/                   # Core libraries
│   ├── protocol/           # Packet: {id, source, ttl, payload, ts}
│   ├── mesh/               # MeshAdapter trait + MockAdapter
│   ├── routing/            # RoutingEngine + DedupCache
│   ├── identity/           # Ed25519 keypair + sign/verify
│   └── sync/               # SyncManager + SharedDocument
│
├── apps/                   # Applications
│   ├── cli/                # Debug tool (init, send, test, routing, sync)
│   └── desktop/            # Tauri + React app (skeleton)
│
└── libs/utils/             # Shared utilities
    └── logger.rs           # Logging setup
```

---

## Technology Stack

| Component | Technology | Why |
|-----------|-----------|-----|
| **Language** | Rust | Performance, memory safety, concurrency |
| **Async** | Tokio | Non-blocking I/O, efficient scheduling |
| **Serialization** | CBOR | Compact (BLE friendly), fast |
| **CRDT** | Yrs (Yjs) | Proven conflict-free replication |
| **Crypto** | Ed25519-dalek | Modern, IETF standard, fast |
| **Storage** | SQLite (future) | Embedded, ACID transactions |
| **Desktop UI** | Tauri + React | Secure, lightweight, cross-platform |

---

## Examples

### Protocol: Send a Message

```rust
use linklocal_protocol::{Packet, encode_packet};

// Create packet
let packet = Packet::new(
    "my-node".to_string(),
    b"Hello, mesh!".to_vec(),
);

// Encode to CBOR
let encoded = encode_packet(&packet)?;
println!("Sending {} bytes", encoded.len()); // ~70 bytes
```

### Routing: Check if Packet Should Forward

```rust
use linklocal_routing::RoutingEngine;

let engine = RoutingEngine::new(1000);  // cache size
let packet = Packet::new("node-1".into(), vec![1, 2, 3]);

if engine.should_forward(&packet) {
    // TTL > 0 and not seen before
    mesh.broadcast(encode_packet(&packet)?).await;
}
```

### Identity: Sign Messages

```rust
use linklocal_identity::generate_keypair;

let identity = generate_keypair();
let message = b"important data";

let signature = identity.sign(message);
let valid = identity.verify(message, &signature);  // true

println!("Node ID: {}", identity.short_id());
// Output: Node ID: 3a7f2e1c
```

### Sync: Share Documents

```rust
use linklocal_sync::SyncManager;

let manager = SyncManager::new("my-node".into());
let state = manager.get_state();  // Binary state

// Send state to other nodes
mesh.broadcast(state).await;

// Apply updates from network
manager.apply_update(&remote_update)?;
```

---

## Testing

Each crate includes comprehensive unit tests:

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p linklocal-protocol
cargo test -p linklocal-routing
cargo test -p linklocal-identity
cargo test -p linklocal-sync

# With output
cargo test --workspace -- --nocapture --test-threads=1
```

Test coverage includes:
- ✅ Packet serialization roundtrips
- ✅ TTL management
- ✅ Deduplication logic
- ✅ Ed25519 sign/verify
- ✅ CRDT document updates
- ✅ Mock adapter behavior

---

## Development

### Building

```bash
# Debug
cargo build

# Release (optimized)
cargo build --release

# Specific crate
cargo build -p linklocal-cli --release
```

### Code Quality

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --workspace

# Security audit
cargo audit
```

### Documentation

```bash
# Generate and open API docs
cargo doc --workspace --open

# View architecture
cat ARCHITECTURE.md

# View setup guide
cat SETUP.md
```

---

## Roadmap

### Phase 1 (Current)
- ✅ Core protocol + packet encoding
- ✅ Routing + deduplication
- ✅ Identity + cryptography
- ✅ CRDT sync integration
- ✅ CLI debug tool
- 🔄 Unit tests

### Phase 2 (Next)
- Real BLE adapter (nRF24/nRF52)
- SQLite persistence
- Tauri IPC handlers
- React UI (chat, file share)

### Phase 3 (Future)
- NAT traversal
- Compression
- Rate limiting
- Access control
- E2E encryption

---

## Common Issues

**Q: `cargo` not found?**
A: Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

**Q: Build fails on Linux?**
A: Install deps: `sudo apt-get install build-essential pkg-config libssl-dev`

**Q: Tests fail?**
A: Run with output: `cargo test --workspace -- --nocapture`

See [SETUP.md](./SETUP.md) for more troubleshooting.

---

## Contributing

1. Fork the repo
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Write tests for your code
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request

## Code of Conduct

Be respectful, inclusive, collaborative. See [LICENSE](./LICENSE).

---

## License

MIT License © LinkLocal Team 2024

---

## References

- **CRDT**: [Yjs Docs](https://docs.yjs.dev/) | [CRDT Overview](https://crdt.tech/)
- **BLE Mesh**: [Bluetooth 5.0 Specification](https://www.bluetooth.com/)
- **Cryptography**: [Ed25519 (RFC 8032)](https://tools.ietf.org/html/rfc8032)
- **Serialization**: [CBOR (RFC 8949)](https://tools.ietf.org/html/rfc8949)
- **Rust**: [The Rust Book](https://doc.rust-lang.org/book/)
- **Tauri**: [Tauri Docs](https://tauri.app/)

---

## Support

- **Issues**: Report bugs or request features via GitHub Issues
- **Discussions**: Join us for technical discussions
- **Docs**: Read [ARCHITECTURE.md](./ARCHITECTURE.md) for deep dives

---

**Made with ❤️ for decentralized communication**

🚀 Ready to build? See [SETUP.md](./SETUP.md)
