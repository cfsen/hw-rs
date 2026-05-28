#![allow(dead_code)]

use std::{io::Write, os::unix::net::UnixStream};

use crate::hyprwire::{
    error::HyprwireError,
    message::HWMessage,
    payload::HWPayload,
    types::{
        HWMessageKind,
        HWValue
    }
};

pub struct HyprpaperIPCHandshake {
    seq: u32,
    manager_id: u32,
    wallpaper_id: u32,
}

#[repr(u32)]
pub enum HyprwireMethodId {
    GetWallpaperObject = 0,
}

#[repr(u32)]
pub enum HyprpaperMethodId {
    SetWallpaperPath = 0,
    SetTargetMonitor = 2,
    Apply = 3,
}

pub struct HyprpaperIPC {}
impl HyprpaperIPC {
    /// connect to hyprpaper socket and set wallpaper
    pub fn set_wallpaper(path: &str, monitor_id: &str) -> Result<(), HyprwireError> {
        let mut stream = Self::socket_connect()?;
        let hs = Self::handshake(&mut stream)?;

        // wallpaper
        Self::socket_write(&mut stream, HWMessage::template_generic(
                hs.wallpaper_id,
                HyprpaperMethodId::SetWallpaperPath as u32,
                vec![HWPayload::compose_varchar(path.to_string())]
        ))?;

        // target monitor
        Self::socket_write(&mut stream, HWMessage::template_generic(
                hs.wallpaper_id,
                HyprpaperMethodId::SetTargetMonitor as u32,
                vec![HWPayload::compose_varchar(monitor_id.to_string())]
        ))?;

        // apply
        Self::socket_write(&mut stream, HWMessage::template_generic(
                hs.wallpaper_id,
                HyprpaperMethodId::Apply as u32,
                vec![]
        ))?;

        // wait for response before closing socket
        let _final_resp = HWMessage::try_from(&mut stream)?;

        Ok(())
    }

    /// performs hyprwire handshakem, returning sequence counter
    fn handshake(stream: &mut UnixStream) -> Result<HyprpaperIPCHandshake, HyprwireError> {
        Self::socket_write(stream, HWMessage::template_sup())?;

        // check server hyprwire protocol support
        let client_hw_version = 1;
        let hw_protocol_ver = Self::get_hw_protocol(stream, client_hw_version)?;

        // ack hyprwire protocol
        Self::socket_write(stream, HWMessage::template_hs_ack(hw_protocol_ver))?;

        // check available hyprpaper protocols
        let client_paper_version = "hyprpaper_core@2";
        let paper_protocol_ver = Self::get_paper_protocol(stream, client_paper_version)?;

        // bind to hyprpaper protocol
        let mut seq = 1;
        Self::socket_write(stream, HWMessage::template_bind_protocol(seq, "hyprpaper_core", paper_protocol_ver))?;
        seq += 1;

        // fetch manager object
        let manager_id = Self::get_manager_object(stream)?;

        // request wallpaper object
        Self::socket_write(stream, HWMessage::template_generic(
                manager_id,
                HyprwireMethodId::GetWallpaperObject as u32,
                vec![HWPayload::compose_seq(1)]
        ))?;
        let wallpaper_id = Self::get_wallpaper_object(stream)?;

        Ok(HyprpaperIPCHandshake { seq, manager_id, wallpaper_id })
    }
    fn get_manager_object(stream: &mut UnixStream) -> Result<u32, HyprwireError> {
        let manager_obj = HWMessage::try_from(stream)?;

        if manager_obj.kind != HWMessageKind::NewObject {
            return Err(HyprwireError::SocketProtocolError)
        }

        let Some(manager_id) = manager_obj.payload.get(0)
        else { return Err(HyprwireError::HyprpaperGetManagerObjNoPayload) };

        match manager_id.value {
            HWValue::Uint(id) => Ok(id),
            _ => Err(HyprwireError::HyprpaperGetManagerObjId),
        }
    }

    fn get_wallpaper_object(stream: &mut UnixStream) -> Result<u32, HyprwireError> {

        let expect_obj = HWMessage::try_from(stream)?;

        if expect_obj.kind != HWMessageKind::NewObject {
            return Err(HyprwireError::SocketProtocolError)
        }

        let Some(wallpaper_id) = expect_obj.payload.get(0)
        else { return Err(HyprwireError::HyprpaperGetWallpaperObjNoPayload) };

        match wallpaper_id.value {
            HWValue::Uint(id) => Ok(id),
            _ => Err(HyprwireError::HyprpaperGetWallpaperObjId),
        }
    }

    /// validates hyprwire protocol during handshake
    fn get_hw_protocol(stream: &mut UnixStream, client_version: u32) -> Result<u32, HyprwireError> {
        let expect_supported_versions = HWMessage::try_from(stream)?;

        if expect_supported_versions.kind != HWMessageKind::HandshakeBegin {
            return Err(HyprwireError::SocketProtocolError);
        }

        // expect one payload containing array of supported versions
        let Some(versions) = expect_supported_versions.payload.get(0)
        else { return Err(HyprwireError::SocketProtocolError); };

        match &versions.value {
            HWValue::ArrayUint(items) => { 
                if items.contains(&client_version) {
                    println!("Supported Hyprwire protocol version.");
                    return Ok(client_version);
                }
            },
            _ => {
                println!("Payload type mismatch!");
            },
        }

        println!("Unsupported Hyprwire protocol version requested: {}", client_version);
        Err(HyprwireError::SocketProtocolError)
    }

    /// validates hyprpaper protocol during handshake
    fn get_paper_protocol(stream: &mut UnixStream, client_paper_version: &str) -> Result<u32, HyprwireError> {
        // pre-process requested hyprpaper version
        let Some((_, version_str)) = client_paper_version.split_once("@") 
            else { return Err(HyprwireError::HyprpaperGetProtocolInvalidVersionStr) };

            let requested_version: u32 = version_str.parse::<u32>()
                .map_err(|_| HyprwireError::HyprpaperGetProtocolUintParseFailure)?;

        // read offered versions from socket
        let expect_supported_versions = HWMessage::try_from(stream)?;

        if expect_supported_versions.kind != HWMessageKind::HandshakeProtocols {
            return Err(HyprwireError::SocketProtocolError);
        }

        for version in expect_supported_versions.payload.iter() {
            match &version.value {
                HWValue::ArrayVarchar(items) => {
                    if items.contains(&client_paper_version.to_string()) {
                        println!("Supported Hyprpaper version");
                        return Ok(requested_version);
                    }
                    else {
                        println!("Server offered protocols:");
                        for i in items {
                            println!("> {}", i);
                        }
                    }
                },
                _ => {
                    println!("Payload type mismatch!");
                },
            }
        }

        println!("Unsupported Hyprpaper version requested: {}", client_paper_version);
        return Err(HyprwireError::SocketProtocolError);
    }

    //
    // socket
    //

    fn socket_connect() -> Result<UnixStream, HyprwireError> {
        let signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .map_err(|e| HyprwireError::HyprlandInstanceSignature(e.to_string()))?;

        let xdg_runtime_path = std::env::var("XDG_RUNTIME_DIR")
            .map_err(|e| HyprwireError::XDGRuntimePath(e.to_string()))?;
        let socket_path = format!("{}/hypr/{}/.hyprpaper.sock", xdg_runtime_path, signature);

        UnixStream::connect(&socket_path).map_err(|e| HyprwireError::SocketConnect(e))
    }

    fn socket_write(stream: &mut UnixStream, msg: HWMessage) -> Result<(), HyprwireError> {
        println!("-> tx: {:?}", &msg.finalize());
        stream.write_all(&msg.finalize())
            .map_err(|e| HyprwireError::SocketWrite(e))?;
        Ok(())
    }
}
