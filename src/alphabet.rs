pub trait Alphabet {
    // return the character for the given index
    fn get_char(&self, index: u8) -> Option<char>;
    // return the index of the give char/symbol
    fn get_index(&self, char: char) -> Option<u8>;
    fn get_padding(&self) -> char;
}

const PADDING_CHAR: char = '=';

pub struct Classic;

impl Alphabet for Classic {
    fn get_char(&self, index: u8) -> Option<char> {
        let index = index as i8;

        let ascii_index = match index {
            0..=25 => index + 65,           // A-Z
            26..=51 => index + 71,          // a-z
            52..=61 => index + (-4),        // 0-9
            62 => 43,                       // +
            63 => 47,                       // /

            _ => return None,
        } as u8;

        Some(ascii_index as char)
    }

    fn get_index(&self, char: char) -> Option<u8> {
        let character = char as i8;
        let base64_index = match character {
            65..=90 => character - 65,          // A-Z
            97..=122 => character - 71,         // a-z
            48..=57 => character - (-4),        // 0-9
            43 => 62,                           // +
            47 => 63,                           // /

            _ => return None,
        } as u8;

        Some(base64_index)
    }

    fn get_padding(&self) -> char {
        PADDING_CHAR
    }
}