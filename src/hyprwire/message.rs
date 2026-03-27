#![allow(dead_code)]

use std::{io::Read, os::unix::net::UnixStream};

use crate::hyprwire::{error::HyprwireError, payload::HWPayload, types::{HWMagic, HWMessageKind, HWValue}};


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
impl TryFrom<&mut UnixStream> for HWMessage {
    type Error = HyprwireError;

    fn try_from(stream: &mut UnixStream) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf)
            .map_err(|e| HyprwireError::MessageStreamRead(e))?;


        let kind = HWMessageKind::try_from(&buf.get(0))?;
        let magic = HWMagic::try_from(&buf.get(1))?;

        let val = HWValue::from_slice(magic, &buf[2..n])?;

        let payload = vec![HWPayload::from_magic_value(magic, val)];

        Ok(Self { kind, payload })
    }
}
