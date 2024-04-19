use std::fs::File;
use std::io::{Read, Seek};
use binread::{BinRead, BinReaderExt, BinResult, ReadOptions, io::Cursor};

#[derive(BinRead, Debug)]
#[br(big)]
pub struct MidiFile {
    pub header: MidiHeader,

    #[br(count = header.track_count as usize)]
    pub tracks: Vec<MidiTrack>,
}

#[derive(BinRead, Debug)]
#[br(magic = b"MThd", big)]
pub struct MidiHeader {
    pub length: u32,
    pub format: u16,
    pub track_count: u16,
    pub division: u16,
}

#[derive(BinRead, Debug)]
#[br(magic = b"MTrk", big)]
pub struct MidiTrack {
    pub length: u32,

    #[br(parse_with = read_track_events)]
    pub events: Vec<TrackEvent>,
}


fn read_track_events<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _: ()) -> BinResult<Vec<TrackEvent>> {
    let mut events = Vec::new();
    loop {
        let delta_time = read_bytes_until_msb_zero(reader).unwrap();
        print!("Delta time: {:?}\n", delta_time);
        let event_type: u8 = reader.read_be().unwrap();
        print!("Event type: {:02X}\n", event_type);
        let event = match event_type {
            0x80 => {
                let channel = reader.read_be().unwrap();
                let note = reader.read_be().unwrap();
                let velocity = reader.read_be().unwrap();
                Event::_NoteOff { channel, note, velocity }
            }
            0x90 => {
                let channel = reader.read_be().unwrap();
                let note = reader.read_be().unwrap();
                let velocity = reader.read_be().unwrap();
                Event::_NoteOn { channel, note, velocity }
            }
            0xB0 => {
                let channel = reader.read_be().unwrap();
                let control = reader.read_be().unwrap();
                let value = reader.read_be().unwrap();
                Event::_ControlChange { channel, control, value }
            }
            0xC0 => {
                let channel = reader.read_be().unwrap();
                let program = reader.read_be().unwrap();
                Event::_ProgramChange { channel, program }
            }
            0xE0 => {
                let channel = reader.read_be().unwrap();
                let value = reader.read_be().unwrap();
                Event::_PitchBend { channel, value }
            }
            _ => {
                Event::_Unknown
            }
        };
        events.push(TrackEvent { delta_time: delta_time.clone(), event });
        if events.len() == 3 {
            break;
        }
    }
    Ok(events)
}

#[derive(Debug)]
pub struct TrackEvent {
    pub delta_time: Vec<u8>,
    pub event: Event,
}

fn read_bytes_until_msb_zero<R: std::io::Read>(reader: &mut R) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = [0; 1];
        reader.read_exact(&mut byte)?;
        bytes.push(byte[0]);
        if byte[0] & 0x80 == 0 {
            break;
        }
    }
    Ok(bytes)
}

#[derive(Debug)]
pub enum Event {
    _NoteOn { channel: u8, note: u8, velocity: u8 },
    _NoteOff { channel: u8, note: u8, velocity: u8 },
    _ControlChange { channel: u8, control: u8, value: u8 },
    _ProgramChange { channel: u8, program: u8 },
    _PitchBend { channel: u8, value: u16 },
    _Meta { delta_time: u32, event_type: u8, data: Vec<u8> },
    _Unknown,
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

#[cfg(test)]
mod tests {
    use crate::midi_parser::MidiParser;

    #[test]
    fn test_parse_midi_with_midly() {
        use std::fs;
        use midly::Smf;

        let mut parser = MidiParser::new("output.mid");
        let actual = parser.parse();

        let data = fs::read("output.mid").unwrap();
        let expected = Smf::parse(&data).unwrap();

        println!("{:#?}", expected);
    }
}