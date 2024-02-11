#[derive(Debug)]
pub enum PineError {
    // Window and event loop
    EventLoopError(winit::error::EventLoopError),
    OsError(winit::error::OsError),

    // Rendering
    CreateSurfaceError(wgpu::CreateSurfaceError),
    RequestDeviceError(wgpu::RequestDeviceError),
    RequestAdapterError,
}
