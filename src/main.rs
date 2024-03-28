use std::fs::File;
use std::io::{self, Read};
use base64::{engine::general_purpose::STANDARD, Engine as _};

fn main() -> io::Result<()> {
    let mut file = File::open("output.mid")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let encoded = STANDARD.encode(&buffer);
    println!("{}", encoded);

    Ok(())
}