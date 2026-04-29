# LinkLocal Architecture

## Overview

LinkLocal is a production-grade, offline-first desktop communication application built with a modular, event-driven Rust architecture. It uses BLE mesh networking, CRDT-based synchronization, and local-first data storage.

## Core Principles

1. **Offline-First**: Everything works without internet
2. **Peer-to-Peer**: No central server or coordinator
3. **Conflict-Free**: CRDT-based data structure ensures consistency
4. **Decentralized Identity**: Ed25519 keypairs for trust
5. **Energy Efficient**: BLE mesh for battery-constrained devices

## Architecture Layers

### 1. Protocol Layer (`core/protocol`)

**Responsibility**: Define packets and serialization

- **Packet Structure**: UUID, source, TTL, payload, timestamp
- **Serialization**: CBOR (Concise Binary Object Representation)
- **Rationale**: CBOR is ideal for BLE—compact, fast, self-describing

| Component | Purpose |
|-----------|---------|
| `packet.rs` | Packet definition with TTL management |
| `codec.rs` | CBOR encode/decode functions |
| `error.rs` | Protocol-level error types |

### 2. Mesh Layer (`core/mesh`)

**Responsibility**: Abstract transport layer

- **Interface**: `MeshAdapter` trait (async)
- **Implementations**: BLE, WiFi Direct, LoRa, TCP mock
- **Benefit**: Pluggable transports, easy to test with mocks

| Component | Purpose |
|-----------|---------|
| `adapter.rs` | `MeshAdapter` trait + `MockAdapter` |
| `lib.rs` | `MeshNetwork` coordinator |

### 3. Routing Layer (`core/routing`)

**Responsibility**: Prevent loops and duplicates

- **Deduplication**: HashSet of seen packet IDs
- **TTL Enforcement**: Drop packets with TTL=0
- **Forwarding Decision**: Based on dedup + TTL

| Component | Purpose |
|-----------|---------|
| `dedup.rs` | Packet ID tracking |
| `lib.rs` | `RoutingEngine` logic |

### 4. Identity Layer (`core/identity`)

**Responsibility**: Cryptographic identities

- **Algorithm**: Ed25519 (modern, fast, IETF standard)
- **Usage**: Sign messages, verify signatures
- **Trust Model**: Public key = identity; no PKI required

| Component | Purpose |
|-----------|---------|
| `identity.rs` | Keypair + sign/verify |
| `lib.rs` | Keypair generation |

### 5. Sync Layer (`core/sync`)

**Responsibility**: Distributed state management

- **CRDT**: Yrs (Yjs for Rust)
- **Model**: Eventual consistency without conflicts
- **Update Propagation**: Binary updates over mesh

| Component | Purpose |
|-----------|---------|
| `state.rs` | `SharedDocument` structure |
| `lib.rs` | `SyncManager` orchestration |

### 6. Utilities (`libs/utils`)

- Logging setup with `env_logger`
- Common error handling patterns

## Applications

### CLI (`apps/cli`)

**Purpose**: Debug and test tool

Commands:
- `init`: Initialize a node
- `send`: Send test message
- `identity`: Generate identity
- `test`: Run connectivity tests
- `routing`: Test routing engine
- `sync`: Create sync document

Usage:
```bash
linklocal init --node-id my-node
linklocal send --message "Hello mesh"
linklocal test
```

### Desktop (`apps/desktop`)

**Purpose**: User-facing application (Tauri + React)

Structure:
- Rust backend (`AppState`, `AppConfig`)
- React frontend (not included in core setup)
- IPC bridge for Rust↔JS communication

## Data Flow

```
User Input (Desktop/CLI)
    ↓
App Logic (AppState)
    ↓
Identity Sign (Ed25519)
    ↓
Protocol Encode (CBOR)
    ↓
Routing Check (Dedup + TTL)
    ↓
Mesh Broadcast (BLE/Mock)
    ↓
(Remote nodes repeat above)
    ↓
Sync Apply (CRDT)
    ↓
Database Store (SQLite - future)
```

## Dependency Graph

```
desktop
├── routing
│   ├── protocol
│   │   └── [serde_cbor, uuid]
│   └── [uuid, tokio]
├── sync
│   ├── protocol
│   └── [yrs, tokio]
├── mesh
│   └── [async-trait, tokio]
├── identity
│   └── [ed25519-dalek, rand]
└── utils
    └── [log, env_logger]

cli
└── [same as desktop]
```

## Design Patterns

### 1. Trait-Based Abstraction

**Example**: `MeshAdapter`

```rust
#[async_trait]
pub trait MeshAdapter: Send + Sync {
    async fn start(&self);
    async fn broadcast(&self, data: Vec<u8>);
    async fn receive(&self) -> Option<Vec<u8>>;
}
```

**Benefit**: Easy to swap implementations (BLE ↔ Mock)

### 2. Arc<Mutex<T>> for Shared State

**Example**: DedupCache

```rust
pub struct RoutingEngine {
    dedup_cache: Arc<Mutex<DedupCache>>,
}
```

**Benefit**: Thread-safe, ACID semantics

### 3. Error Handling with `thiserror`

**Example**:

```rust
#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("CBOR encoding failed: {0}")]
    EncodingError(String),
}

pub type Result<T> = std::result::Result<T, ProtocolError>;
```

**Benefit**: Ergonomic, consistent

### 4. Async/Await with Tokio

**Example**: Mesh operations

```rust
pub async fn broadcast(&self, data: Vec<u8>) {
    self.adapter.broadcast(data).await;
}
```

**Benefit**: Non-blocking, efficient resource use

## Testing Strategy

Each crate includes unit tests:

- **Protocol**: Packet creation, CBOR roundtrip
- **Mesh**: Mock adapter initialization
- **Routing**: Deduplication, TTL
- **Identity**: Keypair, sign/verify
- **Sync**: State updates, document version

```bash
cargo test --workspace
```

## Future Work

### Near-term

1. Real BLE adapter (nRF for Linux/Windows)
2. SQLite integration for persistence
3. Tauri IPC handlers
4. React frontend

### Medium-term

1. NAT traversal (for nodes behind firewalls)
2. Compression (for large sync updates)
3. Rate limiting (to prevent mesh floods)
4. Persistence layer abstraction

### Long-term

1. Multi-transport (BLE + WiFi fallback)
2. Sharded routing tables (for large meshes)
3. Access control (public/private documents)
4. End-to-end encryption

## References

- **CRDT**: Conflict-free Replicated Data Types (Yjs/Yrs)
- **BLE Mesh**: Bluetooth 5.0 Mesh Specification
- **Ed25519**: RFC 8032
- **CBOR**: RFC 8949
- **Tauri**: Secure fusion of Rust + Web
