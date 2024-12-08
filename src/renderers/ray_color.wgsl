@group(0) @binding(0)
var tex: texture_storage_2d<rgba8uint, write>;
@group(0) @binding(1)
var ray_tex: texture_storage_2d<rgba32float, read>;

@compute @workgroup_size(64, 4, 1)
fn compute_main(@builtin(global_invocation_id) param: vec3<u32>) {
    var y = param.y;
    var x = param.x;
    var xy = param.xy;
    var ray_origin = vec3<f32>(0.0, 0.0, 0.0);
    // var ray_dir = vec3f(0.6464263, 0.36333832, 1.0) + vec3f(-0.0012637855, 0.0, 0.0) * f32(x) + vec3f(-0.0, -0.0012637855, -0.0) * f32(y);
    var ray_dir = textureLoad(ray_tex, xy).xyz;
    ray_dir = normalize(ray_dir);

    var oc = ray_origin - vec3(0.0, 0.0, 50.0);// - self.center;
    var a = 1.0;
    var half_b = dot(ray_dir, oc);
    var c = dot(oc, oc) - 100.0;

    var discriminant = half_b * half_b - a * c;


    var outColor = select(vec4u(0, 0, 0, 0), vec4u(255, 0, 0, 255), discriminant >= 0);

    textureStore(tex, param.xy, outColor);
}
