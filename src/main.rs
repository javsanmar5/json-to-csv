use std::fs::File;
use std::io::{Read, Write};

static INPUT_PATH: &str = "data/data.json";
static OUTPUT_PATH: &str = "data/output.csv";


fn main() -> std::io::Result<()> {
    let mut input_file = File::open(INPUT_PATH)?;
    let mut output_file = File::create(OUTPUT_PATH)?;
    let mut data = vec![];
    
    input_file.read_to_end(&mut data)?;

    if let Ok(content) = String::from_utf8(data.clone()) {
        println!("{}", content);
    } else {
        eprintln!("Error: Invalid UTF-8 sequence in input file");
    }
    
    output_file.write_all(&data)?;

    Ok(())
}