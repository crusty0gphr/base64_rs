pub trait Lookup {
    // return the character for the given index - from base64 lookup table
    fn get_b64_char(&self, index: i8) -> Option<char>;
    // return the index of the give char/symbol - from base64 lookup table
    fn get_b64_index(&self, character: char) -> Option<i8>;
    fn get_padding(&self) -> char;
}

const LOOKUP_OFFSET_UPPER: i8 = 65;
const LOOKUP_OFFSET_LOWER: i8 = 71;
const LOOKUP_OFFSET_NUM: i8 = 4;

const PADDING_CHAR: char = '=';

pub struct LookupTable;

impl Lookup for LookupTable {
    fn get_b64_char(&self, i: i8) -> Option<char> {
        let ascii_index = match i {
            0..=25 => i + LOOKUP_OFFSET_UPPER,  // A-Z
            26..=51 => i + LOOKUP_OFFSET_LOWER, // a-z
            52..=61 => i - LOOKUP_OFFSET_NUM,   // 0-9
            62 => 43,                           // +
            63 => 47,                           // /

            _ => return None,
        } as u8;

        Some(ascii_index as char)
    }

    fn get_b64_index(&self, ch: char) -> Option<i8> {
        let character = ch as i8;

        let base64_index = match character {
            65..=90 => character - LOOKUP_OFFSET_UPPER,  // A-Z
            97..=122 => character - LOOKUP_OFFSET_LOWER, // a-z
            48..=57 => character + LOOKUP_OFFSET_NUM,    // 0-9
            43 => 62,                                    // +
            47 => 63,                                    // /

            _ => return None,
        };

        Some(base64_index)
    }

    fn get_padding(&self) -> char {
        PADDING_CHAR
    }
}
