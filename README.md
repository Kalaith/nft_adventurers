# NFT Adventurers

NFT Adventurers is an idle RPG and roster-management game about recruiting adventurers, sending them on missions, and building a persistent party over time.

The focus is on choosing who goes where, reading mission risk, and growing a collection of heroes with useful roles.

## Gameplay

- Recruit and inspect adventurers.
- Send parties on missions for rewards and progress.
- Manage a roster across availability, strength, and mission needs.
- Improve holdings and prepare for harder challenges.
- Track long-term growth across repeated expeditions.

## Goal

Build a capable adventuring roster that can clear better missions and produce stronger rewards over time.

## Controls

- Mouse: navigate menus and manage adventurers.
- Esc: back or pause.

## Current Scope

Playable roster-management foundation with adventurers, missions, progression, and persistent campaign flow.
# Practical Future Improvements

- Add typed contract tests between backend, shared models, and client screens for recruit, inventory, market, mission, and skills flows.
- Centralize item, skill, adventurer, and equipment identifiers in shared data so backend and client cannot silently diverge.
- Add mission-resolution fixtures for reward, injury, level-up, and inventory-capacity edge cases.
- Split client screen mutation from network/domain actions so UI panels can be tested with mocked backend responses.

