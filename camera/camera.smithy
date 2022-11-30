// camera.smithy
// A simple service that provides image frames


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "dev.wamli.interfaces.camera", crate: "camera" } ]

namespace dev.wamli.interfaces.camera

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#n
use org.wasmcloud.model#U8
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64
use org.wasmcloud.model#Unit

/// The Camera service has a single method, 
/// capture, providing image frames
@wasmbus(
    contractId: "wamli:interfaces:camera",
    providerReceive: true,
    protocol: "2",
    )
service Camera {
  version: "0.1",
  operations: [ Capture ]
}

/// provide an image based on a given configuration
operation Capture {
    input: Configuration,
    output: Image,
}

structure Configuration {
    /// the image format, e.g. MJPEG or YUYV
    @required
    @n(0)
    format: String,

    /// the resolution of the image
    /// in case it is missing, the resolution is arbitrary
    @n(1)
    resolution: Resolution,

    /// in frames per second (fps), 0 is for 'one shot'
    @n(2)
    framerate: U8,

    /// e.g. '/dev/video0'
    @n(3)
    device: String,
}

structure Resolution {
    /// height
    @required
    @n(0)
    height: U32,

    /// width
    @required
    @n(1)
    width: U32, 
}

/// Result of a Capture operation
structure Image {
    /// the number of rows affected by the query
    @required
    @n(0)
    pixels: Blob,

    /// optional error information.
    /// If error is included, other values should be ignored.
    @n(1)
    error: CameraError,
}

/// Error returned with InferenceOutput
union CameraError {
    @n(0)
    invalidFormat: Unit,
    
    @n(1)
    invalidResolution: Unit,

    @n(2)
    invalidFramerate: Unit,

    @n(3)
    invalidDevice: Unit,
}