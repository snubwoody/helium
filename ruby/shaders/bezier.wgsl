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

// b.x = width
// b.y = height
// r.x = roundness top-right  
// r.y = roundness bottom-right
// r.z = roundness top-left
// r.w = roundness bottom-left
fn sd_rounded_box( point:vec2<f32>,bounds:vec2<f32>, radius:vec4<f32> ) -> f32 {
	var r = radius;
	if point.x < 0.0 {
		r.x  = r.z;
		r.y  = r.w;
	}
	if point.y < 0.0 {
		r.x = r.y;
	}
    let q = abs(point)-bounds+r.x;
    return min(max(q.x,q.y),0.0) + length(max(q,vec2(0.0))) - r.x;
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
	let center = (position + (size * 0.5)); 
	let p = in.position.xy - center;

	let d = sd_rounded_box(
		p,
		size/2,
		vec4(corner_radius)
	);// Might need to clamp radius
	
	return vec4(in.color.xyz,-d * in.color.w);
}


