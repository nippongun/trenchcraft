use crate::block_shapes::{self, BlockShape};
use crate::parser::{Block, VoxelMap};

pub struct Brush {
    pub min: (i32, i32, i32),
    pub max: (i32, i32, i32),
    pub texture: String,
}

// Map Minecraft block coords to Quake units: X→X, Z→Y, Y→Z, scaled by 32.
fn to_quake(bx: i32, bz: i32, by: i32) -> (i32, i32, i32) {
    (bx * 32, bz * 32, by * 32)
}

fn is_full_match(
    bx: i32, by: i32, bz: i32,
    texture: &str,
    visited: &[bool],
    blocks: &[Option<Block>],
    w: i32, l: i32,
) -> bool {
    let i = (by * (w * l) + bz * w + bx) as usize;
    if visited[i] { return false; }
    if let Some(b) = &blocks[i] {
        if b.name != texture { return false; }
        matches!(block_shapes::get_shape(b), BlockShape::Full)
    } else {
        false
    }
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

                let block = match &map.blocks[idx] {
                    Some(b) => b.clone(),
                    None => continue,
                };

                let texture = block.name.clone();
                let shape = block_shapes::get_shape(&block);

                match shape {
                    BlockShape::Full => {
                        // 1. Expand in X
                        let mut ex = x + 1;
                        while ex < w && is_full_match(ex, y, z, &texture, &visited, &map.blocks, w, l) {
                            ex += 1;
                        }

                        // 2. Expand in Z
                        let mut ez = z + 1;
                        'z_exp: while ez < l {
                            for tx in x..ex {
                                if !is_full_match(tx, y, ez, &texture, &visited, &map.blocks, w, l) {
                                    break 'z_exp;
                                }
                            }
                            ez += 1;
                        }

                        // 3. Expand in Y
                        let mut ey = y + 1;
                        'y_exp: while ey < h {
                            for tz in z..ez {
                                for tx in x..ex {
                                    if !is_full_match(tx, ey, tz, &texture, &visited, &map.blocks, w, l) {
                                        break 'y_exp;
                                    }
                                }
                            }
                            ey += 1;
                        }

                        for vy in y..ey {
                            for vz in z..ez {
                                for vx in x..ex {
                                    visited[get_idx(vx, vy, vz)] = true;
                                }
                            }
                        }

                        let (qx1, qy1, qz1) = to_quake(x, z, y);
                        let (qx2, qy2, qz2) = to_quake(ex, ez, ey);
                        brushes.push(Brush { min: (qx1, qy1, qz1), max: (qx2, qy2, qz2), texture });
                    }

                    BlockShape::SlabBottom => {
                        visited[idx] = true;
                        let (qx1, qy1, qz1) = to_quake(x, z, y);
                        let (qx2, qy2, _) = to_quake(x + 1, z + 1, y + 1);
                        brushes.push(Brush { min: (qx1, qy1, qz1), max: (qx2, qy2, qz1 + 16), texture });
                    }

                    BlockShape::SlabTop => {
                        visited[idx] = true;
                        let (qx1, qy1, qz1) = to_quake(x, z, y);
                        let (qx2, qy2, qz2) = to_quake(x + 1, z + 1, y + 1);
                        brushes.push(Brush { min: (qx1, qy1, qz1 + 16), max: (qx2, qy2, qz2), texture });
                    }

                    BlockShape::ThinPanelNS => {
                        visited[idx] = true;
                        let (qx1, qy1, qz1) = to_quake(x, z, y);
                        let (_, qy2, qz2) = to_quake(x + 1, z + 1, y + 1);
                        // Panel runs N-S (Quake Y), thin in Quake X
                        brushes.push(Brush {
                            min: (qx1 + 14, qy1, qz1),
                            max: (qx1 + 18, qy2, qz2),
                            texture,
                        });
                    }

                    BlockShape::ThinPanelEW => {
                        visited[idx] = true;
                        let (qx1, qy1, qz1) = to_quake(x, z, y);
                        let (qx2, _, qz2) = to_quake(x + 1, z + 1, y + 1);
                        // Panel runs E-W (Quake X), thin in Quake Y
                        brushes.push(Brush {
                            min: (qx1, qy1 + 14, qz1),
                            max: (qx2, qy1 + 18, qz2),
                            texture,
                        });
                    }

                    BlockShape::ThinCross => {
                        visited[idx] = true;
                        let (qx1, qy1, qz1) = to_quake(x, z, y);
                        let (qx2, qy2, qz2) = to_quake(x + 1, z + 1, y + 1);
                        brushes.push(Brush {
                            min: (qx1 + 14, qy1, qz1),
                            max: (qx1 + 18, qy2, qz2),
                            texture: texture.clone(),
                        });
                        brushes.push(Brush {
                            min: (qx1, qy1 + 14, qz1),
                            max: (qx2, qy1 + 18, qz2),
                            texture,
                        });
                    }
                }
            }
        }
    }

    println!("Optimized into {} brushes.", brushes.len());
    brushes
}
