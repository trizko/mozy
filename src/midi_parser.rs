use std::fs::File;
use std::io::{self, Read};
use binread::BinReaderExt;
use binread::{BinRead, io::Cursor};

struct MidiParser {
    data: Cursor<Vec<u8>>,
}

impl MidiParser {
    pub fn new(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("Failed to read file");

        Self {
            data: Cursor::new(data),
        }
    }
}