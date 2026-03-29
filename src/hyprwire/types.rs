#![allow(dead_code)]

use std::fmt::Display;

use crate::hyprwire::{error::HyprwireError, vlq::VLQ};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum HWMessageKind {
    Invalid = 0,
    Sup = 1,
    HandshakeBegin = 2,
    HandshakeAck = 3,
    HandshakeProtocols = 4,
    BindProtocol = 10,
    NewObject = 11,
    ProtocolError = 12,
    RoundtripReq = 13,
    RoundtripDone = 14,
    Generic = 100,
}
impl Display for HWMessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HWMessageKind::Invalid => write!(f, "Invalid"),
            HWMessageKind::Sup => write!(f, "Sup"),
            HWMessageKind::HandshakeBegin => write!(f, "HandshakeBegin"),
            HWMessageKind::HandshakeAck => write!(f, "HandshakeAck"),
            HWMessageKind::HandshakeProtocols => write!(f, "HandshakeProtocols"),
            HWMessageKind::BindProtocol => write!(f, "BindProtocol"),
            HWMessageKind::NewObject => write!(f, "NewObject"),
            HWMessageKind::ProtocolError => write!(f, "ProtocolError"),
            HWMessageKind::RoundtripReq => write!(f, "RoundtripReq"),
            HWMessageKind::RoundtripDone => write!(f, "RoundtripDone"),
            HWMessageKind::Generic => write!(f, "Generic"),
        }
    }
}
impl HWMessageKind {
    fn match_u8(value: &u8) -> Result<Self, HyprwireError> {
        match value {
            0 => Ok(HWMessageKind::Invalid),
            1 => Ok(HWMessageKind::Sup),
            2 => Ok(HWMessageKind::HandshakeBegin),
            3 => Ok(HWMessageKind::HandshakeAck),
            4 => Ok(HWMessageKind::HandshakeProtocols),
            10 => Ok(HWMessageKind::BindProtocol),
            11 => Ok(HWMessageKind::NewObject),
            12 => Ok(HWMessageKind::ProtocolError),
            13 => Ok(HWMessageKind::RoundtripReq),
            14 => Ok(HWMessageKind::RoundtripDone),
            100 => Ok(HWMessageKind::Generic),
            _ => Err(HyprwireError::MessageKindUnknownKey)
        }
    }
}
impl TryFrom<&u8> for HWMessageKind {
    type Error = HyprwireError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        HWMessageKind::match_u8(&value)
    }
}
impl TryFrom<&Option<&u8>> for HWMessageKind {
    type Error = HyprwireError;

    fn try_from(value: &Option<&u8>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => HWMessageKind::match_u8(v),
            None => Err(HyprwireError::MessageKindTryFrom),
        }
    }
}
impl TryFrom<&[u8]> for HWMessageKind {
    type Error = HyprwireError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.get(0) {
            Some(v) => HWMessageKind::match_u8(v),
            None => Err(HyprwireError::MessageKindTryFrom),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum HWMagic {
    End = 0x00,
    Uint = 0x10,
    Int = 0x11,
    F32 = 0x12,
    Seq = 0x13,
    ObjectId = 0x14,
    Varchar = 0x20,
    Array = 0x21,
    Object = 0x22,
    Fd = 0x40,
}
impl Display for HWMagic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HWMagic::End => write!(f, "end"),
            HWMagic::Uint => write!(f, "uint"),
            HWMagic::Int => write!(f, "int"),
            HWMagic::F32 => write!(f, "f32"),
            HWMagic::Seq => write!(f, "seq"),
            HWMagic::ObjectId => write!(f, "object_id"),
            HWMagic::Varchar => write!(f, "varchar"),
            HWMagic::Array => write!(f, "array"),
            HWMagic::Object => write!(f, "object"),
            HWMagic::Fd => write!(f, "fd"),
        }
    }
}
impl HWMagic {
    fn match_u8(value: &u8) -> Result<Self, HyprwireError> {
        match value {
            0x00 => Ok(HWMagic::End),
            0x10 => Ok(HWMagic::Uint),
            0x11 => Ok(HWMagic::Int),
            0x12 => Ok(HWMagic::F32),
            0x13 => Ok(HWMagic::Seq),
            0x14 => Ok(HWMagic::ObjectId),
            0x20 => Ok(HWMagic::Varchar),
            0x21 => Ok(HWMagic::Array),
            0x22 => Ok(HWMagic::Object),
            0x40 => Ok(HWMagic::Fd),
            _ => Err(HyprwireError::MagicUnknownKey),
        }
    }
    pub fn get_length(key: HWMagic) -> Option<usize> {
        match key {
            HWMagic::End => Some(1),
            HWMagic::Uint => Some(1),
            HWMagic::Int => Some(1),
            HWMagic::F32 => Some(1),
            HWMagic::Seq => Some(1),
            HWMagic::ObjectId => Some(1),
            HWMagic::Varchar => None,
            HWMagic::Array => None,
            HWMagic::Object => None,
            HWMagic::Fd => None,
        }
    }
}
impl TryFrom<&Option<&u8>> for HWMagic {
    type Error = HyprwireError;

    fn try_from(value: &Option<&u8>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => HWMagic::match_u8(v),
            None => Err(HyprwireError::MagicTryFrom),
        }
    }
}
impl TryFrom<&[u8]> for HWMagic {
    type Error = HyprwireError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.get(0) {
            Some(v) => HWMagic::match_u8(v),
            None => Err(HyprwireError::MagicTryFrom),
        }
    }
}

#[derive(Clone)]
pub enum HWValue {
    Uint(u32),
    Int(i32),
    F32(f32),
    Seq(u32),
    ObjectId(u32),
    Varchar(String),
    ArrayUint(Vec<u32>),
    ArrayVarchar(Vec<String>),
    Object(u32, String),
    Fd,
}
impl HWValue {
    fn decode_string(bin: &[u8]) -> Result<String, HyprwireError> {
        String::from_utf8(bin.to_vec())
            .map_err(|e| HyprwireError::DecodeBinUtf8(e))
    }
    fn decode_u32(bin: &[u8]) -> Result<u32, HyprwireError> {
        let bytes: [u8; 4] = bin.try_into().map_err(|e| HyprwireError::DecodeBinU32(e))?;
        Ok(u32::from_le_bytes(bytes))
    }
    fn decode_i32(bin: &[u8]) -> Result<i32, HyprwireError> {
        let bytes: [u8; 4] = bin.try_into().map_err(|e| HyprwireError::DecodeBinI32(e))?;
        Ok(i32::from_le_bytes(bytes))
    }
    fn decode_f32(bin: &[u8]) -> Result<f32, HyprwireError> {
        let bytes: [u8; 4] = bin.try_into().map_err(|e| HyprwireError::DecodeBinF32(e))?;
        Ok(f32::from_le_bytes(bytes))
    }
    fn array_kind_branch(bin: &[u8]) -> Result<Self, HyprwireError> {
        let magic = HWMagic::try_from(&bin[..1])?;

        // TODO: exhaustive matching if needed
        match magic {
            HWMagic::Uint => Ok(HWValue::ArrayUint(Self::array_walk(magic, &bin[1..], Self::decode_u32)?)),
            HWMagic::Seq => Ok(HWValue::ArrayUint(Self::array_walk(magic, &bin[1..], Self::decode_u32)?)),
            HWMagic::ObjectId => Ok(HWValue::ArrayUint(Self::array_walk(magic, &bin[1..], Self::decode_u32)?)),
            HWMagic::Varchar => Ok(HWValue::ArrayVarchar(Self::array_walk(magic, &bin[1..], Self::decode_string)?)),
            _ => Err(HyprwireError::DecodeBinArrayNoMatch)
        }
    }
    fn array_walk<F, T>(
        kind: HWMagic,
        bin: &[u8],
        decoder: F
    ) -> Result<Vec<T>, HyprwireError> where
        F: Fn(&[u8]) -> Result<T, HyprwireError>,
    {
        let Some((array_len, array_len_offset)) = VLQ::decode(&bin[..])
        else { return Err(HyprwireError::ArrayWalkVLQ); };

        let mut cursor = array_len_offset;

        let mut elem = 0;
        let mut values = Vec::<T>::new();
        while elem < array_len as usize {
            let offset;
            match kind {
                HWMagic::Varchar => {
                    let Some((data_len, vlq_offset)) = VLQ::decode(&bin[cursor..]) 
                    else { return Err(HyprwireError::ArrayWalkVarCharVLQ); };

                    let value = decoder(&bin[cursor+vlq_offset..cursor+vlq_offset+data_len as usize])
                        .map_err(|_| HyprwireError::ArrayWalkVarCharValue)?; // TODO: bubble error

                    values.push(value);
                    offset = vlq_offset + data_len as usize;
                },
                _ => {
                    let bytes: [u8; 4] = bin[cursor..cursor+4]
                        .try_into()
                        .map_err(|_| HyprwireError::ArrayWalkDecoderValue)?; // TODO: bubble error

                    let value = decoder(&bytes)?;

                    values.push(value);
                    offset = 4;
                },
            };

            cursor += offset;
            elem += 1;

        }
        Ok(values)
    }
    pub fn from_slice(magic: HWMagic, bin: &[u8]) -> Result<Self, HyprwireError> {
        let value = match magic {
            HWMagic::End => { HWValue::Uint(Self::decode_u32(bin)?) },
            HWMagic::Uint => { HWValue::Uint(Self::decode_u32(bin)?) },
            HWMagic::Int => { HWValue::Int(Self::decode_i32(bin)?) },
            HWMagic::F32 => { HWValue::F32(Self::decode_f32(bin)?) },
            HWMagic::Seq => { HWValue::Seq(Self::decode_u32(bin)?) },
            HWMagic::ObjectId => { HWValue::ObjectId(Self::decode_u32(bin)?) },
            HWMagic::Varchar => { HWValue::Varchar(Self::decode_string(bin)?) },
            HWMagic::Array => { Self::array_kind_branch(bin)? },
            HWMagic::Object => {
                println!("error:from_slice: missed object decoding detour, use HWMessage::try_from");
                println!("error:from_slice:bin: {:?}", bin);
                return Err(HyprwireError::WIP); // TODO: 
            },
            HWMagic::Fd => HWValue::Fd,
        };

        Ok(value)
    }
}
impl Display for HWValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HWValue::Uint(v) => write!(f, "uint({})", v),
            HWValue::Int(v) => write!(f, "int({})", v),
            HWValue::F32(v) => write!(f, "f32({})", v),
            HWValue::Seq(v) => write!(f, "seq({})", v),
            HWValue::ObjectId(v) => write!(f, "object_id({})", v),
            HWValue::Varchar(v) => write!(f, "varchar({})", v),
            HWValue::ArrayUint(items) => write!(f, "array<uint>{:?}", items),
            HWValue::ArrayVarchar(items) => write!(f, "array<varchar>{:?}", items),
            HWValue::Object(id, name) => write!(f, "object({}, {})", id, name),
            HWValue::Fd => write!(f, "fd"),
        }
    }
}
