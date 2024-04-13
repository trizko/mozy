mod midi_parser;
use crate::midi_parser::{MidiParser, MidiFile};

fn main() {
    let file_path = "output.mid";
    let midi_parser: MidiParser = MidiParser::new(file_path);
    let midi_file: MidiFile = midi_parser.parse();
    println!("{:#?}", midi_file);
}