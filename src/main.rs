use clap::Parser;
use std::path::PathBuf;

use optimizer::GreedyLevel;

mod nbt_unpack;
mod parser;
mod filter;
mod block_shapes;
mod optimizer;
mod exporter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input Minecraft .schematic, .schem, or .litematic file
    input: PathBuf,

    /// Path to the output Trenchbroom .map file
    output: PathBuf,

    /// Greedy meshing level: none, full-only, or all (default)
    #[arg(long, value_enum, default_value_t = GreedyLevel::All)]
    greedy: GreedyLevel,
}

fn main() {
    let args = Args::parse();
    println!("Converting {} to {}", args.input.display(), args.output.display());

    let ext = args.input.extension().and_then(|e| e.to_str()).unwrap_or("");

    let voxel_map = if ext == "litematic" {
        println!("Loading Litematic file...");
        let result: Result<rustmatica::Litematic, _> = rustmatica::Litematic::read_file(&args.input);
        match result {
            Ok(schem) => parser::VoxelMap::from_litematic(&schem),
            Err(e) => {
                eprintln!("Failed to load litematic {}: {:?}", args.input.display(), e);
                None
            }
        }
    } else {
        println!("Loading NBT/Schematic file...");
        match nbt_unpack::load_schematic(&args.input) {
            Ok(data) => {
                println!("Successfully unpacked the schematic.");
                parser::parse_nbt(&data)
            }
            Err(e) => {
                eprintln!("Failed to load {}: {:?}", args.input.display(), e);
                None
            }
        }
    };

    if let Some(map) = voxel_map {
        let filtered = filter::filter_blocks(map);
        let brushes = optimizer::optimize_mesh(&filtered, args.greedy);
        exporter::export_map(&brushes, &args.output);
    } else {
        eprintln!("Failed to parse the schematic into a 3D block array.");
    }
}
