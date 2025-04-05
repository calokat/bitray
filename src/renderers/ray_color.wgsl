@group(0) @binding(0)
var tex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1)
var<storage> rands: array<u32>;

struct Sphere {
    center: vec3f,
    radius: f32,
}

struct Quad {
    q: vec4f,
    u: vec4f,
    v: vec4f,
}

struct Material {
  emissive: u32,
  color_index: u32,
  _padding: vec2f,
}

struct ResourceDescriptor {
  kind: u32,
  index: u32,
}

struct Entity {
  mesh: ResourceDescriptor,
  material_index: u32,
  _padding: u32,
}

struct HitResult {
    position: vec3f,
    normal: vec3f,
    color: vec4f,
    root: f32,
    // 0: Failure,
    // 1: Scatter,
    // 2: emissive
    success: u32,
}

struct Ray {
    origin: vec3f,
    direction: vec3f
}

struct Camera {
    pixel00_loc: vec4f,
    pixel_delta_u: vec4f,
    pixel_delta_v: vec4f,
    center: vec4f,
}

@group(0) @binding(2)
var<storage> spheres: array<Sphere>;

@group(0) @binding(3)
var<storage> quads: array<Quad>;

@group(0) @binding(4)
var<storage> materials: array<Material>;

@group(0) @binding(5)
var<storage> colors: array<vec4f>;

@group(0) @binding(6)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<storage> entities: array<Entity>;

const LIGHT_DIR = vec3f(0.0, 0.0, 1.0);

fn fail_hit(hit: HitResult) -> HitResult {
  return HitResult(hit.position, hit.normal, hit.color, hit.root, 0);
}

fn hit_sphere(entity: Entity, sphere: Sphere, ray: Ray, closest_hit: HitResult) -> HitResult {
    var sphere_center = sphere.center;
    var sphere_radius = sphere.radius;

    var oc = ray.origin - sphere_center;
    var a = 1.0;
    var half_b = dot(ray.direction, oc);
    var c = dot(oc, oc) - sphere_radius * sphere_radius;

    var discriminant = half_b * half_b - a * c;
    if (discriminant < 0.0) {
        return fail_hit(closest_hit);
    }
    var sqrtd = sqrt(discriminant);
    var root = (-half_b - sqrtd) / a;
    if (root >= closest_hit.root) {
        return closest_hit;
    }
    if (root < 0) {
        return fail_hit(closest_hit);
    }
    var p = ray.origin + ray.direction * root;
    var normal = (p - sphere_center) / sphere_radius;
    var new_hit = HitResult(p, normal, closest_hit.color, root, select(1u, 2u, materials[entity.material_index].emissive == 1u));
    new_hit.color = material_hit(entity, new_hit);
    return new_hit;
}
// select(1u, 2u, materials[entity.material_index].emissive == 1u)
fn hit_quad(entity: Entity, quad: Quad, ray: Ray, closest_hit: HitResult) -> HitResult {
    var quad_n: vec3f = cross(quad.u.xyz, quad.v.xyz);
    var quad_w = quad_n / dot(quad_n, quad_n);
    var quad_normal = normalize(quad_n);
    var denom = dot(quad_normal, ray.direction);
    if (denom < 0.00000001) {
        return fail_hit(closest_hit);
    }

    var d = dot(quad_normal, quad.q.xyz);
    var t = (d - dot(quad_normal, ray.origin)) / denom;
    if (t < 0.0) {
        return fail_hit(closest_hit);
    }

    if (t >= closest_hit.root) {
        return closest_hit;
    }

    var intersection = ray.origin + ray.direction * t;
    var planar_hit = intersection - quad.q.xyz;

    var alpha = dot(quad_w, cross(planar_hit, quad.v.xyz));
    var beta = dot(quad_w, cross(quad.u.xyz, planar_hit));
    if (alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0) {
        return fail_hit(closest_hit);
    }
    var new_hit = HitResult(intersection, quad_normal, closest_hit.color, t, select(1u, 2u, materials[entity.material_index].emissive == 1));
    new_hit.color = material_hit(entity, new_hit);
    return new_hit;
}

fn material_hit(entity: Entity, hit: HitResult) -> vec4f {
  return colors[materials[entity.material_index].color_index];
}

fn seed_rng(seed: u32) -> array<u32, 5> {
    var z1: u32 = 2;
    var z2: u32 = 8;
    var z3: u32 = 16;
    var z4: u32 = 128;

	z1 = (z1 * (seed + 1));
	z2 = (z2 * (seed + 1));
	z3 = (z3 * (seed + 1));
	z4 = (z4 * (seed + 1));

    z1 = select(z1 + 1, z1, z1 > 1);
	z2 = select(z2 + 7, z2, z2 > 7);
	z3 = select(z3 + 15, z3, z3 > 15);
	z4 = select(z4 + 127, z4, z4 > 127);

    return array(z1, z2, z3, z4, 1);
}

fn rng(state: array<u32, 5>) -> array<u32, 5> {
    var z1 = state[0];
    var z2 = state[1];
    var z3 = state[2];
    var z4 = state[3];
    var b = state[4];

	b = (((z1 << 6) ^ z1) >> 13);
	z1 = (((z1 & 4294967294) << 18) ^ b);

	b = (((z2 << 2) ^ z2) >> 27);
	z2 = (((z2 & 4294967288) << 2) ^ b);

	b = (((z3 << 13) ^ z3) >> 21);
	z3 = (((z3 & 4294967280) << 7) ^ b);

	b = (((z4 << 3) ^ z4) >> 12);
	z4 = (((z4 & 4294967168) << 13) ^ b);

	b = (z1 ^ z2 ^ z3 ^ z4);

    return array(z1, z2, z3, z4, b);
}

fn pixel_sample_square(camera: Camera, px: f32, py: f32) -> vec4f {
    return (camera.pixel_delta_u * px) + (camera.pixel_delta_v * py);
}

fn normal_u32(u: u32) -> f32 {
    return f32(u) / f32(4294967295);
}

@compute @workgroup_size(64, 4, 1)
fn compute_main(@builtin(global_invocation_id) param: vec3<u32>) {
    let iterations = 120;
    let ray_bounces = 5;
    var y = param.y;
    var x = param.x;

    var state = seed_rng(810u);
    var color = vec4f(1.0, 1.0, 1.0, 1.0);
    var closest_hit: HitResult = HitResult(vec3f(0.0, 0.0, 0.0), vec3f(0.0, 0.0, 0.0), vec4f(1.0, 1.0, 1.0, 1.0), 2000000000, 0);

    var pixel_center = (camera.pixel00_loc + (camera.pixel_delta_u * f32(x)) + (camera.pixel_delta_v * f32(y))).xyz; 

    for (var i = 0; i < iterations; i += 1) {
        state = rng(state);
        let px = normal_u32(state[4]) - 0.5;
        state = rng(state);
        let py = normal_u32(state[4]) - 0.5;
        var pixel_sample = pixel_center + pixel_sample_square(camera, px, py).xyz;

        var ray_origin = camera.center.xyz;
        var ray_dir = pixel_sample - ray_origin;
        ray_dir = normalize(ray_dir);

        for (var j = 0; j < ray_bounces; j += 1) {
            var ray_color = vec4f(1.0, 0.0, 0.0, 0.0);
            for (var k = 0u; k < arrayLength(&entities); k += 1u) {
                var entity = entities[i];
                if (entity.mesh.kind == 0) {
                    closest_hit = hit_sphere(entity, spheres[entity.mesh.index], Ray(ray_origin, ray_dir), closest_hit);
                } else if (entity.mesh.kind == 1) {
                    closest_hit = hit_quad(entity, quads[entity.mesh.index], Ray(ray_origin, ray_dir), closest_hit);
                }
            }
            if (closest_hit.success == 0) {
                break;
            }
            if (closest_hit.success == 1) {
                if (j == 0) {
                    color = closest_hit.color;
                } else {
                    color *= closest_hit.color;
                }

                state = rng(state);
                let lambert_x = normal_u32(state[4]);
                state = rng(state);
                let lambert_y = normal_u32(state[4]);
                state = rng(state);
                let lambert_z = normal_u32(state[4]);

                let lambert_vec = normalize(vec3f(lambert_x, lambert_y, lambert_z));
                ray_dir = normalize(lambert_vec + closest_hit.normal);
                
                //ray_dir = reflect(ray_dir, closest_hit.normal);
                ray_origin = closest_hit.position;
            }
            else if (closest_hit.success == 2) {
                break;
            }
        }
    }
    textureStore(tex, param.xy, color);
}
