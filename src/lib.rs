use std::io;

mod decoder;
mod encoder;
mod lookup_table;

pub fn encode(input: String) -> String {
    encoder::encode(input)
}

pub fn decode(input: String) -> Result<String, io::Error> {
    decoder::decode(input)
}
