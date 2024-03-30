use std::fs::File;
use std::io::{self, Read};
use binread::BinReaderExt;
use binread::{BinRead, io::Cursor};

#[derive(BinRead,Debug)]
#[br(magic = b"MThd", big)]
struct MidiHeader {
    length: u32,
    format: u16,
    track_count: u16,
    delta: u16,
}

fn main() {
    let mut reader = Cursor::new(File::open("output.mid").unwrap().bytes().collect::<io::Result<Vec<u8>>>().unwrap());
    let header: MidiHeader = reader.read_be().unwrap();
    println!("{:?}", header);
}