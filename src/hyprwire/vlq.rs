#![allow(dead_code)]

pub struct VLQ { }
impl VLQ {
    pub fn encode(value: u64) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut value = value;
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            bytes.push(byte);
            if value == 0 {
                break;
            }
        }
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Option<(u64, usize)> {
        let mut value: u64 = 0;
        let mut shift = 0;
        for (i, &byte) in bytes.iter().enumerate() {
            if shift >= 64 {
                return None; // overflow
            }
            value |= ((byte & 0x7F) as u64) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                return Some((value, i + 1));
            }
        }
        None // truncated
    }
}
