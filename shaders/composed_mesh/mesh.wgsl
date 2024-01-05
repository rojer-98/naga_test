#import mesh_view_bindings
#import mesh_bindings

// NOTE: Bindings must come before functions that use them!
#import mesh_functions as Func
#import mesh_types as Types
#import mesh_vertex_output as Output 

@group(2) @binding(0)
var<uniform> mesh: Types::Mesh;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(5) joint_indices: vec4<u32>,
    @location(6) joint_weights: vec4<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
#ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
    out.world_normal = skin_normals(model, vertex.normal);
#endif
#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
#endif
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

    return out;
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    #import mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
#ifdef VERTEX_COLORS
    return in.color;
#else
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
#endif
}
