// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    error,
    io::{Read, Result as IoResult},
};

/// Returns the variable length integer of the given value.
/// https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
pub fn variable_length_integer(value: u64) -> Vec<u8> {
    match value {
        // bounded by u8::max_value()
        0..=252 => vec![value as u8],
        // bounded by u16::max_value()
        253..=65535 => [&[0xfd], &(value as u16).to_le_bytes()[..]].concat(),
        // bounded by u32::max_value()
        65536..=4_294_967_295 => [&[0xfe], &(value as u32).to_le_bytes()[..]].concat(),
        // bounded by u64::max_value()
        _ => [&[0xff], &value.to_le_bytes()[..]].concat(),
    }
}

/// Decode the value of a variable length integer.
/// https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
pub fn read_variable_length_integer<R: Read>(mut reader: R) -> IoResult<usize> {
    let mut flag = [0u8; 1];
    reader.read_exact(&mut flag)?;

    match flag[0] {
        0..=252 => Ok(flag[0] as usize),
        0xfd => {
            let mut size = [0u8; 2];
            reader.read_exact(&mut size)?;
            match u16::from_le_bytes(size) {
                s if s < 253 => Err(error("Invalid variable size integer")),
                s => Ok(s as usize),
            }
        }
        0xfe => {
            let mut size = [0u8; 4];
            reader.read_exact(&mut size)?;
            match u32::from_le_bytes(size) {
                s if s < 65536 => Err(error("Invalid variable size integer")),
                s => Ok(s as usize),
            }
        }
        _ => {
            let mut size = [0u8; 8];
            reader.read_exact(&mut size)?;
            match u64::from_le_bytes(size) {
                s if s < 4_294_967_296 => Err(error("Invalid variable size integer")),
                s => Ok(s as usize),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const LENGTH_VALUES: [(u64, [u8; 9]); 14] = [
        (20, [0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (32, [0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (200, [0xc8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (252, [0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (253, [0xfd, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (40000, [0xfd, 0x40, 0x9c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (65535, [0xfd, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (65536, [0xfe, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (2000000000, [0xfe, 0x00, 0x94, 0x35, 0x77, 0x00, 0x00, 0x00, 0x00]),
        (2000000000, [0xfe, 0x00, 0x94, 0x35, 0x77, 0x00, 0x00, 0x00, 0x00]),
        (4294967295, [0xfe, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00]),
        (4294967296, [0xff, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]),
        (500000000000000000, [
            0xff, 0x00, 0x00, 0xb2, 0xd3, 0x59, 0x5b, 0xf0, 0x06,
        ]),
        (18446744073709551615, [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ]),
    ];

    #[test]
    fn test_variable_length_integer() {
        LENGTH_VALUES.iter().for_each(|(size, expected_output)| {
            let variable_length_int = variable_length_integer(*size);
            let pruned_expected_output = &expected_output[..variable_length_int.len()];
            assert_eq!(pruned_expected_output, &variable_length_int[..]);
        });
    }

    #[test]
    fn test_read_variable_length_integer() {
        LENGTH_VALUES.iter().for_each(|(expected_size, _expected_output)| {
            let variable_length_int = variable_length_integer(*expected_size);
            let size = read_variable_length_integer(&variable_length_int[..]).unwrap();
            assert_eq!(*expected_size as usize, size);
        });
    }
}
