# NFT Adventurers: Legends Forged in Chain

## Game Design Document

**Genre**: Real-Time Idle RPG with Cryptographic Ownership  
**Platform**: WebGL (WASM) + Native Windows  
**Engine**: Macroquad + macroquad-toolkit (client) / Axum (backend)

---

## Vision Statement

A D&D-inspired idle adventure game where adventurers, loot, and holds are persistent entities with cryptographically-verified ownership. Every kill, every victory, every death etches an eternal "Feat" into your items and heroes. **Dead is Dead** – no revives, only legends that live on through the gear they wielded.

---

## Core Pillars

| Pillar | Description |
|:-------|:------------|
| **Permanent Consequence** | Permadeath creates real stakes. Dead heroes are gone forever, but their feats empower surviving gear. |
| **Living History** | Items evolve through use – a sword that kills a dragon becomes "Dragon-Piercer." Kill a god? "God-Slayer." |
| **Hold vs Party Dilemma** | Invest in your permanent Hold (base) for long-term buffs, or gamble on party gear for immediate power. |
| **Free-to-Start** | Players begin with a free adventurer and basic sword. No paywall to experience the core loop. |

---

## Core Gameplay Loop

```
┌─────────────────────────────────────────────────────────────────┐
│  1. CLAIM/RECRUIT  →  2. EQUIP & PARTY  →  3. SEND ON MISSION  │
│         ↑                                           │           │
│         │                                           ↓           │
│  5. INVEST IN HOLD  ←  4. RESOLVE: LOOT/FEATS/DEATH            │
└─────────────────────────────────────────────────────────────────┘
```

1. **Claim Free Hero**: New players get a Common Adventurer + Rusty Sword
2. **Party & Equip**: Synergize heroes with Feat-boosted gear
3. **Risk Missions**: Choose peril level (1-10). Higher = more permadeath chance, better rewards
4. **Resolve**: Success appends Feats. Failure may result in permadeath
5. **Eternal Legacy**: Dead hero's gear gains "Fallen Comrade" Feat. Hold accumulates passive buffs

---

## v1.0 Content Scope

### Classes (3)

| Class | Role | Key Stat | Signature Ability |
|:------|:-----|:---------|:------------------|
| Warrior | Tank/DPS | STR | Taunt (protects allies) |
| Mage | Ranged/CC | INT | Fireball (AoE damage) |
| Cleric | Healer/Support | CHA | Heal (restores HP mid-mission) |

Each class has a 5-node skill tree.

### Missions (3)

| Type | Duration | Permadeath % | Reward |
|:-----|:---------|:-------------|:-------|
| Quick Skirmish | 1-4 hours | 0% | 1x (Common loot) |
| Dungeon Crawl | 12 hours | 10-25% | 3x (Rare loot) |
| Boss Raid | 24 hours | 40-60% | 10x (Epic loot) |

**Real-time idle**: Missions run in actual clock time, even when offline.

### Hold Buildings (3)

| Building | Effect |
|:---------|:-------|
| Hearth (Starter) | +5% XP, basic rest |
| Training Yard | +10% skill options, dummy fights |
| Feat Anvil | +8% XP multiplier, fusion |

### Consumables (3)

| Item | Effect |
|:-----|:-------|
| Health Potion | +50% HP restore mid-mission |
| Fire Resistance | 80% fire damage resist |
| Peril Veil | -15% permadeath chance |

### Legendary Ledger (Feats)

Items accumulate Feats based on their history:

```
Iron Sword → (5 Goblin kills) → "Gob-Bane" (+10% vs Goblins)
           → (Kill Dragon) → "Dragon-Slayer" (+30% Fire DMG)
```

Feats are stored in an append-only ledger – immutable and permanent.

---

## Roadmap

| Version | Content |
|:--------|:--------|
| **v1.0** | Core loop, 3 classes, 3 missions, 3 buildings, basic feats |
| **v1.1** | Pets, taming, more missions |
| **v1.2** | PvP Honor Arenas, Death Duels |
| **v2.0** | Guilds, raid content |
