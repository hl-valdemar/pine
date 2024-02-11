pub mod color;

use winit::window::Window as WinitWindow;

use crate::{error::PineError, windowing::Window};
#[derive(Debug)]
pub struct FrameData<'surface> {
    pub clear_color: wgpu::Color,
    pub surface: wgpu::Surface<'surface>,
}

#[derive(Debug)]
pub struct FrameDataBuilder<'surface> {
    pub clear_color: Option<wgpu::Color>,
    pub surface: Option<wgpu::Surface<'surface>>,
}

impl<'b> Default for FrameDataBuilder<'b> {
    fn default() -> Self {
        Self {
            clear_color: Some(wgpu::Color::BLACK),
            surface: None,
        }
    }
}

impl<'b> FrameDataBuilder<'b> {
    pub fn with_surface(mut self, surface: wgpu::Surface<'b>) -> Self {
        self.surface = Some(surface);
        self
    }

    pub fn with_clear_color(mut self, clear_color: wgpu::Color) -> Self {
        self.clear_color = Some(clear_color);
        self
    }

    pub fn build(self) -> FrameData<'b> {
        FrameData {
            clear_color: self.clear_color.unwrap(),
            surface: self.surface.expect("No surface found in frame builder"),
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
}

impl Renderer {
    pub async fn new(window: &WinitWindow) -> Result<Self, PineError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance
            .create_surface(window)
            .map_err(|err| PineError::CreateSurfaceError(err))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(PineError::RequestAdapterError)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .map_err(|err| PineError::RequestDeviceError(err))?;

        let surface_config =
            Self::configure_surface(&adapter, &device, &surface, window.inner_size());

        let renderer = Self {
            instance,
            adapter,
            device,
            queue,
            surface_config,
        };
        Ok(renderer)
    }

    /// Configures the given surface.
    fn configure_surface(
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);
        if surface_caps.formats.is_empty() {
            panic!("No texture formats found in surface capabilities")
        }

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, // For V-Sync. Change as needed.
            desired_maximum_frame_latency: 2,      // Default
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(device, &config);
        config
    }

    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
    }

    pub fn prepare<'window>(
        &self,
        window: &'window Window,
    ) -> Result<FrameData<'window>, PineError> {
        let surface = self
            .instance
            .create_surface(&window.handle)
            .map_err(|err| PineError::CreateSurfaceError(err))?;

        let frame_data_builder = FrameDataBuilder::default()
            .with_surface(surface)
            .with_clear_color(window.clear_color.into());

        let data = frame_data_builder.build();
        Ok(data)
    }

    pub fn render<'rpass>(&self, frame_data: &'rpass FrameData) {
        frame_data
            .surface
            .configure(&self.device, &self.surface_config);

        let surface_texture = frame_data.surface.get_current_texture().unwrap();
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.create_encoder();

        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(frame_data.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // render_pass.set_pipeline(&pipeline);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // tracing::info!("Resize method called from backend!");

        if new_size.width > 0 && new_size.height > 0 {
            // As the surface is currently recreated on every render pass, it's configured upon
            // creation in the `render` function.
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            // self.camera.aspect_ratio = new_size.width as f32 / new_size.height as f32;
        }
    }
}
