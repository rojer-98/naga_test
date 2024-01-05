struct MeshX_naga_oil_mod_XNVSXG2C7OR4XAZLTX {
    model: mat4x4<f32>,
    inverse_transpose_model: mat4x4<f32>,
    flags: u32,
}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
}

@group(2) @binding(0) 
var<uniform> mesh: MeshX_naga_oil_mod_XNVSXG2C7OR4XAZLTX;

@vertex 
fn vertex(vertex_1: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let _e1: VertexOutput = out;
    return _e1;
}

@fragment 
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
}
