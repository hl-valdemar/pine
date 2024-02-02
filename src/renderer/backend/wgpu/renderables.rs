use super::{camera::Camera, vertex, Vertex};

use std::mem;

use wgpu::util::DeviceExt;

pub trait Renderable {
    fn setup(&mut self, device: &wgpu::Device);
    fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera: &Camera,
    );
}

#[derive(Default)]
pub struct Cube {
    vertex_buf: Option<wgpu::Buffer>,
    index_buf: Option<wgpu::Buffer>,
    index_count: usize,
    // uniform_buf: Option<wgpu::Buffer>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    bind_group: Option<wgpu::BindGroup>,
}

impl Renderable for Cube {
    fn setup(&mut self, device: &wgpu::Device) {
        // Create the vertex and index arrays
        let (vertices, indices) = create_vertices();

        // Create a vertex buffer
        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create an index buffer
        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create the bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(mem::size_of::<[f32; 16]>() as _),
                },
                count: None,
            }],
        });

        self.vertex_buf = Some(vertex_buf);
        self.index_buf = Some(index_buf);
        self.index_count = indices.len();
        self.bind_group_layout = Some(bind_group_layout);
    }

    fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera: &Camera,
    ) {
        let vp_matrix = camera.build_view_projection_matrix();
        let mx_ref: &[f32; 16] = vp_matrix.as_ref();

        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform buffer"),
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout.as_ref().unwrap(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            }],
            label: Some("Uniform bind group"),
        });

        self.bind_group = Some(bind_group);

        render_pass.set_vertex_buffer(0, self.vertex_buf.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(
            self.index_buf.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.set_bind_group(0, &self.bind_group.as_ref().unwrap(), &[]);
        render_pass.draw_indexed(0..self.index_count as u32, 0, 0..1);
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
