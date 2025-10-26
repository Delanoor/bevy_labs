### 🦀 Bevy Lab — Project Overview

<span style="font-size: 12px;">
A single Rust workspace (bevy_lab/) containing ten progressively complex mini-games.
Each game teaches a new Bevy 0.17 concept while reusing shared ECS systems from core_engine.
The goal: reach commercial-quality roguelite and meta-progression proficiency.

---
</span>

#### Project Descriptions
<span style="font-size: 12px;">

| # | Game | Core Focus | Description |
|:-:|------|-------------|--------------|
| **1** | **Square Chaser** | ECS · Input · Timers | Move a square to collect targets before time runs out. |
| **2** | **Bounce Engine** | Physics · Transform updates | Bouncing balls with velocity, friction, and edge collision. |
| **3** | **Pixel Dodge** | Spawning · Collision detection | Dodge falling obstacles; difficulty scales dynamically. |
| **4** | **Stoopid Clicker** | UI · Persistence | Idle clicker with upgrades and RON-based auto-save. |
| **5** | **Mini Slots** | Animation · Timing | Three-reel slot machine with easing-based spin animation. |
| **6** | **Cat Invaders** | Projectiles · Combat loop | Shoot waves of enemies and manage damage systems. |
| **7** | **Fish Factory Tycoon** | Resource loops · Timers | Build an idle production system with upgradable efficiency. |
| **8** | **Tiny Dungeon** | Procedural generation · XP | Explore randomly generated rooms, gain XP, and grow stronger. |
| **9** | **Stoopid Survivors** | Roguelite loop · Scaling | Survive endless waves; auto-attack and level up. |
| **10** | **Stoopia City** | Meta-progression · Save system | Central hub that connects all games and tracks global progress. |

---
</span>


#### Learning Goals

<small>
Each project focuses on a key Bevy concept:
- ECS architecture, plugins, and resources  
- Real-time input and physics systems  
- Animation and timing  
- Procedural generation  
- UI, save/load, and persistence  
- Scaling combat and roguelite progression  
- Meta-systems and modular worldbuilding  
</small>
---

#### Development Flow

1. <small>**Phase 1 — Fundamentals:** (01-03) Input, movement, collisions</small>
2. <small>**Phase 2 — Interface:** (04-05) UI, audio, animation</small>
3. <small>**Phase 3 — Combat/Simulation:** (06-07) Enemies, projectiles, systems</small>
4. <small>**Phase 4 — Roguelite:** (08-09) XP, scaling, upgrades</small>
5. <small>**Phase 5 — Integration:** (10) Hub world and persistence</small>  

---

## To run

```bash
# Build everything
cargo build --workspace

# Run an individual game
cargo run -p 01_square_chaser
```