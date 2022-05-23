// Vertex shader

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(in.position, 1.0);
    out.tex_coords = in.tex_coords;
    return out;
}

// Fragment shader

// Implementing https://raytracing.github.io/books/RayTracingInOneWeekend.html
// Attribution to assitance from https://www.shadertoy.com/view/lssBD7

// Constants
struct Constants {
    infinity: f32;
    epsilon: f32;
    pi: f32;
    samples_per_pixel: i32;
    max_depth: i32;
};

[[group(0), binding(0)]]
var<uniform> constants: Constants;

// Utilities
fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * constants.pi / 180.0;
}

fn vec3_near_zero(v: vec3<f32>) -> bool {
    return abs(v.x) < constants.epsilon && abs(v.y) < constants.epsilon && abs(v.z) < constants.epsilon;
}

fn vec3_reflect(v: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    return v - 2.0 * dot(v, n) * n;
}

// Window
struct Window {
    width_pixels: u32;
    height_pixels: u32;
};

[[group(1), binding(0)]]
var<uniform> window: Window;

// Random
// Attribution: https://github.com/bevyengine/bevy/blob/main/assets/shaders/game_of_life.wgsl
fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn entropy_window_space(tex_coords: vec2<f32>) -> u32 {
    return hash(u32(u32(tex_coords.x * f32(window.width_pixels)) + u32(tex_coords.y * f32(window.height_pixels)) * window.width_pixels));
}

fn random_float(entropy: u32) -> f32 {
    return f32(hash(entropy)) / 4294967295.0;
}

fn random_float_range(entropy: u32, min: f32, max: f32) -> f32 {
    return random_float(entropy) * (max - min) + min;
}

fn random_vec3(entropy: u32) -> vec3<f32> {
    return vec3<f32>(random_float(entropy), random_float(hash(entropy + 1u)), random_float(hash(entropy + 2u)));
}

fn random_vec3_range(entropy: u32, min: f32, max: f32) -> vec3<f32> {
    return vec3<f32>(random_float_range(entropy, min, max), random_float_range(hash(entropy + 1u), min, max), random_float_range(hash(entropy + 2u), min, max));
}

fn random_in_unit_sphere(entropy: u32) -> vec3<f32> {
    var p: vec3<f32>;
    var i = 0u;
    loop {
        p = random_vec3_range(hash(entropy + i), -1.0, 1.0);
        i = i + 1u;
        if (dot(p, p) < 1.0) {
            break;
        }
    }
    return p;
}

fn random_in_hemisphere(normal: vec3<f32>, entropy: u32) -> vec3<f32> {
    var in_unit_sphere = random_in_unit_sphere(entropy);
    if (dot(in_unit_sphere, normal) > 0.0) {
        return in_unit_sphere;
    }
    return -in_unit_sphere;
}

fn random_unit_vector(entropy: u32) -> vec3<f32> {
    return normalize(random_in_unit_sphere(entropy));
}

// Camera
struct Camera {
    origin: vec3<f32>;
    lower_left_corner: vec3<f32>;
    horizontal: vec3<f32>;
    vertical: vec3<f32>;
};

[[group(2), binding(0)]]
var<uniform> camera: Camera;

// Scene
struct Sphere {
    center: vec3<f32>;
    radius: f32;
    albedo: vec3<f32>;
    material_type: u32; // 0 = lambertian, 1 = metal, 2 = dielectric
};

struct Scene {
    spheres: array<Sphere>;
};

[[group(3), binding(0)]]
var<storage, read> scene: Scene;

// Ray
struct Ray {
    origin: vec3<f32>;
    direction: vec3<f32>;
};

fn ray_at(ray: ptr<function,Ray>, t: f32) -> vec3<f32> {
    return (*ray).origin + (*ray).direction * t;
}

// Hittable
struct HitRecord {
    p: vec3<f32>;
    normal: vec3<f32>;
    t: f32;
    front_face: bool;

    material_type: u32; // 0 = lambertian, 1 = metal, 2 = dielectric
    albedo: vec3<f32>;
};

fn new_hit_record() -> HitRecord {
    return HitRecord(
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(0.0, 0.0, 0.0),
        0.0,
        false,
        0u,
        vec3<f32>(0.0, 0.0, 0.0),
    );
}

fn set_face_normal(hit_record: ptr<function, HitRecord>, r: ptr<function, Ray>, outward_normal: vec3<f32>) {
    (*hit_record).front_face = dot((*r).direction, outward_normal) < 0.0;
    if ((*hit_record).front_face) {
        (*hit_record).normal = outward_normal;
    } else {
        (*hit_record).normal = -1.0 * outward_normal;
    }
}

// Sphere Helpers
fn sphere_hit(sphere_worlds_index: i32, ray: ptr<function, Ray>, t_min: f32, t_max: f32, hit_record: ptr<function, HitRecord>) -> bool {
    var sphere = scene.spheres[sphere_worlds_index];

    var oc = (*ray).origin - sphere.center;
    var a = dot((*ray).direction, (*ray).direction);
    var half_b = dot(oc, (*ray).direction);
    var c = dot(oc, oc) - sphere.radius * sphere.radius;

    var discriminant = half_b * half_b - a * c;
    if (discriminant < 0.0) {
        return false;
    }
    var sqrtd = sqrt(discriminant);
 
    // Find the nearest root that lies in acceptable range
    var root = (-half_b - sqrtd) / a;
    if (root < t_min || root > t_max) {
        root = (-half_b + sqrtd) / a;
        if (root < t_min || root > t_max) {
            return false;
        }
    }

    (*hit_record).t = root;
    (*hit_record).p = ray_at(ray, (*hit_record).t);
    var outward_normal = ((*hit_record).p - sphere.center) / sphere.radius;
    set_face_normal(hit_record, ray, outward_normal);
    (*hit_record).material_type = sphere.material_type;
    (*hit_record).albedo = sphere.albedo;

    return true;
} 

fn sphere_hits(ray: ptr<function, Ray>, t_min: f32, t_max: f32, rec: ptr<function, HitRecord>) -> bool {

    // Reset hitrecord

    var hit_anything = false;
    var closest_so_far = t_max;

    var num_spheres_world = i32(arrayLength(&scene.spheres));
    for (var i = 0; i < num_spheres_world; i = i + 1) {
        var hit_sphere = sphere_hit(i, ray, t_min, closest_so_far, rec);
        if (hit_sphere) {
            hit_anything = true;
            closest_so_far = (*rec).t;
        }
    }
    return hit_anything;
}

// Ray trace
fn camera_get_ray(u: f32, v: f32) -> Ray {
    return Ray(camera.origin, camera.lower_left_corner + u * camera.horizontal + v * camera.vertical - camera.origin);
}

// This is a loop version of the recursive reference implmentation.
fn ray_color(ray: ptr<function, Ray>, depth: i32, entropy: u32) -> vec3<f32> {
    var hit_record = new_hit_record();
    var current_ray = Ray((*ray).origin, (*ray).direction);
    var current_ray_color = vec3<f32>(1.0, 1.0, 1.0);
    for (var i = 0; i < depth; i = i + 1) {
        // Check if we hit anything
        if (sphere_hits(&current_ray, 0.001, constants.infinity, &hit_record)) {
            if (hit_record.material_type == 0u) {
                    var target = hit_record.p + random_in_hemisphere(hit_record.normal, (entropy * u32(i + 1)));

                    if (vec3_near_zero(target)) {
                        target = hit_record.normal;
                    }

                    current_ray = Ray(hit_record.p, target - hit_record.p);
                    current_ray_color = current_ray_color * hit_record.albedo;
            }
        } else {
            // No hit, return background / sky color
            var unit_direction = normalize(current_ray.direction);
            var t = 0.5 * (unit_direction.y + 1.0);
            current_ray_color = current_ray_color * ((1.0 - t) * vec3<f32>(1.0, 1.0, 1.0) + t * vec3<f32>(0.5, 0.7, 1.0));
            break;
        }
    }
    return current_ray_color;
}

fn color_gamma_correction(color: ptr<function, vec3<f32>>) {
    (*color).r = pow((*color).r, 1.0 / 2.2);
    (*color).g = pow((*color).g, 1.0 / 2.2);
    (*color).b = pow((*color).b, 1.0 / 2.2);
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var pixel_color = vec3<f32>(0.0, 0.0, 0.0);
    var num_samples = constants.samples_per_pixel;
    for (var s = 0; s < num_samples; s = s + 1) {
        // Notes

        // tex_coords x and y are in range [0, 1] (f32)
        // tex_coords bottom left is (0, 0)
        // focal_length is 1.0

        // window.width_pixels and window.height_pixels are in range [0, n] (u32)

        // Camera is currently defined in screen/tex_coords space
        // TODO - decide if to move camera / scene to a world space and how to store that transformation
        // TODO - decide how to use the screen size/aspect ratio to stop output image in window from being stretched

        // Multisampled pixels
        var pixel_entropy = hash(entropy_window_space(in.tex_coords));
        var pixel_sample_entropy = hash(pixel_entropy * u32(s + 1));
        var u = in.tex_coords.x + random_float(hash(pixel_sample_entropy + 1u)) / f32(window.width_pixels);
        var v = in.tex_coords.y + random_float(hash(pixel_sample_entropy + 2u)) / f32(window.height_pixels);
        var ray = camera_get_ray(u, v);
        pixel_color = pixel_color + ray_color(&ray, constants.max_depth, hash(pixel_sample_entropy + 3u));
    }
    pixel_color = pixel_color / f32(num_samples);
    color_gamma_correction(&pixel_color);
    return vec4<f32>(pixel_color, 1.0);
}
