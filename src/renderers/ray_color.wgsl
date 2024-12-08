@group(0) @binding(0)
var tex: texture_storage_2d<rgba8uint, write>;
@group(0) @binding(1)
var ray_tex: texture_storage_2d<rgba32float, read>;

@compute @workgroup_size(64, 4, 1)
fn compute_main(@builtin(global_invocation_id) param: vec3<u32>) {
    var y = param.y;
    var x = param.x;
    var xy = param.xy;
    var ray_origin = vec3f(0.0, 0.0, 0.0);
    var ray_dir = textureLoad(ray_tex, xy).xyz;
    ray_dir = normalize(ray_dir);

    var sphere_center = vec3(0.0, 0.0, 50.0);

    var oc = ray_origin - sphere_center;// - self.center;
    var a = 1.0;
    var half_b = dot(ray_dir, oc);
    var c = dot(oc, oc) - 100.0;

    var discriminant = half_b * half_b - a * c;

    var sqrtd = sqrt(max(0.0, discriminant));
    var root = (-half_b - sqrtd) / a;
    var p = select(vec3f(0.0, 0.0, 0.0), ray_origin + ray_dir * root, root > 0.0);
    var normal = (sphere_center - p) / 50.0;
    var outColorF = select(vec4f(0, 0, 0, 0), vec4f(255, 0, 0, 255), discriminant >= 0);
    var outColor = vec4u(outColorF * dot(normal, normalize(vec3f(2.0, -1.0, 0.0))));
    textureStore(tex, param.xy, outColor);
}
