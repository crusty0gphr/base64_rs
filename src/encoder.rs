use crate::lookup_table::{Lookup, LookupTable};

// shits chunks
fn shift_bytes_per_chunk(chunk: &[u8]) -> Vec<u8> {
    match chunk.len() {
        // 1byte chunk input
        1 => vec![&chunk[0] >> 2, (&chunk[0] & 0b00000011) << 4],

        // 2byte chunk input
        2 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            (&chunk[1] & 0b00001111) << 2,
        ],

        // 3byte chunk input
        3 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            (&chunk[1] & 0b00001111) << 2 | &chunk[2] >> 6,
            &chunk[2] & 0b00111111,
        ],

        // default
        _ => vec![],
    }
}

// encoder per chunk
// converts base64_id to base64_char
fn encode_chunks(chunk: Vec<u8>) -> Vec<char> {
    let mut out = vec![LookupTable.get_padding(); 4];

    for i in 0..chunk.len() {
        if let Some(ch) = LookupTable.get_b64_char(chunk[i]) {
            out[i] = ch
        }
    }

    out
}

pub fn encode(input: String) -> String {
    let bytes = input.into_bytes();
    let encoded = bytes
        // split byte array into 3byte chunks
        // Example: [99 114 117 115 116 121] => [99 114 117] [115 116 121]
        .chunks(3)
        // iterate through 3byte chunks and shift every byte
        .map(shift_bytes_per_chunk)
        // iterate through shifted chunks and replace witch chars from the lookup table
        // Example: ascii_chunk[99 114 117] => base64_chunk[24 55 9] => base64_chars[Y 3 J]
        .flat_map(|chunk| encode_chunks(chunk));

    String::from_iter(encoded)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        {
            let res = encode(String::from("a"));
            let exp = String::from("YQ==");
            assert_eq!(res, exp);
        }

        {
            let res = encode(String::from("abc"));
            let exp = String::from("YWJj");
            assert_eq!(res, exp);
        }

        {
            let res = encode(String::from("crusty0gphr"));
            let exp = String::from("Y3J1c3R5MGdwaHI=");
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn test_shift_bytes() {}
}
