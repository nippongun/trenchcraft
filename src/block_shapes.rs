use crate::parser::Block;

pub enum BlockShape {
    Full,
    SlabBottom,
    SlabTop,
    ThinPanelNS,
    ThinPanelEW,
    ThinCross,
}

pub fn get_shape(block: &Block) -> BlockShape {
    let name = &block.name;

    if name.ends_with("_slab") {
        let half = block.properties.get("half").map(|s| s.as_str()).unwrap_or("bottom");
        return if half == "top" { BlockShape::SlabTop } else { BlockShape::SlabBottom };
    }

    let is_thin = name.ends_with("_fence")
        || name.contains("glass_pane")
        || name.contains("iron_bars");

    if is_thin {
        let prop_true = |key: &str| -> bool {
            block.properties.get(key).map(|v| v != "false" && v != "none").unwrap_or(false)
        };
        let ns = prop_true("north") || prop_true("south");
        let ew = prop_true("east") || prop_true("west");
        return match (ns, ew) {
            (true, false) => BlockShape::ThinPanelNS,
            (false, true) => BlockShape::ThinPanelEW,
            _ => BlockShape::ThinCross,
        };
    }

    BlockShape::Full
}
