use std::{fs::File, io::Read};

use lexer::Lexer;
use shared::types::uom::UcreError;

mod lexer;

pub fn run(file_name: &str) -> Result<(), UcreError> {
    let file = File::open(file_name).map_err(|e| UcreError::new(e.to_string()))?;
    let bytes = file.bytes().flatten().collect::<Vec<u8>>();
    let mut l = Lexer::new(&bytes);
    println!("{:?}", l.run()?);
    Ok(())
}
