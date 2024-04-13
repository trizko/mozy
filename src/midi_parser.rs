use binread::{BinRead, BinReaderExt};
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

#[derive(BinRead, Debug)]
#[br(magic = b"MThd")]
struct MidiHeader {
    length: u32,
    format: u16,
    track_count: u16,
    time_division: u16,
}

#[derive(BinRead, Debug)]
#[br(magic = b"MTrk")]
struct MidiTrack {
    length: u32,
    #[br(count = length)]
    events: Vec<u8>,
}

#[derive(Debug)]
pub struct MidiFile {
    header: MidiHeader,
    tracks: Vec<MidiTrack>,
}

pub struct MidiParser {
    file_path: String,
}

impl MidiParser {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn parse(&self) -> MidiFile {
        let mut file = File::open(Path::new(&self.file_path)).unwrap();

        let header: MidiHeader = file.read_be().unwrap();
        let mut tracks = Vec::new();

        for _ in 0..header.track_count {
            let track: MidiTrack = file.read_be().unwrap();
            tracks.push(track);
        }

        let midi_file = MidiFile { header, tracks };
        self.parse_midi_events(&midi_file);

        midi_file
    }

    fn parse_midi_events(&self, midi_file: &MidiFile) {
        for (i, track) in midi_file.tracks.iter().enumerate() {
            println!("Track {}: length = {}", i + 1, track.length);
            self.parse_track_events(&track.events);
        }
    }

    fn parse_track_events(&self, events: &[u8]) {
        let mut cursor = Cursor::new(events);

        while cursor.position() < events.len() as u64 {
            let delta_time = self.read_variable_length_quantity(&mut cursor);
            let event_type = cursor.read_be::<u8>().unwrap();

            match event_type {
                0x80..=0x8F => self.parse_note_off_event(&mut cursor),
                0x90..=0x9F => self.parse_note_on_event(&mut cursor),
                0xA0..=0xAF => self.parse_polyphonic_key_pressure_event(&mut cursor),
                0xB0..=0xBF => self.parse_control_change_event(&mut cursor),
                0xC0..=0xCF => self.parse_program_change_event(&mut cursor),
                0xD0..=0xDF => self.parse_channel_pressure_event(&mut cursor),
                0xE0..=0xEF => self.parse_pitch_bend_change_event(&mut cursor),
                0xF0 | 0xF7 => self.parse_system_exclusive_event(&mut cursor),
                0xFF => self.parse_meta_event(&mut cursor),
                _ => panic!("Unknown event type: {:#04X}", event_type),
            }
        }
    }

    fn parse_note_off_event(&self, cursor: &mut Cursor<&[u8]>) {
        let note_number = cursor.read_be::<u8>().unwrap();
        let velocity = cursor.read_be::<u8>().unwrap();
        println!("Note Off: note = {}, velocity = {}", note_number, velocity);
    }

    fn parse_note_on_event(&self, cursor: &mut Cursor<&[u8]>) {
        let note_number = cursor.read_be::<u8>().unwrap();
        let velocity = cursor.read_be::<u8>().unwrap();
        println!("Note On: note = {}, velocity = {}", note_number, velocity);
    }

    fn parse_polyphonic_key_pressure_event(&self, cursor: &mut Cursor<&[u8]>) {
        let note_number = cursor.read_be::<u8>().unwrap();
        let pressure = cursor.read_be::<u8>().unwrap();
        println!("Polyphonic Key Pressure: note = {}, pressure = {}", note_number, pressure);
    }

    fn parse_control_change_event(&self, cursor: &mut Cursor<&[u8]>) {
        let controller_number = cursor.read_be::<u8>().unwrap();
        let controller_value = cursor.read_be::<u8>().unwrap();
        println!("Control Change: controller = {}, value = {}", controller_number, controller_value);
    }

    fn parse_program_change_event(&self, cursor: &mut Cursor<&[u8]>) {
        let program_number = cursor.read_be::<u8>().unwrap();
        println!("Program Change: program = {}", program_number);
    }

    fn parse_channel_pressure_event(&self, cursor: &mut Cursor<&[u8]>) {
        let pressure = cursor.read_be::<u8>().unwrap();
        println!("Channel Pressure: pressure = {}", pressure);
    }

    fn parse_pitch_bend_change_event(&self, cursor: &mut Cursor<&[u8]>) {
        let lsb = cursor.read_be::<u8>().unwrap();
        let msb = cursor.read_be::<u8>().unwrap();
        let pitch_bend = ((msb as u16) << 7) | (lsb as u16);
        println!("Pitch Bend Change: pitch bend = {}", pitch_bend);
    }

    fn parse_system_exclusive_event(&self, cursor: &mut Cursor<&[u8]>) {
        let length = self.read_variable_length_quantity(cursor);
        let data = self.read_bytes(cursor, length as usize);
        println!("System Exclusive: length = {}, data = {:?}", length, data);
    }

    fn parse_meta_event(&self, cursor: &mut Cursor<&[u8]>) {
        let meta_type = cursor.read_be::<u8>().unwrap();
        let length = self.read_variable_length_quantity(cursor);
        let data = self.read_bytes(cursor, length as usize);
        println!("Meta Event: type = {:#04X}, length = {}, data = {:?}", meta_type, length, data);
    }

    fn read_variable_length_quantity(&self, cursor: &mut Cursor<&[u8]>) -> u32 {
        let mut value = 0;
        let mut bytes_read = 0;

        loop {
            let byte = cursor.read_be::<u8>().unwrap();
            value |= ((byte & 0x7F) as u32) << (7 * bytes_read);
            bytes_read += 1;

            if byte & 0x80 == 0 {
                break;
            }
        }

        value
    }

    fn read_bytes(&self, cursor: &mut Cursor<&[u8]>, length: usize) -> Vec<u8> {
        let mut buffer = vec![0; length];
        cursor.read(&mut buffer).unwrap();
        buffer
    }
}
