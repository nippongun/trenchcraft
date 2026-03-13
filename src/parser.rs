use fastnbt::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Block {
    pub name: String,
    pub properties: HashMap<String, String>,
}

pub struct VoxelMap {
    pub width: i32,
    pub height: i32,
    pub length: i32,
    pub blocks: Vec<Option<Block>>,
}

pub fn parse_nbt(data: &Value) -> Option<VoxelMap> {
    if let Value::Compound(map) = data {
        let get_int = |key: &str| -> Option<i32> {
            match map.get(key) {
                Some(Value::Short(s)) => Some(*s as i32),
                Some(Value::Int(i)) => Some(*i),
                _ => None,
            }
        };

        let width = get_int("Width")?;
        let height = get_int("Height")?;
        let length = get_int("Length")?;
        
        let mut palette_map: HashMap<i32, Block> = HashMap::new();
        if let Some(Value::Compound(palette)) = map.get("Palette") {
            for (name, val) in palette {
                if let Value::Int(id) = val {
                    let block_name = name.split('[').next().unwrap_or(name).to_string();
                    let props_str = name.split('[').nth(1).unwrap_or("").trim_end_matches(']');
                    let mut properties = HashMap::new();
                    for pair in props_str.split(',') {
                        let mut kv = pair.splitn(2, '=');
                        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                            if !k.is_empty() {
                                properties.insert(k.to_string(), v.to_string());
                            }
                        }
                    }
                    palette_map.insert(*id, Block { name: block_name, properties });
                }
            }
        }
        
        if let Some(Value::ByteArray(block_data)) = map.get("BlockData") {
            let mut blocks = vec![None; (width * height * length) as usize];
            let mut cursor = 0;
            
            for ptr in 0..blocks.len() {
                if cursor >= block_data.len() { break; }
                
                // Read VarInt
                let mut val = 0;
                let mut shift = 0;
                loop {
                    if cursor >= block_data.len() { break; }
                    let b = block_data[cursor] as u8;
                    cursor += 1;
                    val |= ((b & 0x7F) as i32) << shift;
                    if (b & 0x80) == 0 { break; }
                    shift += 7;
                }
                
                blocks[ptr] = palette_map.get(&val).cloned();
            }
            
            println!("Successfully parsed {} blocks.", blocks.len());
            return Some(VoxelMap {
                width,
                height,
                length,
                blocks,
            });
        }
    }
    None
}

impl VoxelMap {
    pub fn from_litematic(schem: &rustmatica::Litematic) -> Option<VoxelMap> {
        if schem.regions.is_empty() { return None; }
        
        let region = &schem.regions[0]; // Take the first region for now
        let size = &region.size;
        let width = size.x;
        let height = size.y;
        let length = size.z;
        
        let mut blocks = vec![None; (width * height * length) as usize];
        
        for y in 0..height {
            for z in 0..length {
                for x in 0..width {
                    let mc_block = region.get_block(mcdata::util::BlockPos::new(x, y, z));
                    let block_name = mc_block.name.to_string();
                    
                    let idx = (y * (width * length) + z * width + x) as usize;
                    blocks[idx] = Some(Block { name: block_name, properties: HashMap::new() });
                }
            }
        }
        
        println!("Successfully extracted {} blocks from Litematic.", blocks.len());
        Some(VoxelMap {
            width,
            height,
            length,
            blocks,
        })
    }
}
