use std::fs::File;
use std::io::{self, Read};
use binread::BinReaderExt;
use binread::{BinRead, io::Cursor};

struct MidiFile {
    pub header: MidiHeader,
    pub tracks: Vec<MidiTrack>,
}

struct MidiTrack {
    pub events: Vec<MidiEvent>,
}

enum MidiEvent {
    NoteOn {
        delta_time: u32,
        channel: u8,
        note_number: u8,
        velocity: u8,
    },
    NoteOff {
        delta_time: u32,
        channel: u8,
        note_number: u8,
        velocity: u8,
    },
    ProgramChange {
        delta_time: u32,
        channel: u8,
        program_number: u8,
    },
    ControlChange {
        delta_time: u32,
        channel: u8,
        controller: u8,
        value: u8,
    },
    MetaEvent {
        delta_time: u32,
        event_type: u8,
        data: Vec<u8>,
    },
}

#[derive(BinRead,Debug)]
#[br(magic = b"MThd", big)]
struct MidiHeader {
    length: u32,
    format: u16,
    track_count: u16,
    division: u16,
}

fn main() {
    let mut reader = Cursor::new(File::open("output.mid").unwrap().bytes().collect::<io::Result<Vec<u8>>>().unwrap());
    let header: MidiHeader = reader.read_be().unwrap();
    println!("{:?}", header);
    println!("length = {:?}\nformat = {:?}\ntracks = {:?}\ndelta = {:?}", header.length, header.format, header.track_count, header.delta);
}