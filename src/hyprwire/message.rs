#![allow(dead_code)]

use std::{io::Read, os::unix::net::UnixStream};

use crate::hyprwire::{error::HyprwireError, payload::HWPayload, types::{HWMagic, HWMessageKind, HWValue}, vlq::VLQ};


pub struct HWMessage {
    pub kind: HWMessageKind,
    pub payload: Vec<HWPayload>,
}
impl HWMessage {
    pub fn finalize(&self) -> Vec<u8> {
        let mut buf = vec![self.kind as u8];
        for val in self.payload.iter() {
            buf.extend(val.encode());
        }
        buf.push(HWMagic::End as u8);
        buf
    }

    pub fn template_sup() -> Self {
        HWMessage {
            kind: HWMessageKind::Sup,
            payload: vec![HWPayload::compose_varchar("VAX".to_string())],
        }
    }
    pub fn template_hs_ack(version: u32) -> Self {
        HWMessage {
            kind: HWMessageKind::HandshakeAck,
            payload: vec![HWPayload::compose_uint(version)],
        }
    }
    pub fn template_bind_protocol(seq: u32, protocol: &str, version: u32) -> Self {
        HWMessage { 
            kind: HWMessageKind::BindProtocol,
            payload: vec![
                HWPayload::compose_uint(seq),
                HWPayload::compose_varchar(protocol.to_string()),
                HWPayload::compose_uint(version),
            ],
        }
    }
    pub fn template_generic(object_id: u32, method_id: u32, args: Vec<HWPayload>) -> Self {
        let mut payload = vec![
            HWPayload::compose_object(object_id, String::new()),
            HWPayload::compose_uint(method_id),
        ];
        payload.extend(args);
        HWMessage {
            kind: HWMessageKind::Generic,
            payload,
        }
    }
}
/// object decoding detour
impl HWMessage {
    pub fn decode_object(bin: &[u8]) -> Result<Vec<HWPayload>, HyprwireError> {
        let mut payload = Vec::new();
        let mut cursor = 0;

        while cursor < bin.len() {
            let magic = HWMagic::try_from(&bin.get(cursor))?;
            match magic {
                HWMagic::End => break,
                _ => { },
            }
            let offset;
            cursor += 1;

            match magic {
                HWMagic::Varchar => {
                    let Some((data_len, vlq_offset)) = VLQ::decode(&bin[cursor..]) 
                    else { return Err(HyprwireError::DecodeObjectVlq); };

                    let value = HWValue::from_slice(magic, &bin[cursor+vlq_offset..cursor+vlq_offset+data_len as usize])
                        .map_err(|_| HyprwireError::DecodeObjectVarChar)?; // TODO: bubble error

                    payload.push(HWPayload { magic, value });
                    offset = vlq_offset + data_len as usize;
                },
                _ => {
                    let bytes: [u8; 4] = bin[cursor..cursor+4]
                        .try_into()
                        .map_err(|_| HyprwireError::DecodeObjectU8)?; // TODO: bubble error

                    let value = HWValue::from_slice(magic, &bytes)?;

                    payload.push(HWPayload { magic, value });
                    offset = 4;
                },
            }

            cursor += offset;
        }

        Ok(payload)
    }
}
impl TryFrom<&mut UnixStream> for HWMessage {
    type Error = HyprwireError;

    fn try_from(stream: &mut UnixStream) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf)
            .map_err(|e| HyprwireError::MessageStreamRead(e))?;

        println!("-> rx: {:?}", &buf[..n]);

        let kind = HWMessageKind::try_from(&buf.get(0))?;

        let payload = match kind {
            HWMessageKind::NewObject => {
                Self::decode_object(&buf[1..n])?
            },
            HWMessageKind::Generic => {
                if n <= 6 {
                    return Err(HyprwireError::WIP);
                }

                // NOTE: first four bytes of generic messages contain object id
                let mut result = Vec::new();
                let object_id = HWPayload { 
                    magic: HWMagic::ObjectId,
                    value: HWValue::from_slice(HWMagic::ObjectId, &buf[2..6])?,
                };
                let decode = Self::decode_object(&buf[6..n])?;

                result.push(object_id);
                result.extend(decode);
                result
            },
            _ => {
                let magic = HWMagic::try_from(&buf.get(1))?;
                let val = HWValue::from_slice(magic, &buf[2..n])?;

                vec![HWPayload::from_magic_value(magic, val)]
            }
        };

        Ok(Self { kind, payload })
    }
}
