mod midi_parser;
use crate::midi_parser::{MidiParser, MidiFile};

fn main() {
    let file: MidiFile = MidiParser::new("output.mid").parse();
    println!("{:?}", file);
}