# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

**Trenchcraft** is a Rust CLI tool that converts Minecraft schematic/structure files (`.schematic`, `.schem`, `.litematic`) into Quake/Valve `.map` format for use with TrenchBroom.

## Commands

```bash
cargo build                          # Debug build
cargo build --release                # Optimized build
cargo run -- <input> <output>        # Run (e.g. cargo run -- foo.schem out.map)
cargo test                           # Run tests
cargo clippy                         # Lint
```

## Architecture

The code is a linear data processing pipeline:

```
nbt_unpack.rs → parser.rs → filter.rs → optimizer.rs → exporter.rs
```

1. **`nbt_unpack.rs`** — Decompresses gzip NBT with fallback to raw NBT; returns raw bytes for parsing.
2. **`parser.rs`** — Parses NBT bytes (`.schematic`/`.schem`) or uses `rustmatica` (`.litematic`) into a `VoxelMap` — a flat `Vec<Option<Block>>` indexed as `[x + width * (y * length + z)]`.
3. **`filter.rs`** — Removes air/structure_void blocks; rewrites block names from `minecraft:stone` to `textures/minecraft/stone`.
4. **`optimizer.rs`** — Greedy meshing: expands voxels into maximal axis-aligned rectangular `Brush` volumes (X → Z → Y), marking consumed cells to avoid reuse.
5. **`exporter.rs`** — Writes Valve 220 `.map` format. Each `Brush` becomes a worldspawn entity. Minecraft→Quake coordinate mapping uses 32 units/block; Y-axis is inverted.
6. **`main.rs`** — CLI via clap derive macros; detects format from file extension and orchestrates the pipeline.

### Key types

- `Block` — wraps a block name string
- `VoxelMap` — 3D grid with `width`, `height`, `length` dimensions
- `Brush` — axis-aligned box `(x0,y0,z0)→(x1,y1,z1)` with a texture string

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `fastnbt` | `.schematic`/`.schem` NBT parsing |
| `flate2` | GZIP decompression |
| `mcdata` | Minecraft block ID/metadata mapping |
| `rustmatica` | `.litematic` format parsing |
