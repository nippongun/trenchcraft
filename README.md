# Trenchcraft

Trenchcraft is a robust, blazingly-fast command-line tool written in Rust for converting Minecraft schematic and structure files into standard Quake/Valve `.map` format for use with [TrenchBroom](https://trenchbroom.github.io/)!

## Features

- **Multi-Format Support**: Parses standard Minecraft `.schematic` and `.schem` files alongside modern Fabric `.litematic` files.
- **Smart Material Filtering**: Automatically culls empty `minecraft:air` blocks to prevent invisible garbage geometry from crashing the BSP compiler.
- **Shape-Aware Greedy Meshing**: Consolidates thousands of individual blocks into large optimized brushes. Full blocks use 3-axis greedy expansion; slabs, glass panes, iron bars, and fences are each expanded along their geometrically valid axes. Configurable via `--greedy <none|full-only|all>`.
- **Valve 220 Map Configuration**: Outputs geometry using the modern Valve 220 Map format specification. This guarantees pixel-perfect UV mapping scaling in Trenchbroom.

## Usage

```
trenchcraft <input> <output> [--greedy <none|full-only|all>]
```

| Argument | Description |
|----------|-------------|
| `input`  | Path to a `.schem`, `.schematic`, or `.litematic` file |
| `output` | Path to write the `.map` file |
| `--greedy` | Meshing level — `none`, `full-only`, or `all` (default) |

### Examples

```bash
# Basic conversion
cargo run --release -- build.schem out.map

# Maximum merging (default)
cargo run --release -- build.schem out.map --greedy all

# Full blocks only — skips slab/pane merging
cargo run --release -- build.schem out.map --greedy full-only

# No merging — one brush per block, useful for debugging
cargo run --release -- build.schem out.map --greedy none
```

> **Textures**: Trenchcraft assigns `textures/minecraft/<blockname>` paths to all geometry. To see block textures in TrenchBroom, point it at a folder containing the matching Minecraft assets.

## Dependencies

- **`fastnbt` & `flate2`**: Handles raw and gzip-compressed NBT schematic formats.
- **`rustmatica`**: Reads `.litematic` Fabric schema designs.
- **`mcdata`**: Provides block ID context and parsing mapping.
- **`clap`**: Drives the command-line argument interface.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
