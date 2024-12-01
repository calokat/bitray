@group(0) @binding(0)
var tex: texture_storage_2d<rgba8uint, write>;

@compute @workgroup_size(64, 4, 1)
fn compute_main(@builtin(global_invocation_id) param: vec3<u32>) {
    textureStore(tex, param.xy, vec4u(255, 0, 0, 255));
}
