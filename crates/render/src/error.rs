use wgpu::{CreateSurfaceError, RequestDeviceError};

#[derive(thiserror::Error, Debug)]
pub enum RendererBuildError {
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),

    #[error("Found no supported Adaptors")]
    RequestAdaptorError,

    #[error("Surface Unsupported By Adapter")]
    SurfaceConfigError,

    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),
}
