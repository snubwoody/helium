// Window attributes
@group(0) @binding(0) var<uniform> window_size: vec2<f32>;

// Widget attributes
@group(1) @binding(0) var<uniform> corner_radius: f32;
@group(1) @binding(1) var<uniform> size: vec2<f32>;
@group(1) @binding(2) var<uniform> position: vec2<f32>;

struct VertexOutput{
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>,
}

struct VertexInput{
	@location(0) position: vec2<f32>,
	@location(1) color:vec4<f32>,
	@location(2) uv:vec2<f32>
}

// Convert screen space coordinates to normalised device coordinates
fn screen_to_ndc(in:vec2<f32>) -> vec2<f32>{
	return vec2<f32>(
		(in.x / window_size.x) * 2.0 - 1.0, // Scale by 2 and translate by -1
		-((in.y / window_size.y) * 2.0 - 1.0),
	);
}

fn bezier_curve(t:f32,p1:vec2<f32>,p2:vec2<f32>) -> vec2<f32>{
	return (1-t) * p1 + t * p2;
}

@vertex
fn vs_main(in:VertexInput) -> VertexOutput {
	var out: VertexOutput;
	
	// Normalize the coordinates
	var coords =  screen_to_ndc(in.position);
	
	out.position = vec4<f32>(coords,1.0,1.0);
	out.color = in.color;
	return out;
}

@fragment
fn fs_main(in:VertexOutput) -> @location(0) vec4<f32> {

	return vec4(in.color);
}


