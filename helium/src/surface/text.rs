use std::io::Cursor;
use image::RgbaImage;
use text_to_png::TextRenderer;
use wgpu::util::DeviceExt;
use crate::{
	app::AppState, Color, surface::Surface, Bounds, Position,Size, vertex::Vertex
};

// FIXME text getting blurry at large window sizes
// FIXME change the color to Color enum
/// A rasterized texture of text  
#[derive(Debug,Clone)]
pub struct TextSurface{
	position:Position,
	size:Size,
	text:String,
	font_size:u8,
	color:String,
	img: RgbaImage
}

impl TextSurface {
	pub fn new(text:&str,color:&str,font_size:u8) -> Self{
		let text_renderer = TextRenderer::default();

		// Render the text as a png
		let text_image = text_renderer.render_text_to_png_data(
			text, 
			font_size, 
			"#000"
		).unwrap();

		let img = image::load(
			Cursor::new(text_image.data), 
			image::ImageFormat::Png
		).unwrap().to_rgba8();
		
		Self {
			position:Position::new(0.0, 0.0), 
			size:Size::new(text_image.size.width as f32, text_image.size.height as f32),
			text:String::from(text), 
			font_size, 
			color:String::from(color),
			img
		}
	}
	
	/// Rasterize the text and return the texture 
	pub fn build(&self,device: &wgpu::Device) -> (wgpu::Texture,wgpu::Extent3d) {
		let texture_size = wgpu::Extent3d{
			width:self.size.width as u32,
			height: self.size.height as u32,
			depth_or_array_layers:1
		};

		let texture = device.create_texture(
			&wgpu::TextureDescriptor {
				size: texture_size,
				mip_level_count: 1,
				sample_count: 1,
				dimension: wgpu::TextureDimension::D2,
				format: wgpu::TextureFormat::Rgba8UnormSrgb,
				usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
				label: Some("Text Texture"),
				view_formats: &[],
			}
		);

		return (texture,texture_size);

	}

	fn to_vertices(&self,width:f32,height:f32) -> Vec<Vertex>{
		let color = Color::default().normalize();
		let x = self.position.x;
		let y = self.position.y;

		let vertex1 = Vertex::new_with_texture(x,y,color,[0.0,0.0]); //Top left
		let vertex2 = Vertex::new_with_texture(x+width,y,color,[1.0,0.0]); // Top right
		let vertex3 = Vertex::new_with_texture(x, y+height,color,[0.0,1.0]); //Bottom left
		let vertex4 = Vertex::new_with_texture(x+width,y,color,[1.0,0.0]); //Top right
		let vertex5 = Vertex::new_with_texture(x, y+height,color,[0.0,1.0]); // Bottom left
		let vertex6 = Vertex::new_with_texture(x+width, y+height,color,[1.0,1.0]); //Bottom right
	
		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}
}

impl Surface for TextSurface {
	fn draw(
		&self,
		render_pass:&mut wgpu::RenderPass,
		context: &crate::app::RenderContext,
		state: &AppState
	) {

		let (texture,texture_size) = self.build(&state.device);

		let vertices = self.to_vertices(texture_size.width as f32,texture_size.height as f32);

		let vertex_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
			label: Some("Vertex buffer"),
			contents: bytemuck::cast_slice(&vertices), // TODO maybe remove bytemuck
			usage: wgpu::BufferUsages::VERTEX,
		});

		let texture_view = texture.create_view(&Default::default());
		let texture_sampler = state.device.create_sampler(
			&wgpu::SamplerDescriptor { 
				label: Some("Texture sampler"), 
				address_mode_u: wgpu::AddressMode::ClampToEdge, 
				address_mode_v: wgpu::AddressMode::ClampToEdge, 
				address_mode_w: wgpu::AddressMode::ClampToEdge, 
				mag_filter: wgpu::FilterMode::Linear, 
				min_filter: wgpu::FilterMode::Nearest, 
				mipmap_filter: wgpu::FilterMode::Nearest, 
				..Default::default()
			}
		);

		let texture_bind_group = state.device.create_bind_group(
			&wgpu::BindGroupDescriptor { 
				label: Some("Text bind group"), 
				layout:&context.text_renderer.texture_bind_group_layout, 
				entries: &[
					wgpu::BindGroupEntry{
						binding:0,
						resource:wgpu::BindingResource::TextureView(&texture_view)
					},
					wgpu::BindGroupEntry{
						binding:1,
						resource:wgpu::BindingResource::Sampler(&texture_sampler)
					}
				]
			}
		);

		state.queue.write_texture(
			wgpu::ImageCopyTextureBase { 
				texture: &texture, 
				mip_level: 0, 
				origin: wgpu::Origin3d::ZERO, 
				aspect: wgpu::TextureAspect::All
			},
			&self.img, 
			wgpu::ImageDataLayout { 
				offset: 0, 
				bytes_per_row: Some(4 * self.size.width as u32), 
				rows_per_image: Some(self.size.height as u32)
			}, 
			texture_size
		);

		// Set the render pipeline and vertex buffer
		render_pass.set_pipeline(&context.text_renderer.render_pipeline);
		render_pass.set_bind_group(0, &context.rect_renderer.window_bind_group, &[]);
		render_pass.set_bind_group(1, &texture_bind_group, &[]);
		render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

		render_pass.draw(0..vertices.len() as u32, 0..1);
	}

	fn size(&mut self,width:f32,height:f32) {
		self.size.width = width;
		self.size.height = height;
	}
	
	fn get_size(&self) -> Size {
		self.size
	}

	fn get_bounds(&self) -> Bounds {
		Bounds{
			x:[self.position.x,self.position.x + self.size.width],
			y:[self.position.y,self.position.y + self.size.height]
		}
	}

	fn width(&mut self, width:f32) {
		self.size.width = width
	}

	fn height(&mut self, height:f32) {
		self.size.height = height
	}

	fn position(&mut self, x:f32,y:f32) {
		self.position.x = x;
		self.position.y = y;
	}

	fn get_position(&self) -> Position {
		self.position
	}
}
