# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

**Trenchcraft** is a Rust CLI tool that converts Minecraft schematic/structure files (`.schematic`, `.schem`, `.litematic`) into Quake/Valve `.map` format for use with TrenchBroom.

## Commands

```bash
cargo build                                      # Debug build
cargo build --release                            # Optimized build
cargo run -- <input> <output>                    # Run (e.g. cargo run -- foo.schem out.map)
cargo run -- <input> <output> --greedy all       # Full shape-aware merging (default)
cargo run -- <input> <output> --greedy full-only # Full blocks only
cargo run -- <input> <output> --greedy none      # No merging (debug)
cargo test                                       # Run tests
cargo clippy                                     # Lint
```

## Architecture

The code is a linear data processing pipeline:

```
nbt_unpack.rs → parser.rs → filter.rs → optimizer.rs → exporter.rs
```

1. **`nbt_unpack.rs`** — Decompresses gzip NBT with fallback to raw NBT; returns raw bytes for parsing.
2. **`parser.rs`** — Parses NBT bytes (`.schematic`/`.schem`) or uses `rustmatica` (`.litematic`) into a `VoxelMap` — a flat `Vec<Option<Block>>` indexed as `[x + width * (y * length + z)]`.
3. **`filter.rs`** — Removes air/structure_void blocks; rewrites block names from `minecraft:stone` to `textures/minecraft/stone`.
4. **`optimizer.rs`** — Shape-aware greedy meshing. Full blocks expand X→Z→Y; slabs expand X→Z; NS panels expand Z→Y; EW panels expand X→Y. Stores Quake-unit coords directly (Minecraft X→Quake X, Z→Y, Y→Z, ×32). Controlled by `GreedyLevel` enum (`none`/`full-only`/`all`).
5. **`exporter.rs`** — Writes Valve 220 `.map` format. Each `Brush` becomes a worldspawn brush. Receives Quake-unit coords directly from optimizer — no coordinate math here.
6. **`main.rs`** — CLI via clap derive macros; detects format from file extension and orchestrates the pipeline.

### Key types

- `Block` — block name string + `properties: HashMap<String, String>` (parsed from `[key=value,...]` state suffix)
- `VoxelMap` — 3D grid with `width`, `height`, `length`; blocks indexed as `[y*(w*l) + z*w + x]`
- `Brush` — axis-aligned box with Quake-unit `min`/`max` (i32 tuples) and a texture string
- `BlockShape` — `Full | SlabBottom | SlabTop | ThinPanelNS | ThinPanelEW | ThinCross`
- `GreedyLevel` — `None | FullOnly | All` (clap `ValueEnum`, lives in `optimizer.rs`)

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `fastnbt` | `.schematic`/`.schem` NBT parsing |
| `flate2` | GZIP decompression |
| `mcdata` | Minecraft block ID/metadata mapping |
| `rustmatica` | `.litematic` format parsing |
