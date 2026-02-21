# Trenchcraft

Trenchcraft is a robust, blazingly-fast command-line tool written in Rust for converting Minecraft schematic and structure files into standard Quake/Valve `.map` format for use with [TrenchBroom](https://trenchbroom.github.io/)!

## Features

- **Multi-Format Support**: Parses standard Minecraft `.schematic` and `.schem` files alongside modern Fabric `.litematic` schemas.
- **Smart Material Filtering**: Automatically culls empty `minecraft:air` blocks to prevent invisible garbage geometry from crashing the BSP compiler.
- **Greedy Meshing Optimizer**: Runs a sophisticated 3-dimensional face traversal algorithm. Trenchcraft consolidates thousands of single `1x1x1` blocks into large optimized geometric brushes, delivering **massive performance bumps** and keeping the Trenchbroom editor completely lag-free.
- **Valve 220 Map Configuration**: Outputs geometry using the modern Valve 220 Map format specification. This guarantees pixel-perfect UV mapping scaling in Trenchbroom.

## Usage

Trenchcraft takes two positional arguments: the input schematic file and the output map destination.

```bash
cargo run -- <input_schematic> <output_map>
```

### Example
```bash
cargo run -- The_White_House-from-abfielder.schem target/debug/The_White_House.map
```

\* *Note: Trenchcraft automatically assigns `textures/minecraft/<blockname>` paths to the output geometry. To see block textures in TrenchBroom, add a Texture WAD pack or game directory folder to TrenchBroom containing the matching Minecraft assets!*

## Dependencies

- **`fastnbt` & `flate2`**: Handles raw and gzip-compressed NBT schematic formats.
- **`rustmatica`**: Reads `.litematic` Fabric schema designs.
- **`mcdata`**: Provides block ID context and parsing mapping.
- **`clap`**: Drives the command-line argument interface.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
