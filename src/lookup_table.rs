// Basic implementation of the b64 lookup table with 64 chars
// using basic addition and subtraction
//
// Example: ascii_c(A) -> b64_c(A) =>
//          i := ascii_i(65) - b64_upper_i =>
//          65 - 65 =>
//          b65_i(0) == 'A'

pub trait Lookup {
    // return the character for the given index - from base64 lookup table
    fn get_b64_char(&self, index: i8) -> Option<char>;
    // return the index of the give char/symbol - from base64 lookup table
    fn get_b64_index(&self, character: char) -> Option<i8>;
    fn get_padding(&self) -> char;
}

// shift to b65 indices
const LOOKUP_OFFSET_UPPER: i8 = 65;
const LOOKUP_OFFSET_LOWER: i8 = 71;
const LOOKUP_OFFSET_NUM: i8 = 4;

const PADDING_CHAR: char = '=';

// zero value struct as a base type
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_get_b64_char() {
        // upper case chars
        {
            let i: i8 = 12;
            let exp: char = (i + LOOKUP_OFFSET_UPPER) as u8 as char;

            let res = LookupTable.get_b64_char(i);
            assert_eq!(res.unwrap(), exp);
        }

        // lower case chars
        {
            let i: i8 = 44;
            let exp: char = (i + LOOKUP_OFFSET_LOWER) as u8 as char;

            let res = LookupTable.get_b64_char(i);
            assert_eq!(res.unwrap(), exp);
        }

        // None
        {
            let i: i8 = 127;

            let res = LookupTable.get_b64_char(i);
            assert_eq!(res, None);
        }
    }

    #[test]
    fn lookup_get_b64_index() {
        // upper case chars
        {
            let ch: char = 'E';
            let exp: i8 = ch as i8 - LOOKUP_OFFSET_UPPER;

            let res = LookupTable.get_b64_index(ch);
            assert_eq!(res.unwrap(), exp);
        }

        // lower case chars
        {
            let ch: char = 'o';
            let exp: i8 = ch as i8 - LOOKUP_OFFSET_LOWER;

            let res = LookupTable.get_b64_index(ch);
            assert_eq!(res.unwrap(), exp);
        }

        // None
        {
            let ch: char = '~';

            let res = LookupTable.get_b64_index(ch);
            assert_eq!(res, None);
        }
    }
}
