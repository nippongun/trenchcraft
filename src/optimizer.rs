use crate::parser::VoxelMap;

pub struct Brush {
    pub min: (i32, i32, i32),
    pub max: (i32, i32, i32),
    pub texture: String,
}

pub fn optimize_mesh(map: &VoxelMap) -> Vec<Brush> {
    println!("Optimizing geometry...");
    let mut brushes = Vec::new();
    let mut visited = vec![false; map.blocks.len()];
    
    let w = map.width;
    let h = map.height;
    let l = map.length;
    
    let get_idx = |x, y, z| (y * (w * l) + z * w + x) as usize;
    
    for y in 0..h {
        for z in 0..l {
            for x in 0..w {
                let idx = get_idx(x, y, z);
                if visited[idx] { continue; }
                
                if let Some(block) = &map.blocks[idx] {
                    let texture = block.name.clone();
                    
                    // 1. Expand in X
                    let mut ex = x + 1;
                    while ex < w {
                        let i = get_idx(ex, y, z);
                        if visited[i] { break; }
                        if let Some(b) = &map.blocks[i] {
                            if b.name != texture { break; }
                        } else {
                            break; // empty block
                        }
                        ex += 1;
                    }
                    
                    // 2. Expand in Z
                    let mut ez = z + 1;
                    'z_expansion: while ez < l {
                        for test_x in x..ex {
                            let i = get_idx(test_x, y, ez);
                            if visited[i] { break 'z_expansion; }
                            if let Some(b) = &map.blocks[i] {
                                if b.name != texture { break 'z_expansion; }
                            } else {
                                break 'z_expansion; // empty block
                            }
                        }
                        ez += 1;
                    }
                    
                    // 3. Expand in Y
                    let mut ey = y + 1;
                    'y_expansion: while ey < h {
                        for test_z in z..ez {
                            for test_x in x..ex {
                                let i = get_idx(test_x, ey, test_z);
                                if visited[i] { break 'y_expansion; }
                                if let Some(b) = &map.blocks[i] {
                                    if b.name != texture { break 'y_expansion; }
                                } else {
                                    break 'y_expansion; // empty block
                                }
                            }
                        }
                        ey += 1;
                    }
                    
                    // Mark as visited
                    for vy in y..ey {
                        for vz in z..ez {
                            for vx in x..ex {
                                visited[get_idx(vx, vy, vz)] = true;
                            }
                        }
                    }
                    
                    brushes.push(Brush {
                        min: (x, y, z),
                        max: (ex, ey, ez),
                        texture,
                    });
                }
            }
        }
    }
    
    println!("Optimized into {} brushes.", brushes.len());
    brushes
}
