// This file is @generated by wasmcloud/weld-codegen 0.6.0.
// It is not intended for manual editing.
// namespace: dev.wamli.interfaces.camera

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

/// Error returned with InferenceOutput
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CameraError {
    /// n(0)
    InvalidFormat,

    /// n(1)
    InvalidResolution,

    /// n(2)
    InvalidFramerate,

    /// n(3)
    InvalidDevice,
}

// Encode CameraError as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_camera_error<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &CameraError,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    // encoding union CameraError
    e.array(2)?;
    match val {
        CameraError::InvalidFormat => {
            e.u16(0)?;
            e.null()?;
        }
        CameraError::InvalidResolution => {
            e.u16(1)?;
            e.null()?;
        }
        CameraError::InvalidFramerate => {
            e.u16(2)?;
            e.null()?;
        }
        CameraError::InvalidDevice => {
            e.u16(3)?;
            e.null()?;
        }
    }
    Ok(())
}

// Decode CameraError from cbor input stream
#[doc(hidden)]
pub fn decode_camera_error(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<CameraError, RpcError> {
    let __result = {
        // decoding union CameraError
        let len = d.fixed_array()?;
        if len != 2 {
            return Err(RpcError::Deser(
                "decoding union 'CameraError': expected 2-array".to_string(),
            ));
        }
        match d.u16()? {
            0 => {
                d.null()?;
                CameraError::InvalidFormat
            }

            1 => {
                d.null()?;
                CameraError::InvalidResolution
            }

            2 => {
                d.null()?;
                CameraError::InvalidFramerate
            }

            3 => {
                d.null()?;
                CameraError::InvalidDevice
            }

            n => {
                return Err(RpcError::Deser(format!(
                    "invalid field number for union 'dev.wamli.interfaces.camera#CameraError':{}",
                    n
                )));
            }
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Configuration {
    /// the image format, e.g. MJPEG or YUYV
    #[serde(default)]
    pub format: String,
    /// the resolution of the image
    /// in case it is missing, the resolution is arbitrary
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    /// in frames per second (fps), 0 is for 'one shot'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framerate: Option<u8>,
    /// e.g. '/dev/video0'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

// Encode Configuration as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_configuration<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Configuration,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(4)?;
    e.str(&val.format)?;
    if let Some(val) = val.resolution.as_ref() {
        encode_resolution(e, val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.framerate.as_ref() {
        e.u8(*val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.device.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode Configuration from cbor input stream
#[doc(hidden)]
pub fn decode_configuration(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<Configuration, RpcError> {
    let __result = {
        let mut format: Option<String> = None;
        let mut resolution: Option<Option<Resolution>> = Some(None);
        let mut framerate: Option<Option<u8>> = Some(None);
        let mut device: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Configuration, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => format = Some(d.str()?.to_string()),
                    1 => {
                        resolution = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_resolution(d).map_err(|e| {
                                format!("decoding 'dev.wamli.interfaces.camera#Resolution': {}", e)
                            })?))
                        }
                    }
                    2 => {
                        framerate = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.u8()?))
                        }
                    }
                    3 => {
                        device = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "format" => format = Some(d.str()?.to_string()),
                    "resolution" => {
                        resolution = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_resolution(d).map_err(|e| {
                                format!("decoding 'dev.wamli.interfaces.camera#Resolution': {}", e)
                            })?))
                        }
                    }
                    "framerate" => {
                        framerate = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.u8()?))
                        }
                    }
                    "device" => {
                        device = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        Configuration {
            format: if let Some(__x) = format {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Configuration.format (#0)".to_string(),
                ));
            },
            resolution: resolution.unwrap(),
            framerate: framerate.unwrap(),
            device: device.unwrap(),
        }
    };
    Ok(__result)
}
/// Result of a Capture operation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Image {
    /// the number of rows affected by the query
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub pixels: Vec<u8>,
    /// optional error information.
    /// If error is included, other values should be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CameraError>,
}

// Encode Image as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_image<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Image,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(2)?;
    e.bytes(&val.pixels)?;
    if let Some(val) = val.error.as_ref() {
        encode_camera_error(e, val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode Image from cbor input stream
#[doc(hidden)]
pub fn decode_image(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Image, RpcError> {
    let __result = {
        let mut pixels: Option<Vec<u8>> = None;
        let mut error: Option<Option<CameraError>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Image, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => pixels = Some(d.bytes()?.to_vec()),
                    1 => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_camera_error(d).map_err(|e| {
                                format!("decoding 'dev.wamli.interfaces.camera#CameraError': {}", e)
                            })?))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "pixels" => pixels = Some(d.bytes()?.to_vec()),
                    "error" => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_camera_error(d).map_err(|e| {
                                format!("decoding 'dev.wamli.interfaces.camera#CameraError': {}", e)
                            })?))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        Image {
            pixels: if let Some(__x) = pixels {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Image.pixels (#0)".to_string(),
                ));
            },
            error: error.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Resolution {
    /// height
    #[serde(default)]
    pub height: u32,
    /// width
    #[serde(default)]
    pub width: u32,
}

// Encode Resolution as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_resolution<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Resolution,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(2)?;
    e.u32(val.height)?;
    e.u32(val.width)?;
    Ok(())
}

// Decode Resolution from cbor input stream
#[doc(hidden)]
pub fn decode_resolution(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Resolution, RpcError> {
    let __result = {
        let mut height: Option<u32> = None;
        let mut width: Option<u32> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Resolution, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => height = Some(d.u32()?),
                    1 => width = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "height" => height = Some(d.u32()?),
                    "width" => width = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        }
        Resolution {
            height: if let Some(__x) = height {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Resolution.height (#0)".to_string(),
                ));
            },

            width: if let Some(__x) = width {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Resolution.width (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// The Camera service has a single method,
/// capture, providing image frames
/// wasmbus.contractId: wamli:interfaces:camera
/// wasmbus.providerReceive
#[async_trait]
pub trait Camera {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wamli:interfaces:camera"
    }
    /// provide an image based on a given configuration
    async fn capture(&self, ctx: &Context, arg: &Configuration) -> RpcResult<Image>;
}

/// CameraReceiver receives messages defined in the Camera service trait
/// The Camera service has a single method,
/// capture, providing image frames
#[doc(hidden)]
#[async_trait]
pub trait CameraReceiver: MessageDispatch + Camera {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "Capture" => {
                let value: Configuration =
                    wasmbus_rpc::common::decode(&message.arg, &decode_configuration)
                        .map_err(|e| RpcError::Deser(format!("'Configuration': {}", e)))?;
                let resp = Camera::capture(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_image(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Camera::{}",
                message.method
            ))),
        }
    }
}

/// CameraSender sends messages to a Camera service
/// The Camera service has a single method,
/// capture, providing image frames
/// client for sending Camera messages
#[derive(Debug)]
pub struct CameraSender<T: Transport> {
    transport: T,
}

impl<T: Transport> CameraSender<T> {
    /// Constructs a CameraSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl CameraSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Camera provider
    /// implementing the 'wamli:interfaces:camera' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wamli:interfaces:camera",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Camera provider
    /// implementing the 'wamli:interfaces:camera' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wamli:interfaces:camera",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Camera for CameraSender<T> {
    #[allow(unused)]
    /// provide an image based on a given configuration
    async fn capture(&self, ctx: &Context, arg: &Configuration) -> RpcResult<Image> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_configuration(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Camera.Capture",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Image = wasmbus_rpc::common::decode(&resp, &decode_image)
            .map_err(|e| RpcError::Deser(format!("'{}': Image", e)))?;
        Ok(value)
    }
}
