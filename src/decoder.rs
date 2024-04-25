use std::io::{Error, ErrorKind};

use crate::lookup_table::{Lookup, LookupTable};

fn shift_bytes_per_chunk(chunk: Vec<u8>) -> Vec<u8> {
    let out = match chunk.len() {
        // 2byte chunk input
        2 => vec![
            (chunk[0] & 0b00111111) << 2 | chunk[1] >> 4,
            (chunk[1] & 0b00001111) << 4,
        ],

        // 3byte chunk input
        3 => vec![
            (chunk[0] & 0b00111111) << 2 | chunk[1] >> 4,
            (chunk[1] & 0b00001111) << 4 | chunk[2] >> 2,
            (chunk[2] & 0b00000011) << 6,
        ],

        // 4byte chunk input
        4 => vec![
            (chunk[0] & 0b00111111) << 2 | chunk[1] >> 4,
            (chunk[1] & 0b00001111) << 4 | chunk[2] >> 2,
            (chunk[2] & 0b00000011) << 6 | chunk[3] & 0b00111111,
        ],

        // default
        _ => vec![],
    };
    out.into_iter().filter(|&x| x > 0).collect()
}

fn decode_chunk(chunk: &[char]) -> Vec<u8> {
    chunk
        .iter()
        // get all chars that are not the padding char
        .filter(|ch| *ch != &LookupTable.get_padding())
        // iterate through shifted chunks and replace witch indices from the lookup table
        // Example: base64_chunk[24 55 9] => ascii_chunk[99 114 117] => ascii_chars[c r u]
        .map(|ch| {
            LookupTable
                .get_b64_index(*ch)
                .expect("unable to locate char")
        })
        .collect()
}

pub fn decode(input: String) -> Result<String, Error> {
    if input.as_bytes().len() % 4 != 0 {
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    let decoded: Vec<u8> = input
        .chars()
        .collect::<Vec<char>>()
        // split byte array into 3byte chunks
        // Example: [28 43 46 44 45 50] => [28 43 46 44] [45 50]
        .chunks(4)
        .map(|chunk| decode_chunk(chunk))
        // iterate through 4byte chunks and shift every byte
        .flat_map(shift_bytes_per_chunk)
        .collect();

    Ok(String::from_utf8(decoded).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        {
            let res = decode(String::from("YQ=="));
            let exp = String::from("a");

            match res {
                Ok(res) => assert_eq!(res, exp),
                Err(err) => assert_eq!(
                    err.to_string(),
                    Error::from(ErrorKind::InvalidInput).to_string()
                ),
            };
        }

        {
            let res = decode(String::from("YWJj"));
            let exp = String::from("abc");

            match res {
                Ok(res) => assert_eq!(res, exp),
                Err(err) => assert_eq!(
                    err.to_string(),
                    Error::from(ErrorKind::InvalidInput).to_string()
                ),
            };
        }

        {
            let res = decode(String::from("Y3J1c3R5MGdwaHI="));
            let exp = String::from("crusty0gphr");

            match res {
                Ok(res) => assert_eq!(res, exp),
                Err(err) => assert_eq!(
                    err.to_string(),
                    Error::from(ErrorKind::InvalidInput).to_string()
                ),
            };
        }
    }
}
