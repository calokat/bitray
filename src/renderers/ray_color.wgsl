@group(0) @binding(0)
var tex: texture_storage_2d<rgba8uint, write>;
@group(0) @binding(1)
var ray_tex: texture_storage_3d<rgba32float, read>;

struct Sphere {
    center: vec3f,
    radius: f32,
}

struct Quad {
    q: vec4f,
    u: vec4f,
    v: vec4f,
}

struct ResourceDescriptor {
  kind: u32,
  index: u32,
}

struct Entity {
  mesh: ResourceDescriptor,
}

struct HitResult {
    position: vec3f,
    normal: vec3f,
    color: vec4f,
    root: f32,
    success: bool,
}

struct Ray {
    origin: vec3f,
    direction: vec3f
}

@group(0) @binding(2)
var<storage> spheres: array<Sphere>;

@group(0) @binding(3)
var<storage> quads: array<Quad>;

@group(1) @binding(0)
var<storage> entities: array<Entity>;

const LIGHT_DIR = vec3f(2.0, 1.0, 1.0);

fn fail_hit(hit: HitResult) -> HitResult {
  return HitResult(hit.position, hit.normal, hit.color, hit.root, false);
}

fn hit_sphere(sphere: Sphere, ray: Ray, closest_hit: HitResult) -> HitResult {
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
        return fail_hit(closest_hit);
    }
    var p = ray.origin + ray.direction * root;
    var normal = (sphere_center - p) / sphere_radius;
    return HitResult(p, normal, vec4f(255, 0, 0, 255), root, true);
}

fn hit_quad(quad: Quad, ray: Ray, closest_hit: HitResult) -> HitResult {
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

    var intersection = ray.origin + ray.direction * t;
    var planar_hit = intersection - quad.q.xyz;

    var alpha = dot(quad_w, cross(planar_hit, quad.v.xyz));
    var beta = dot(quad_w, cross(quad.u.xyz, planar_hit));
    if (alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0) {
        return fail_hit(closest_hit);
    }
    return HitResult(intersection, quad_normal, vec4f(0.0, 0.0, 255.0, 255.0), t, true);
}

@compute @workgroup_size(64, 4, 1)
fn compute_main(@builtin(global_invocation_id) param: vec3<u32>) {
    var y = param.y;
    var x = param.x;
    var xy = param.xy;
    var ray_origin = textureLoad(ray_tex, vec3u(xy, 0)).xyz;
    var ray_dir = textureLoad(ray_tex, vec3u(xy, 1)).xyz;
    ray_dir = normalize(ray_dir);
    var closest_hit: HitResult = HitResult(vec3f(0.0, 0.0, 0.0), vec3f(0.0, 0.0, 0.0), vec4f(0.0, 0.0, 0.0, 0.0), 2000000000, false);
    for (var i = 0; i < 10; i = i + 1) {
      for (var i: u32 = 0; i < arrayLength(&entities); i = i + 1) {
          var entity = entities[i];
          if (entity.mesh.kind == 0) {
              closest_hit = hit_sphere(spheres[entity.mesh.index], Ray(ray_origin, ray_dir), closest_hit);
          } else if (entity.mesh.kind == 1) {
              closest_hit = hit_quad(quads[entity.mesh.index], Ray(ray_origin, ray_dir), closest_hit);
          }
      }

      if (!closest_hit.success) {
        break;
      }
      ray_dir = reflect(ray_dir, closest_hit.normal);
      ray_origin = closest_hit.position;
    }
    var outColorF = closest_hit.color;
    var outColor = vec4u(outColorF * dot(closest_hit.normal, normalize(LIGHT_DIR)));
    textureStore(tex, param.xy, vec4u(outColor));
}
