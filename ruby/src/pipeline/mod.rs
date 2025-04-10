mod circle;
mod icon;
mod image;
mod rect;
mod text;
mod bezier;
use crate::builders::{BindGroupBuilder, BindGroupLayoutBuilder, BufferBuilder};
pub use circle::CirclePipeline;
use helium_core::Size;
pub use icon::IconPipeline;
pub use image::ImagePipeline;
pub use rect::RectPipeline;
pub use text::TextPipeline;
pub use bezier::BezierPipeline;

/// Global resources needed by all pipelines
#[derive(Debug)]
pub struct GlobalResources {
    window_buffer: wgpu::Buffer,
    window_bind_group: wgpu::BindGroup,
    window_layout: wgpu::BindGroupLayout,
}

impl GlobalResources {
    pub fn new(device: &wgpu::Device, window_size: Size) -> Self {
        let window_layout = BindGroupLayoutBuilder::new()
            .label("Global window bind group layout")
            .uniform(wgpu::ShaderStages::VERTEX_FRAGMENT)
            .build(&device);

        let window_buffer = BufferBuilder::new()
            .label("Global window buffer")
            .copy_dst()
            .uniform()
            .init(&[window_size])
            .build(&device);

        let window_bind_group = BindGroupBuilder::new()
            .label("Global window bind group")
            .buffer(&window_buffer)
            .build(&window_layout, &device);

        Self {
            window_buffer,
            window_bind_group,
            window_layout,
        }
    }

    /// The global window `Buffer`
    pub fn window_buffer(&self) -> &wgpu::Buffer {
        &self.window_buffer
    }

    /// The global window `BindGroup`
    pub fn window_bind_group(&self) -> &wgpu::BindGroup {
        &self.window_bind_group
    }

    /// The global window `BindGroup`
    pub fn window_layout(&self) -> &wgpu::BindGroupLayout {
        &self.window_layout
    }
}
