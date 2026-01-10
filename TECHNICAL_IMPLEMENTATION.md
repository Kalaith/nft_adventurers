# NFT Adventurers: Technical Implementation Guide

## Architecture

**Client**: Macroquad + macroquad-toolkit (Rust → WASM)  
**Backend**: Axum (Rust) + PostgreSQL  
**Hosting**: Local development (production deployment later)  
**Art**: Stable Diffusion API for generated assets  
**Auth**: Wallet signature verification (Phantom/MetaMask)

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  BROWSER                                                        │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  Macroquad Client (WASM)                                  │ │
│  │  - UI, rendering, animations                              │ │
│  │  - Wallet connection for signing                          │ │
│  │  - REST API calls                                         │ │
│  └─────────────────────────┬─────────────────────────────────┘ │
└────────────────────────────┼────────────────────────────────────┘
                             │ HTTPS
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│  BACKEND SERVER                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  Axum (Rust)                                              │ │
│  │  - Wallet signature auth                                  │ │
│  │  - Game logic, mission resolution                         │ │
│  │  - Feat generation                                        │ │
│  └─────────────────────────┬─────────────────────────────────┘ │
│  ┌─────────────────────────┴─────────────────────────────────┐ │
│  │  PostgreSQL                                               │ │
│  │  - Players, adventurers, items, holds                     │ │
│  │  - Append-only feat_ledger                                │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

## Project Structure

```
nft_adventurers/
├── Cargo.toml              # Workspace manifest
├── GDD.md
├── TECHNICAL_IMPLEMENTATION.md
│
├── client/                 # Macroquad game client
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── game.rs
│   │   ├── api/            # REST client
│   │   ├── state/          # Screen states
│   │   ├── ui/             # UI components
│   │   └── data/           # Type definitions
│   └── assets/
│       ├── classes.json
│       ├── missions.json
│       └── buildings.json
│
├── backend/                # Axum API server
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── auth.rs
│   │   ├── db/
│   │   ├── models/
│   │   ├── handlers/
│   │   └── engine/
│   └── migrations/
│
└── shared/                 # Shared types between client/backend
    ├── Cargo.toml
    └── src/lib.rs
```

---

## Core Data Types (shared)

```rust
// Adventurer
pub struct Adventurer {
    pub id: Uuid,
    pub name: String,
    pub class: Class,
    pub level: u32,
    pub xp: u32,
    pub stats: Stats,
    pub skills: Vec<SkillId>,
    pub status: Status,
}

pub enum Class { Warrior, Mage, Cleric }
pub enum Status { Healthy, Injured, OnMission, Dead }

// Item with Feats
pub struct Item {
    pub id: Uuid,
    pub base_type: ItemType,
    pub current_name: String,
    pub rarity: Rarity,
    pub feats: Vec<Feat>,
}

pub struct Feat {
    pub name: String,
    pub source: FeatSource,
    pub bonuses: FeatBonuses,
    pub timestamp: DateTime,
}

// Hold
pub struct Hold {
    pub buildings: HashMap<Building, u32>,
    pub echoes: Vec<Echo>,
    pub total_feats: u32,
}

pub enum Building { Hearth, TrainingYard, FeatAnvil }
```

---

## API Endpoints

| Method | Endpoint | Purpose |
|:-------|:---------|:--------|
| POST | `/auth/challenge` | Get signing nonce |
| POST | `/auth/verify` | Verify signature, get session |
| GET | `/player/me` | Get all player data |
| POST | `/adventurer/mint` | Create adventurer |
| POST | `/mission/start` | Start a mission |
| POST | `/mission/resolve` | Complete mission |
| POST | `/hold/upgrade` | Upgrade building |
| GET | `/ledger/{id}` | View feat history |

---

## Database Schema

```sql
CREATE TABLE players (
    wallet_address TEXT PRIMARY KEY,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE adventurers (
    id UUID PRIMARY KEY,
    owner TEXT REFERENCES players(wallet_address),
    name TEXT,
    class TEXT,
    level INT DEFAULT 1,
    xp INT DEFAULT 0,
    stats JSONB,
    skills TEXT[],
    status TEXT DEFAULT 'healthy'
);

CREATE TABLE items (
    id UUID PRIMARY KEY,
    owner TEXT REFERENCES players(wallet_address),
    base_type TEXT,
    current_name TEXT,
    rarity TEXT,
    equipped_by UUID REFERENCES adventurers(id)
);

CREATE TABLE feat_ledger (
    id SERIAL PRIMARY KEY,
    entity_type TEXT,
    entity_id UUID,
    feat_name TEXT,
    bonuses JSONB,
    timestamp TIMESTAMP DEFAULT NOW()
    -- Append-only: no updates or deletes
);

CREATE TABLE holds (
    player TEXT PRIMARY KEY REFERENCES players(wallet_address),
    buildings JSONB DEFAULT '{"hearth": 1}',
    echoes JSONB DEFAULT '[]'
);

CREATE TABLE active_missions (
    id UUID PRIMARY KEY,
    player TEXT REFERENCES players(wallet_address),
    mission_type TEXT,
    party UUID[],
    start_time TIMESTAMP,
    duration_seconds INT
);
```

---

## Real-Time Idle System

1. **Start**: Server records `start_time` in database
2. **Progress**: Client calculates `(now - start_time) / duration`
3. **Complete**: When elapsed >= duration, client calls `/mission/resolve`
4. **Offline**: On reconnect, server returns all completable missions

---

## Art Generation

Using Stable Diffusion API for:
- Adventurer portraits (per class)
- Item icons (evolve with feats)
- Hold building visuals
- Mission scene backgrounds

Prompts stored in `assets/image_prompts.json`.

---

## Implementation Phases

### Phase 1: Scaffold
- [ ] Cargo workspace setup
- [ ] Basic Axum server
- [ ] Basic Macroquad client
- [ ] Shared types crate

### Phase 2: Auth
- [ ] Wallet signature verification
- [ ] Connect wallet UI
- [ ] Session management

### Phase 3: Entities
- [ ] Database migrations
- [ ] CRUD endpoints
- [ ] Display in client

### Phase 4: Missions
- [ ] Start/resolve flow
- [ ] Timer UI
- [ ] Feat generation

### Phase 5: Progression
- [ ] Hold upgrades
- [ ] Skill trees
- [ ] Consumables

### Phase 6: Art
- [ ] Stable Diffusion integration
- [ ] Generate class portraits
- [ ] Item icon evolution
