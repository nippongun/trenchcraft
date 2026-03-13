use clap::ValueEnum;

use crate::block_shapes::{self, BlockShape};
use crate::parser::{Block, VoxelMap};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Default)]
pub enum GreedyLevel {
    /// No merging — one brush per block
    None,
    /// Merge full blocks only
    #[value(name = "full-only")]
    FullOnly,
    /// Merge all compatible shapes (default)
    #[default]
    All,
}

pub struct Brush {
    pub min: (i32, i32, i32),
    pub max: (i32, i32, i32),
    pub texture: String,
}

#[derive(Copy, Clone)]
enum Axis { X, Y, Z }

const AXES_XZY: &[Axis] = &[Axis::X, Axis::Z, Axis::Y]; // full blocks
const AXES_XZ:  &[Axis] = &[Axis::X, Axis::Z];           // slabs
const AXES_ZY:  &[Axis] = &[Axis::Z, Axis::Y];           // NS panels
const AXES_XY:  &[Axis] = &[Axis::X, Axis::Y];           // EW panels

// Map Minecraft block coords to Quake units: X→X, Z→Y, Y→Z, scaled by 32.
fn to_quake(bx: i32, bz: i32, by: i32) -> (i32, i32, i32) {
    (bx * 32, bz * 32, by * 32)
}

fn is_shape_match(
    bx: i32, by: i32, bz: i32,
    texture: &str,
    shape: BlockShape,
    visited: &[bool],
    blocks: &[Option<Block>],
    w: i32, l: i32,
) -> bool {
    let i = (by * (w * l) + bz * w + bx) as usize;
    if visited[i] { return false; }
    if let Some(b) = &blocks[i] {
        b.name == texture && block_shapes::get_shape(b) == shape
    } else {
        false
    }
}

// Greedy-expand from (x, y, z) along the given axes. Returns exclusive block-coord extents.
fn greedy_expand(
    x: i32, y: i32, z: i32,
    texture: &str,
    shape: BlockShape,
    axes: &[Axis],
    map: &VoxelMap,
    visited: &[bool],
) -> (i32, i32, i32) {
    let w = map.width;
    let h = map.height;
    let l = map.length;

    let mut ex = x + 1;
    let mut ey = y + 1;
    let mut ez = z + 1;

    for &axis in axes {
        match axis {
            Axis::X => {
                'x: loop {
                    if ex >= w { break; }
                    for cy in y..ey {
                        for cz in z..ez {
                            if !is_shape_match(ex, cy, cz, texture, shape, visited, &map.blocks, w, l) {
                                break 'x;
                            }
                        }
                    }
                    ex += 1;
                }
            }
            Axis::Y => {
                'y: loop {
                    if ey >= h { break; }
                    for cx in x..ex {
                        for cz in z..ez {
                            if !is_shape_match(cx, ey, cz, texture, shape, visited, &map.blocks, w, l) {
                                break 'y;
                            }
                        }
                    }
                    ey += 1;
                }
            }
            Axis::Z => {
                'z: loop {
                    if ez >= l { break; }
                    for cx in x..ex {
                        for cy in y..ey {
                            if !is_shape_match(cx, cy, ez, texture, shape, visited, &map.blocks, w, l) {
                                break 'z;
                            }
                        }
                    }
                    ez += 1;
                }
            }
        }
    }

    (ex, ey, ez)
}

fn single_block_brush(x: i32, y: i32, z: i32, shape: BlockShape, texture: String) -> Vec<Brush> {
    let (qx1, qy1, qz1) = to_quake(x, z, y);
    let (qx2, qy2, qz2) = to_quake(x + 1, z + 1, y + 1);
    match shape {
        BlockShape::SlabBottom => vec![Brush { min: (qx1, qy1, qz1),      max: (qx2, qy2, qz1 + 16), texture }],
        BlockShape::SlabTop    => vec![Brush { min: (qx1, qy1, qz1 + 16), max: (qx2, qy2, qz2),      texture }],
        BlockShape::ThinPanelNS => vec![Brush { min: (qx1 + 14, qy1, qz1), max: (qx1 + 18, qy2, qz2), texture }],
        BlockShape::ThinPanelEW => vec![Brush { min: (qx1, qy1 + 14, qz1), max: (qx2, qy1 + 18, qz2), texture }],
        BlockShape::ThinCross => vec![
            Brush { min: (qx1 + 14, qy1, qz1), max: (qx1 + 18, qy2, qz2), texture: texture.clone() },
            Brush { min: (qx1, qy1 + 14, qz1), max: (qx2, qy1 + 18, qz2), texture },
        ],
        BlockShape::Full => vec![Brush { min: (qx1, qy1, qz1), max: (qx2, qy2, qz2), texture }],
    }
}

pub fn optimize_mesh(map: &VoxelMap, level: GreedyLevel) -> Vec<Brush> {
    println!("Optimizing geometry... (greedy: {:?})", level);
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

                let axes: Option<&[Axis]> = match (level, shape) {
                    (GreedyLevel::None, _) => None,
                    (GreedyLevel::FullOnly, BlockShape::Full) => Some(AXES_XZY),
                    (GreedyLevel::FullOnly, _) => None,
                    (GreedyLevel::All, BlockShape::Full) => Some(AXES_XZY),
                    (GreedyLevel::All, BlockShape::SlabBottom | BlockShape::SlabTop) => Some(AXES_XZ),
                    (GreedyLevel::All, BlockShape::ThinPanelNS) => Some(AXES_ZY),
                    (GreedyLevel::All, BlockShape::ThinPanelEW) => Some(AXES_XY),
                    (GreedyLevel::All, BlockShape::ThinCross) => None,
                };

                if let Some(axes) = axes {
                    let (ex, ey, ez) = greedy_expand(x, y, z, &texture, shape, axes, map, &visited);

                    for vy in y..ey {
                        for vz in z..ez {
                            for vx in x..ex {
                                visited[get_idx(vx, vy, vz)] = true;
                            }
                        }
                    }

                    let (qx1, qy1, qz1) = to_quake(x, z, y);
                    let (qx2, qy2, qz2) = to_quake(ex, ez, ey);

                    let brush = match shape {
                        BlockShape::SlabBottom  => Brush { min: (qx1, qy1, qz1),      max: (qx2, qy2, qz1 + 16), texture },
                        BlockShape::SlabTop     => Brush { min: (qx1, qy1, qz1 + 16), max: (qx2, qy2, qz2),      texture },
                        BlockShape::ThinPanelNS => Brush { min: (qx1 + 14, qy1, qz1), max: (qx1 + 18, qy2, qz2), texture },
                        BlockShape::ThinPanelEW => Brush { min: (qx1, qy1 + 14, qz1), max: (qx2, qy1 + 18, qz2), texture },
                        _ =>                       Brush { min: (qx1, qy1, qz1),       max: (qx2, qy2, qz2),      texture },
                    };
                    brushes.push(brush);
                } else {
                    visited[idx] = true;
                    brushes.extend(single_block_brush(x, y, z, shape, texture));
                }
            }
        }
    }

    println!("Optimized into {} brushes.", brushes.len());
    brushes
}
