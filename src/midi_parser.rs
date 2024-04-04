use std::fs::File;
use std::io::{self, Read};
use binread::BinReaderExt;
use binread::{BinRead, io::Cursor};

#[derive(BinRead,Debug)]
#[br(big)]
pub struct MidiFile {
    pub header: MidiHeader,

    #[br(count = header.track_count as usize)]
    pub tracks: Vec<MidiTrack>,
}

#[derive(BinRead,Debug)]
#[br(magic = b"MThd", big)]
struct MidiHeader {
    length: u32,
    format: u16,
    track_count: u16,
    division: u16,
}

#[derive(BinRead,Debug)]
#[br(magic = b"MTrk", big)]
struct MidiTrack {
    length: u32,

    #[br(count = length as usize)]
    events: Vec<u8>,
}

pub struct MidiParser {
    data: Cursor<Vec<u8>>,
}

impl MidiParser {
    pub fn new(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("Failed to read file");

        MidiParser {
            data: Cursor::new(data),
        }
    }

    pub fn parse(&mut self) -> MidiFile {
        self.data.read_be().unwrap()
    }
}