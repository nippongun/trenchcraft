use crate::parser::VoxelMap;

pub fn filter_blocks(mut map: VoxelMap) -> VoxelMap {
    println!("Filtering air blocks and applying textures...");
    for block_opt in &mut map.blocks {
        let is_empty = match block_opt {
            Some(block) => block.name.contains("air") || block.name == "minecraft:structure_void",
            None => true,
        };

        if is_empty {
            *block_opt = None;
        } else if let Some(block) = block_opt {
            let cleaned = block.name.replace("minecraft:", "minecraft/");
            block.name = format!("textures/{}", cleaned);
        }
    }
    map
}
