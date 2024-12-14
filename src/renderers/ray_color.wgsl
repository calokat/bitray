@group(0) @binding(0)
var tex: texture_storage_2d<rgba8uint, write>;
@group(0) @binding(1)
var ray_tex: texture_storage_2d<rgba32float, read>;

struct Sphere {
    center: vec3f,
    radius: f32,
}

struct Entity {
    shape: u32,
    color: u32,
}

struct HitResult {
    position: vec3f,
    normal: vec3f,
    color: vec4f,
    root: f32,
}

@group(0) @binding(2)
var<storage> spheres: array<Sphere>;

@group(1) @binding(0)
var<storage> entities: array<Entity>;

@compute @workgroup_size(64, 4, 1)
fn compute_main(@builtin(global_invocation_id) param: vec3<u32>) {
    var y = param.y;
    var x = param.x;
    var xy = param.xy;
    var ray_origin = vec3f(0.0, 0.0, 0.0);
    var ray_dir = textureLoad(ray_tex, xy).xyz;
    ray_dir = normalize(ray_dir);

    var closest_hit: HitResult = HitResult(vec3f(0.0, 0.0, 0.0), vec3f(0.0, 0.0, 0.0), vec4f(0.0, 0.0, 0.0, 0.0), 2000000000);
    for (var i: u32 = 0; i < arrayLength(&spheres); i = i + 1) {
        var entity = entities[i];
        var sphere = spheres[entity.shape];

        var sphere_center = sphere.center;
        var sphere_radius = sphere.radius;

        var oc = ray_origin - sphere_center;
        var a = 1.0;
        var half_b = dot(ray_dir, oc);
        var c = dot(oc, oc) - sphere_radius * sphere_radius;

        var discriminant = half_b * half_b - a * c;
        if (discriminant < 0.0) {
            continue;
        }
        var sqrtd = sqrt(discriminant);
        var root = (-half_b - sqrtd) / a;
        if (root >= closest_hit.root) {
            continue;
        }
        var p = ray_origin + ray_dir * root;
        var normal = (sphere_center - p) / sphere_radius;
        closest_hit = HitResult(p, normal, vec4f(255, 0, 0, 255), root);
    }

    var outColorF = closest_hit.color;
    var outColor = vec4u(outColorF * dot(closest_hit.normal, normalize(vec3f(2.0, -1.0, 0.0))));
    textureStore(tex, param.xy, vec4u(outColor));
}
