use super::GlobalResources;
use crate::{
    builders::{
        BindGroupBuilder, BindGroupLayoutBuilder, BufferBuilder, VertexBufferLayoutBuilder,
    },
    primitives::Rect,
    vertex::Vertex, Bezier,
};
use std::rc::Rc;
use helium_core::Position;

pub struct BezierPipeline {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    global: Rc<GlobalResources>,
	draw_queue: Vec<Bezier>
}

impl BezierPipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        global: Rc<GlobalResources>,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Bezier Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/bezier.wgsl").into()),
        });

        let layout = BindGroupLayoutBuilder::new()
            .label("Bezier bind group layout")
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .build(device);

        let vertex_buffer_layout = VertexBufferLayoutBuilder::new()
            .array_stride(size_of::<Vertex>() as u64)
            .attribute(0, wgpu::VertexFormat::Float32x2)
            .attribute(size_of::<[f32; 2]>() as u64, wgpu::VertexFormat::Float32x4)
            .attribute(size_of::<[f32; 6]>() as u64, wgpu::VertexFormat::Float32x2)
            .build();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Rect Pipeline Layout"),
            bind_group_layouts: &[global.window_layout(), &layout],
            push_constant_ranges: &[],
        });

        // TODO create a builder for this
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rect Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[vertex_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: None,
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            layout,
            global,
			draw_queue: Vec::new()
        }
    }

	/// Draw a [`Bezier`]. 
	/// 
	/// This method pushes the bezier to the draw queue, which is 
	/// drawn to the screen every frame. 
	pub fn draw(&mut self, bezier: Bezier){
		self.draw_queue.push(bezier);
	}

    pub fn render(&mut self, device: &wgpu::Device, pass: &mut wgpu::RenderPass) {
		for bezier in self.draw_queue.drain(..){
			
			let vertices = Vertex::bezier(
				[
					Position::new(20.0, 20.0),	
					Position::new(120.0, 100.0),	
					Position::new(320.0, 50.0),	
					Position::new(520.0, 120.0),	
				], 
				bezier.color.clone()
			);

			let vertex_buffer = BufferBuilder::new()
				.label("Bezier vertex buffer")
				.vertex()
				.init(&vertices)
				.build(device);
	
			let size = BufferBuilder::new()
				.label("Bezier size buffer")
				.uniform()
				.copy_dst()
				.init(&[bezier.size])
				.build(device);
	
			let position = BufferBuilder::new()
				.label("Bezier position buffer")
				.uniform()
				.copy_dst()
				.init(&[bezier.position])
				.build(device);
	
			let corner_radius = BufferBuilder::new()
				.label("Bezier corner radius buffer")
				.uniform()
				.copy_dst()
				.init(&[bezier.corner_radius])
				.build(device);
	
			let bezier_bind_group = BindGroupBuilder::new()
				.label("Bezier bind group")
				.buffer(&corner_radius)
				.buffer(&size)
				.buffer(&position)
				.build(&self.layout, device);
	
			pass.set_pipeline(&self.pipeline);
			pass.set_bind_group(0, self.global.window_bind_group(), &[]);
			pass.set_bind_group(1, &bezier_bind_group, &[]);
			pass.set_vertex_buffer(0, vertex_buffer.slice(..));
	
			pass.draw(0..vertices.len() as u32, 0..1);
		}
    }
}
