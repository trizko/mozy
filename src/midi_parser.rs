use std::fs::File;
use std::io::{Read, Seek};
use binread::BinReaderExt;
use binread::{BinRead, BinResult, ReadOptions, io::Cursor};

#[derive(BinRead)]
#[br(big)]
pub struct MidiFile {
    pub header: MidiHeader,

    #[br(count = header.track_count as usize)]
    pub tracks: Vec<MidiTrack>,
}

#[derive(BinRead)]
#[br(magic = b"MThd", big)]
pub struct MidiHeader {
    pub length: u32,
    pub format: u16,
    pub track_count: u16,
    pub division: u16,
}

#[derive(BinRead)]
#[br(magic = b"MTrk", big)]
pub struct MidiTrack {
    length: u32,

    #[br(parse_with = read_track_events)]
    events: Vec<TrackEvent>,
}

fn read_track_events<R: Read + Seek>(_reader: &mut R, _ro: &ReadOptions, _: ())
    -> BinResult<Vec<TrackEvent>>
{
    unimplemented!()
}

struct TrackEvent {
    delta_time: u32,
    event: Event,
}

enum Event {
    NoteOn { channel: u8, note: u8, velocity: u8 },
    NoteOff { channel: u8, note: u8, velocity: u8 },
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