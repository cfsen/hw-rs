#![allow(dead_code)]

use std::{array::TryFromSliceError, fmt::Display, string::FromUtf8Error};

pub enum HyprwireError {
    ArrayWalkDecoderValue,
    ArrayWalkVLQ,
    ArrayWalkVarCharVLQ,
    ArrayWalkVarCharValue,

    DecodeBinArrayNoMatch,
    DecodeBinF32(TryFromSliceError),
    DecodeBinI32(TryFromSliceError),
    DecodeBinU32(TryFromSliceError),
    DecodeBinUtf8(FromUtf8Error),

    HyprlandInstanceSignature(String),

    MagicTryFrom,
    MagicUnknownKey,

    MessageKindTryFrom,
    MessageKindUnknownKey,

    MessageStreamRead(std::io::Error),

    SocketConnect(std::io::Error),
    SocketProtocolError,
    SocketWrite(std::io::Error),

    WIP,

    XDGRuntimePath(String),
}
impl Display for HyprwireError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HyprwireError::ArrayWalkDecoderValue => write!(f, "ArrayWalk:DecodeValue"),
            HyprwireError::ArrayWalkVLQ => write!(f, "ArrayWalk:VLQ"),
            HyprwireError::ArrayWalkVarCharVLQ => write!(f, "ArrayWalk:VarCharVLQ"),
            HyprwireError::ArrayWalkVarCharValue => write!(f, "ArrayWalk:VarCharValue"),

            HyprwireError::DecodeBinArrayNoMatch => write!(f, "DecodeBin:Array: No match"),
            HyprwireError::DecodeBinF32(try_from_slice_error) => write!(f, "DecodeBin:F32: {}", try_from_slice_error),
            HyprwireError::DecodeBinI32(try_from_slice_error) => write!(f, "DecodeBin:I32: {}", try_from_slice_error),
            HyprwireError::DecodeBinU32(try_from_slice_error) => write!(f, "DecodeBin:U32: {}", try_from_slice_error),
            HyprwireError::DecodeBinUtf8(utf8_error) => write!(f, "DecodeBin:Utf8: {}", utf8_error),

            HyprwireError::HyprlandInstanceSignature(e) => write!(f, "Socket:HyprlandInstanceSignature: {}", e),

            HyprwireError::MagicTryFrom => write!(f, "Magic: TryFrom failure"),
            HyprwireError::MagicUnknownKey => write!(f, "Magic: Unknown key"),

            HyprwireError::MessageKindTryFrom => write!(f, "Message kind: TryFrom failure"),
            HyprwireError::MessageKindUnknownKey => write!(f, "Message kind: Unknown key"),

            HyprwireError::MessageStreamRead(error) => write!(f, "Message: Stream read error: {}", error),

            HyprwireError::SocketConnect(e) => write!(f, "Socket:Connect: {}", e),
            HyprwireError::SocketProtocolError => write!(f, "Socket:Protocol error"),
            HyprwireError::SocketWrite(e) => write!(f, "Socket:Write: {}", e),

            HyprwireError::WIP => write!(f, "Error:WIP"),

            HyprwireError::XDGRuntimePath(e) => write!(f, "Socket:XDGRuntimePath: {}", e),
        }
    }
}
