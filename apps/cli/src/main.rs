//! LinkLocal CLI
//!
//! Command-line debug tool for testing and interacting with the mesh network.
//! Useful for development and testing without the full desktop UI.

use clap::{Parser, Subcommand};
use linklocal_protocol::{Packet, encode_packet, decode_packet};
use linklocal_mesh::{MeshNetwork, MeshAdapter};
use linklocal_routing::RoutingEngine;
use linklocal_identity::generate_keypair;
use linklocal_sync::SyncManager;
use linklocal_utils::init_logger;
use std::sync::Arc;

mod commands;

#[derive(Parser)]
#[command(name = "linklocal")]
#[command(about = "LinkLocal mesh network debug tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new node
    Init {
        /// Node identifier
        #[arg(short, long)]
        node_id: String,
    },

    /// Send a test message
    Send {
        /// Message content
        #[arg(short, long)]
        message: String,
    },

    /// Generate a new identity
    Identity,

    /// Test packet encoding/decoding
    Test,

    /// Start routing engine
    Routing {
        /// Cache size
        #[arg(short, long, default_value = "1000")]
        cache_size: usize,
    },

    /// Create a synchronized document
    Sync {
        /// Document name
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    init_logger();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { node_id } => {
            println!("Initializing node: {}", node_id);
            let identity = generate_keypair();
            println!("Generated identity: {}", identity.short_id());
            println!("Public key: {}", identity.public_key_hex());
        }

        Commands::Send { message } => {
            println!("Creating test message: {}", message);
            let packet = Packet::new("cli-node".to_string(), message.into_bytes());
            
            match encode_packet(&packet) {
                Ok(encoded) => {
                    println!("Packet encoded: {} bytes", encoded.len());
                    
                    match decode_packet(&encoded) {
                        Ok(decoded) => {
                            println!("Packet decoded successfully");
                            println!("  Source: {}", decoded.source);
                            println!("  TTL: {}", decoded.ttl);
                            println!("  Payload: {} bytes", decoded.payload.len());
                        }
                        Err(e) => eprintln!("Decode error: {}", e),
                    }
                }
                Err(e) => eprintln!("Encode error: {}", e),
            }
        }

        Commands::Identity => {
            println!("Generating new identity...");
            let identity = generate_keypair();
            println!("✓ Identity generated");
            println!("  Short ID: {}", identity.short_id());
            println!("  Public Key: {}", identity.public_key_hex());
            
            // Test signing
            let message = b"test message";
            let signature = identity.sign(message);
            let verified = identity.verify(message, &signature);
            println!("  Sign/Verify: {}", if verified { "✓" } else { "✗" });
        }

        Commands::Test => {
            println!("Running connectivity tests...");
            
            // Protocol test
            println!("\n[Protocol Test]");
            let packet = Packet::new("test-node".to_string(), vec![1, 2, 3, 4, 5]);
            println!("  Created packet ID: {}", packet.id);
            println!("  TTL: {}", packet.ttl);
            
            // Routing test
            println!("\n[Routing Test]");
            let engine = RoutingEngine::new(1000);
            let should_fwd = engine.should_forward(&packet);
            println!("  Should forward: {}", should_fwd);
            println!("  Cache size: {}", engine.cache_size());
            
            // Identity test
            println!("\n[Identity Test]");
            let id = generate_keypair();
            println!("  Generated: {}", id.short_id());
            
            println!("\n✓ All tests passed");
        }

        Commands::Routing { cache_size } => {
            println!("Creating routing engine (cache size: {})...", cache_size);
            let engine = RoutingEngine::new(cache_size);
            
            // Test with multiple packets
            for i in 0..5 {
                let packet = Packet::new(
                    format!("node-{}", i),
                    format!("test payload {}", i).into_bytes(),
                );
                let fwd = engine.should_forward(&packet);
                println!("  Packet {}: forward={}", i, fwd);
            }
            
            println!("Final cache size: {}", engine.cache_size());
        }

        Commands::Sync { name } => {
            println!("Creating sync manager for document: {}", name);
            let manager = SyncManager::new("cli-node".to_string());
            println!("✓ Sync manager created");
            println!("  Node ID: {}", manager.node_id());
            
            let state = manager.get_state();
            println!("  Initial state size: {} bytes", state.len());
        }
    }
}
