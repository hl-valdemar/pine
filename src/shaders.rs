use std::fs;

/// Creates a shader module from the given shader source using the given device.
pub fn load_shader(device: &wgpu::Device, path: &str) -> Result<wgpu::ShaderModule, ()> {
    let shader_module = match fs::read_to_string(path) {
        Ok(shader_src) => device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(path),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        }),
        Err(err) => {
            tracing::error!("failed to load shader: {}", err);
            return Err(());
        }
    };
    Ok(shader_module)
}
