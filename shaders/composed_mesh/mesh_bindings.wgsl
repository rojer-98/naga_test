#define_import_path mesh_bindings

#import mesh_types as Types

@group(2) @binding(0)
var<uniform> mesh: Types::Mesh;