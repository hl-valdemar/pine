mod shaders;

use crate::renderer::backend::wgpu::shaders::ShaderModule;

use super::Backend;

use bytemuck::{Pod, Zeroable};
use winit::window::Window;

use std::mem;

/// Holds state for the wgpu backend.
pub struct Wgpu {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl Backend for Wgpu {
    fn resize(&mut self, new_size: winit::dpi::LogicalSize<u32>) {
        tracing::info!("Resize method called from backend!");
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            // Remember to update the camera when the time for that comes around...
        }
    }

    fn render(&mut self, window: &Window) {
        tracing::info!("Render method called from backend!");

        // Create the surface and configure it.
        let surface = Self::create_surface(&self.instance, &window);
        surface.configure(&self.device, &self.surface_config);

        // Get the surface texture from the surface.
        let surface_texture = Self::get_current_texture(&surface);

        // Create the view from the texture.
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        // TEMPORARY: CREATE THE RENDERING PIPELINE. //
        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                mem::size_of::<[f32; 16]>() as _
                            ),
                        },
                        count: None,
                    }],
                });

        let shader_module = ShaderModule::load_shader(&self.device, "shader.wgsl")
            .expect("failed to load shader module");

        let pipeline = Self::create_render_pipeline(
            &self.device,
            &self.surface_config,
            &bind_group_layout,
            &shader_module,
            "vs_main",
            "fs_main",
        );
        ///////////////////////////////////////////////

        // New scope to make sure that the render pass is dropped before the command encoder is
        // used for submission to the command queue.
        {
            // Begin the render pass.
            let mut render_pass = self.begin_render_pass(&mut encoder, &view);

            // Set the pipeline.
            render_pass.set_pipeline(&pipeline);
        }

        // Submit the commands to the GPU.
        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    }
}

impl Wgpu {
    /// Constructs a new wgpu backend.
    ///
    /// The window argument is used to create the surface.
    pub async fn new(window: &Window) -> Wgpu {
        let instance = Self::create_instance();
        let surface = Self::create_surface(&instance, &window);
        let adapter = Self::request_adapter(&instance, &surface).await;
        let (device, queue) = Self::request_device(&adapter).await;

        let surface_config =
            Self::configure_surface(&adapter, &device, &surface, window.inner_size());

        Self {
            instance,
            adapter,
            device,
            queue,
            surface_config,
        }
    }

    /// Constructs a wgpu instance.
    pub fn create_instance() -> wgpu::Instance {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        instance
    }

    /// Constructs a surface from the given instance and window.
    pub fn create_surface<'window>(
        instance: &wgpu::Instance,
        window: &'window Window,
    ) -> wgpu::Surface<'window> {
        instance
            .create_surface(window)
            .expect("failed to create surface")
    }

    /// Fetches an adapter from the given instance compatible with the given surface.
    async fn request_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface<'_>,
    ) -> wgpu::Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("failed to get an adapter")
    }

    /// Fetches a device (and a corresponding command queue) using the given adapter.
    ///
    /// NB: the device is just a handle on the GPU.
    async fn request_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("failed to get a device")
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

    /// Fetches the current texture from the given surface.
    pub fn get_current_texture(surface: &wgpu::Surface) -> wgpu::SurfaceTexture {
        surface
            .get_current_texture()
            .expect("failed to get the current texture from the given surface")
    }

    /// Begins a render pass from the given command encoder.
    pub fn begin_render_pass<'pass, 'a: 'pass>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'pass> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                        a: 1.,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        })
    }

    /// Constructs a render pipeline from the given parameters.
    pub fn create_render_pipeline(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        bind_group_layout: &wgpu::BindGroupLayout,
        shader_module: &wgpu::ShaderModule,
        vertex_shader_entry: &str,
        fragment_shader_entry: &str,
    ) -> wgpu::RenderPipeline {
        // Create the layout of the pipeline.
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render pipeline layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: vertex_shader_entry,
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: fragment_shader_entry,
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    _pos: [f32; 4],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x4,
            }],
        }
    }
}

fn vertex(pos: [i8; 3]) -> Vertex {
    Vertex {
        _pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
    }
}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let vertex_data = [
        // top (0, 0, 1)
        vertex([-1, -1, 1]),
        vertex([1, -1, 1]),
        vertex([1, 1, 1]),
        vertex([-1, 1, 1]),
        // bottom (0, 0, -1)
        vertex([-1, 1, -1]),
        vertex([1, 1, -1]),
        vertex([1, -1, -1]),
        vertex([-1, -1, -1]),
        // right (1, 0, 0)
        vertex([1, -1, -1]),
        vertex([1, 1, -1]),
        vertex([1, 1, 1]),
        vertex([1, -1, 1]),
        // left (-1, 0, 0)
        vertex([-1, -1, 1]),
        vertex([-1, 1, 1]),
        vertex([-1, 1, -1]),
        vertex([-1, -1, -1]),
        // front (0, 1, 0)
        vertex([1, 1, -1]),
        vertex([-1, 1, -1]),
        vertex([-1, 1, 1]),
        vertex([1, 1, 1]),
        // back (0, -1, 0)
        vertex([1, -1, 1]),
        vertex([-1, -1, 1]),
        vertex([-1, -1, -1]),
        vertex([1, -1, -1]),
    ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data.to_vec(), index_data.to_vec())
}
