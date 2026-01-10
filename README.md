# NFT Adventurers

A blockchain-integrated RPG game featuring adventurers, missions, and dynamic asset generation.

## Overview

NFT Adventurers combines a Rust-based game client (Macroquad) with a robust backend (Axum) to deliver a persistent world where players can recruit adventurers, send them on missions, and manage their holdings.

## Architecture

- **Client**: Macroquad + macroquad-toolkit (Rust → WASM)
- **Backend**: Axum (Rust) + PostgreSQL
- **Hosting**: Local development
- **Art**: Stable Diffusion API for generated assets
- **Auth**: Wallet signature verification

See [TECHNICAL_IMPLEMENTATION.md](TECHNICAL_IMPLEMENTATION.md) for detailed architecture and design documentation.

## Project Structure

- `client/`: Macroquad game client source code.
- `backend/`: Axum API server and game logic.
- `shared/`: Shared types and library code used by both client and backend.
- `assets/`: Game assets and configuration files.

## Getting Started

### Prerequisites

- Rust toolchain (stable)
- PostgreSQL (running locally)

### Running the Project

This project uses a Cargo workspace. You can run individual components using `-p`.

**Run the Client:**
```bash
cargo run -p client
```

**Run the Backend:**
```bash
cargo run -p backend
```
