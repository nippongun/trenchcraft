---
name: make-map
description: Build trenchcraft and convert a Minecraft schematic/litematic file to a TrenchBroom .map file
argument-hint: <input.(schem|schematic|litematic)> [output.map]
---

# Make TrenchBroom Map

Convert a Minecraft structure file to Quake/Valve `.map` format using trenchcraft.

## Arguments

`$ARGUMENTS`

Parse as: first argument = input file path, second argument (optional) = output `.map` path.
If no output path given, derive it by replacing the input file's extension with `.map`.

## Pipeline

The trenchcraft pipeline is:

```
nbt_unpack → parser → filter → optimizer → exporter
```

1. **Decompress** gzip NBT (or raw NBT fallback)
2. **Parse** `.schematic`/`.schem` (fastnbt) or `.litematic` (rustmatica) → `VoxelMap`
3. **Filter** air/structure_void; rewrite names `minecraft:stone` → `textures/minecraft/stone`
4. **Optimize** greedy mesh for full blocks; shaped brushes for slabs/panes/fences
5. **Export** Valve 220 `.map` (32 Quake units per Minecraft block; Z↑)

## Steps

1. Build the project in release mode:
   ```
   cargo build --release
   ```

2. Run the conversion:
   ```
   cargo run --release -- <input> <output>
   ```

3. Report the brush count and output path from the conversion log.

4. If conversion fails, diagnose from the error:
   - "Failed to load" → check the file exists and is a valid schematic
   - "Failed to parse" → the NBT structure may be an unsupported version; check file extension matches actual format
   - Compile error → run `cargo clippy` and fix warnings/errors first

## Notes

- Coordinate mapping: Minecraft X→Quake X, Minecraft Z→Quake Y, Minecraft Y→Quake Z
- Scale: 1 Minecraft block = 32×32×32 Quake units
- Slabs: exported as half-height brushes (16 units)
- Glass panes / iron bars / fences: exported as 4-unit-wide thin panel brushes
- Full blocks use greedy meshing — expect far fewer brushes than input blocks
- `.litematic` files lose block state properties (empty properties map); shape detection falls back to `Full` for all blocks
