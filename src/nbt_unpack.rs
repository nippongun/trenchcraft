use fastnbt::Value;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Nbt(fastnbt::error::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<fastnbt::error::Error> for Error {
    fn from(err: fastnbt::error::Error) -> Self {
        Error::Nbt(err)
    }
}

pub fn load_schematic(path: &Path) -> Result<Value, Error> {
    let file = File::open(path)?;
    // Most .schematic files are gzipped.
    let mut decoder = GzDecoder::new(file);
    let mut bytes = Vec::new();

    // Sometimes they are NOT gzipped. We should handle both cases eventually,
    // but typically we can try reading as gzip first.
    if let Err(e) = decoder.read_to_end(&mut bytes) {
        // Fallback: try reading the file without gzip
        println!("Not gzipped or invalid gzip, attempting raw read: {}", e);
        let mut raw_file = File::open(path)?;
        bytes.clear();
        raw_file.read_to_end(&mut bytes)?;
    }

    let nbt_res: Value = fastnbt::from_bytes(&bytes)?;
    Ok(nbt_res)
}
