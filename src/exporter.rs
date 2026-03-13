use crate::optimizer::Brush;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn export_map(brushes: &[Brush], output: &Path) {
    println!("Exporting {} brushes to {}...", brushes.len(), output.display());
    
    let mut file = match File::create(output) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create output map file: {:?}", e);
            return;
        }
    };

    writeln!(file, "// Game: Generic").unwrap();
    writeln!(file, "// Format: Valve").unwrap();
    writeln!(file, "// entity 0").unwrap();
    writeln!(file, "{{").unwrap();
    writeln!(file, "\"classname\" \"worldspawn\"").unwrap();
    writeln!(file, "\"mapversion\" \"220\"").unwrap();

    for brush in brushes {
        let tex = &brush.texture;

        let x1 = brush.min.0;
        let y1 = brush.min.1;
        let z1 = brush.min.2;
        let x2 = brush.max.0;
        let y2 = brush.max.1;
        let z2 = brush.max.2;

        writeln!(file, "// Brush").unwrap();
        writeln!(file, "{{").unwrap();
        
        // 6 Planes (Outward facing, CW)
        // Quake MAP plane definition: points must be defined clockwise when looking AT the face from the outside
        // TrenchBroom calculates the normal as (p0 - p1) x (p2 - p1). CCW makes normals point inward!
        
        // Top (+Z)
        writeln!(file, "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 -1 0 0 ] 0 1 1", x2, y2, z2,  x2, y1, z2,  x1, y1, z2, tex).unwrap();
        // Bottom (-Z)
        writeln!(file, "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 -1 0 0 ] 0 1 1", x2, y1, z1,  x2, y2, z1,  x1, y2, z1, tex).unwrap();
        // Front (+X)
        writeln!(file, "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 0 1 0 0 ] [ 0 0 -1 0 ] 0 1 1", x2, y2, z2,  x2, y2, z1,  x2, y1, z1, tex).unwrap();
        // Back (-X)
        writeln!(file, "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 0 1 0 0 ] [ 0 0 -1 0 ] 0 1 1", x1, y1, z2,  x1, y1, z1,  x1, y2, z1, tex).unwrap();
        // Right (+Y)
        writeln!(file, "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 0 -1 0 ] 0 1 1", x2, y2, z2,  x1, y2, z2,  x1, y2, z1, tex).unwrap();
        // Left (-Y)
        writeln!(file, "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 0 -1 0 ] 0 1 1", x1, y1, z2,  x2, y1, z2,  x2, y1, z1, tex).unwrap();
        
        writeln!(file, "}}").unwrap();
    }
    
    writeln!(file, "}}").unwrap();
    println!("Export complete.");
}
