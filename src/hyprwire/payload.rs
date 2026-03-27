#![allow(dead_code)]

use crate::hyprwire::{types::{HWMagic, HWValue}, vlq::VLQ};

pub struct HWPayload {
    pub magic: HWMagic,
    pub value: HWValue,
}
impl HWPayload {
    pub fn compose_uint(uint: u32) -> Self {
        Self::compose(HWMagic::Uint, HWValue::Uint(uint))
    }
    pub fn compose_int(int: i32) -> Self {
        Self::compose(HWMagic::Int, HWValue::Int(int))
    }
    pub fn compose_f32(f32: f32) -> Self {
        Self::compose(HWMagic::F32, HWValue::F32(f32))
    }
    pub fn compose_seq(seq: u32) -> Self {
        Self::compose(HWMagic::Seq, HWValue::Seq(seq))
    }
    pub fn compose_object_id(object_id: u32) -> Self {
        Self::compose(HWMagic::ObjectId, HWValue::ObjectId(object_id))
    }
    pub fn compose_varchar(varchar: String) -> Self {
        Self::compose(HWMagic::Varchar, HWValue::Varchar(varchar))
    }
    pub fn compose_object(object_id: u32, object: String) -> Self {
        Self::compose(HWMagic::Object, HWValue::Object(object_id, object))
    }
    pub fn compose_fd() -> Self {
        Self::compose(HWMagic::Fd, HWValue::Fd)
    }

    fn compose(magic: HWMagic, value: HWValue) -> Self {
        if !Self::validate_magic(magic, &value) {
            panic!("Composition error: magic and value must match.");
        }
        HWPayload {
            magic,
            value,
        }
    }

    fn validate_magic(magic: HWMagic, value: &HWValue) -> bool {
        match value {
            HWValue::Uint(_) => magic == HWMagic::Uint,
            HWValue::Int(_) => magic == HWMagic::Int,
            HWValue::F32(_) => magic == HWMagic::F32,
            HWValue::Seq(_) => magic == HWMagic::Seq,
            HWValue::ObjectId(_) => magic == HWMagic::ObjectId,
            HWValue::Varchar(_) => magic == HWMagic::Varchar,
            HWValue::ArrayUint(_) => magic == HWMagic::Array,
            HWValue::ArrayVarchar(_) => magic == HWMagic::Array,
            HWValue::Object(_, _) => magic == HWMagic::Object,
            HWValue::Fd => magic == HWMagic::Fd,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        bytes.push(self.magic as u8);
        bytes.extend(self.encode_value());

        bytes
    }
    fn encode_value(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        match &self.value {
            HWValue::Uint(v) => {
                buf.extend(v.to_le_bytes())
            },
            HWValue::Int(v) => {
                buf.extend(v.to_le_bytes())
            },
            HWValue::F32(v) => {
                buf.extend(v.to_le_bytes());
            },
            HWValue::Seq(v) => {
                buf.extend(v.to_le_bytes());
            },
            HWValue::ObjectId(v) => {
                buf.extend(v.to_le_bytes());
            },
            HWValue::Varchar(s) => {
                buf.extend(VLQ::encode(s.len() as u64));
                buf.extend(s.as_bytes());
            },
            HWValue::ArrayUint(vals) => {
                buf.push(HWMagic::Uint as u8);
                buf.extend(VLQ::encode(vals.len() as u64));
                for v in vals {
                    buf.extend(v.to_le_bytes());
                }
            },
            HWValue::ArrayVarchar(vals) => {
                buf.push(HWMagic::Varchar as u8);
                buf.extend(VLQ::encode(vals.len() as u64));
                for s in vals {
                    buf.extend(VLQ::encode(s.len() as u64));
                    buf.extend(s.as_bytes());
                }
            },
            HWValue::Object(id, _name) => { // TODO: clean up
                buf.extend(id.to_le_bytes());
                // buf.extend(VLQ::encode(name.len() as u64));
                // buf.extend(name.as_bytes());
            }
            HWValue::Fd => { },
        }
        buf
    }
    pub fn from_magic_value(magic: HWMagic, value: HWValue) -> Self {
        Self { magic, value }
    }
}
