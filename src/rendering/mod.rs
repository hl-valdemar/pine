pub mod color;
pub mod frame_data;
pub mod scene;
pub mod shaders;

use self::{
    frame_data::{FrameData, FrameDataBuilder},
    scene::{SceneNode2D, Transform},
};

use crate::{error::PineError, windowing::Window};

use winit::window::Window as WinitWindow;

use std::fmt::Debug;

pub trait Renderer: Debug {
    /// Prepares data for the rendering step.
    ///
    /// Returns either the frame data for the rendering step or a PineError.
    fn prepare<'window>(&self, window: &'window Window) -> Result<FrameData<'window>, PineError>;

    /// Renders with the given frame data.
    fn render(&self, frame_data: &FrameData);

    /// Sets the new size for the stored Surface config.
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
}

#[derive(Debug)]
/// State useful for rendering.
pub struct Renderer2D {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    scene_graph: SceneNode2D,
}

impl Renderer for Renderer2D {
    fn prepare<'window>(&self, window: &'window Window) -> Result<FrameData<'window>, PineError> {
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

    fn render(&self, frame_data: &FrameData) {
        frame_data
            .surface
            .configure(&self.device, &self.surface_config);

        let surface_texture = frame_data.surface.get_current_texture().unwrap();
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.create_encoder();

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            self.scene_graph.render();

            // render_pass.set_pipeline(&pipeline);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            // self.camera.aspect_ratio = new_size.width as f32 / new_size.height as f32;
        }
    }
}

impl Renderer2D {
    /// Constructs a new Renderer.
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

        // NB: DEBUGGING
        let scene_graph = SceneNode2D::new()
            .with_transform(Transform::from(1., 1., 1.))
            .add_node(SceneNode2D::new().with_transform(Transform::from(1., 2., 3.)));

        let renderer = Self {
            instance,
            adapter,
            device,
            queue,
            surface_config,
            scene_graph,
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

    /// Creates a new command encoder.
    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
    }
}
