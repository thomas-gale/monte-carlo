// ** Vertex shader **

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

// ** Fragment shader **

// Implementing https://raytracing.github.io/books/RayTracingInOneWeekend.html
// Attribution of assitance from https://www.shadertoy.com/view/lssBD7

// Constants
struct Constants {
    infinity: f32;
    epsilon: f32;
    pi: f32;
    pass_samples_per_pixel: i32;
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

fn vec3_refract(uv: vec3<f32>, n: vec3<f32>, etai_over_etat: f32) -> vec3<f32> {
    var cos_theta = min(dot(-uv, n), 1.0);
    var r_out_perp = etai_over_etat * (uv + cos_theta * n);
    var r_out_parellel = -sqrt(abs(1.0 - dot(r_out_perp, r_out_perp))) * n;
    return r_out_perp + r_out_parellel;
}

fn vec3_schlick_reflectance(cosine: f32, ref_idx: f32) -> f32 {
    var r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * pow(1.0 - cosine, 5.0);
}

// Window
struct Window {
    width_pixels: u32;
    height_pixels: u32;
};

[[group(0), binding(1)]]
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

fn random_in_unit_disk(entropy: u32) -> vec3<f32> {
    var p: vec3<f32>;
    var i = 0u;
    loop {
        p = vec3<f32>(random_float_range(hash(entropy + 2u * i), -1.0, 1.0), random_float_range(hash(entropy + 2u * i + 1u), -1.0, 1.0), 0.0);
        i = i + 1u;
        if (dot(p, p) < 1.0) {
            break;
        }
    }
    return p;
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
    u: vec3<f32>;
    v: vec3<f32>;
    w: vec3<f32>;
    lens_radius: f32;
};

[[group(1), binding(0)]]
var<uniform> camera: Camera;

// Scene
struct Aabb {
    min: vec3<f32>;
    max: vec3<f32>;
};

struct BvhNode {
    left_hittable: u32;  // Pointer to left hittable
    right_hittable: u32; // Pointer to right hittable
    aabb: Aabb;
};

struct Sphere {
    center: vec3<f32>;
    radius: f32;
    material_type: u32; // 0 = lambertian, 1 = metal, 2 = dielectric
    fuzz: f32; // Roughness for metals
    refraction_index: f32; // Refraction index for dielectrics
    albedo: vec3<f32>; // Ray bounce color
};

/// Experimental data structure to hold all bvh compatible data for a single hittable geometry to compose into the bvh tree
struct Hittable {
    /// 0 = BvhNode, 1 = Sphere
    geometry_type: u32;
    bvh_node: BvhNode;
    sphere: Sphere;
};

// Releated to Hittable
let bvh_node_null_ptr: u32 = 4294967295u;

struct SceneBvh {
    hittables: array<Hittable>;
};

[[group(2), binding(0)]]
var<storage, read> scene_bvh: SceneBvh;

// Ray
struct Ray {
    origin: vec3<f32>;
    direction: vec3<f32>;
};

fn ray_at(ray: ptr<function,Ray>, t: f32) -> vec3<f32> {
    return (*ray).origin + (*ray).direction * t;
}

// Bvh helpers

// Optimised method from Andrew Kensler at Pixar.
fn aabb_hit(aabb: ptr<function, Aabb>, ray: ptr<function, Ray>, t_min: f32, t_max: f32) -> bool {
    for (var a = 0; a < 3; a = a + 1) {
        var inv_d = 1.0 / (*ray).direction[a];
        var t_0 = ((*aabb).min[a] - (*ray).origin[a]) * inv_d;
        var t_1 = ((*aabb).max[a] - (*ray).origin[a]) * inv_d;
        if (inv_d < 0.0) {
            var tmp = t_0;
            t_0 = t_1;
            t_1 = tmp;
        }
        var t_min_test = t_min;
        if (t_0 > t_min) {
            t_min_test = t_0;
        }
        var t_max_test = t_max;
        if (t_1 < t_max) {
            t_max_test = t_1;
        }
        if (t_max_test <= t_min_test) {
            return false;
        }
    }
    return true;
}

// Hittable
struct HitRecord {
    p: vec3<f32>;
    normal: vec3<f32>;
    t: f32;
    front_face: bool;

    material_type: u32; // 0 = lambertian, 1 = metal, 2 = dielectric
    albedo: vec3<f32>; // Ray bounce coloring
    fuzz: f32; // Roughness for metals
    refraction_index: f32; // Refraction index for dielectrics
};

fn new_hit_record() -> HitRecord {
    return HitRecord(
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(0.0, 0.0, 0.0),
        0.0,
        false,
        0u,
        vec3<f32>(0.0, 0.0, 0.0),
        0.0,
        0.0,
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
    var sphere = scene_bvh.hittables[sphere_worlds_index].sphere; // WIP: Hard coded to only work on spheres

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
    (*hit_record).fuzz = sphere.fuzz;
    (*hit_record).refraction_index = sphere.refraction_index;

    return true;
} 

fn scene_hits(ray: ptr<function, Ray>, t_min: f32, t_max: f32, rec: ptr<function, HitRecord>) -> bool {
    var hit_anything = false;
    var closest_so_far = t_max;

    // Precondition, return early if scene is empty
    if (arrayLength(&scene_bvh.hittables) == 0u) {
        return hit_anything;
    }

    // WIP Now refactor to bvh

    // Use a basic stack data structure from a fixed array (the stack value is the index of the scene hittable)
    // Max depth is 64 (TODO - add error if exceeded)
    var stack: array<i32, 64>;
    var stack_top = 0;

    // Push the root node index onto the stack (which is the first value in the scene_bvh array)
    stack[stack_top] = 0;
    // stack_top = stack_top + 1;

    // for (;stack_top >= 0;) {
    //     // Check the type of this entity
    //     switch scene_bvh.hittables[stack[stack_top]].geometry_type {
    //         case 0: {
    //             // Bvh
    //             // DFS into the left and right children, if they exist
    //             if (scene_bh)

    //         }
    //         case 1: {
    //             // Sphere
    //         }
    //         default {
    //             // Error
    //         }
    //     } 
    // }

    // OLD - flat loop over all entities in scene and assume are all spheres
    var num_spheres_world = i32(arrayLength(&scene_bvh.hittables));
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
fn camera_get_ray(s: f32, t: f32, entropy: u32) -> Ray {
    var rd = camera.lens_radius * random_in_unit_disk(entropy);
    var offset = camera.u * rd.x + camera.v * rd.y;
    return Ray(camera.origin + offset, camera.lower_left_corner + s * camera.horizontal + t * camera.vertical - camera.origin - offset);
}

// This is a loop version of the recursive reference implmentation.
fn ray_color(ray: ptr<function, Ray>, depth: i32, entropy: u32) -> vec3<f32> {
    var hit_record = new_hit_record();
    var current_ray = Ray((*ray).origin, (*ray).direction);
    var current_ray_color = vec3<f32>(1.0, 1.0, 1.0);
    for (var i = 0; i < depth; i = i + 1) {
        // Check if we hit anything
        if (scene_hits(&current_ray, 0.001, constants.infinity, &hit_record)) {
            if (hit_record.material_type == 0u) {
                // Lambertian material
                var scattered = hit_record.p + random_in_hemisphere(hit_record.normal, (entropy * u32(i + 1)));

                // Check for degenerate target scatter
                if (vec3_near_zero(scattered)) {
                    scattered = hit_record.normal;
                }

                current_ray = Ray(hit_record.p, scattered - hit_record.p);
                current_ray_color = current_ray_color * hit_record.albedo;
            } else if (hit_record.material_type == 1u) {
                // Metallic material
                var reflected = vec3_reflect(normalize(current_ray.direction), hit_record.normal);
                var scattered = Ray(hit_record.p, reflected + hit_record.fuzz * random_in_unit_sphere(entropy * u32(i + 2)));

                if (dot(scattered.direction, hit_record.normal) > 0.0) {
                    current_ray = scattered;
                    current_ray_color = current_ray_color * hit_record.albedo;
                } else {
                    current_ray_color = vec3<f32>(0.0, 0.0, 0.0);
                    break;
                }
            } else if (hit_record.material_type == 2u) {
                // Dielectric material
                var refraction_ratio = 0.0;
                if (hit_record.front_face) {
                    refraction_ratio = 1.0 / hit_record.refraction_index;
                } else {
                    refraction_ratio = hit_record.refraction_index;
                }

                var unit_direction = normalize(current_ray.direction);
                var cos_theta = min(dot(-unit_direction, hit_record.normal), 1.0);
                var sin_theta = sqrt(1.0 - cos_theta * cos_theta);

                var cannot_refract = refraction_ratio * sin_theta > 1.0;
                var direction = vec3<f32>(0.0);
                if (cannot_refract || vec3_schlick_reflectance(cos_theta, refraction_ratio) > random_float(entropy * u32(i + 3))) {
                    direction = vec3_reflect(unit_direction, hit_record.normal);
                } else {
                    direction = vec3_refract(unit_direction, hit_record.normal, refraction_ratio);
                }

                var scattered = Ray(hit_record.p, direction);
                current_ray = scattered;
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

// Result storage texture  
[[group(3), binding(0)]]
var texture: texture_storage_2d<rgba32float, read_write>;

// Result uniforms  
struct ResultUniforms {
    pass_index: u32;
};

[[group(3), binding(1)]]
var<uniform> result_uniforms: ResultUniforms;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // Compute a new sampled color
    var new_sampled_pixel_color = vec3<f32>(0.0, 0.0, 0.0);
    var num_samples = constants.pass_samples_per_pixel;
    for (var s = 0; s < num_samples; s = s + 1) {
        var pixel_entropy = hash(entropy_window_space(in.tex_coords) + result_uniforms.pass_index);
        var pixel_sample_entropy = hash(pixel_entropy * u32(s + 1));
        var u = in.tex_coords.x + random_float(hash(pixel_sample_entropy + 1u)) / f32(window.width_pixels);
        var v = in.tex_coords.y + random_float(hash(pixel_sample_entropy + 2u)) / f32(window.height_pixels);
        var ray = camera_get_ray(u, v, hash(pixel_sample_entropy + 3u));
        new_sampled_pixel_color = new_sampled_pixel_color + ray_color(&ray, constants.max_depth, hash(pixel_sample_entropy + 4u));
    }
    new_sampled_pixel_color = new_sampled_pixel_color / f32(num_samples);
    var new_pixel_color_with_alpha = vec4<f32>(new_sampled_pixel_color, 1.0);

    // Weighted average with existing pixel color in result storage texture.
    var texture_coords = vec2<i32>(i32(in.tex_coords.x * f32(window.width_pixels)), i32(in.tex_coords.y * f32(window.height_pixels)));
    var existing_pixel_color_with_alpha = textureLoad(texture, texture_coords);
    var averaged_pixel_color_with_alpha = (1.0 / (1.0 + f32(result_uniforms.pass_index))) * new_pixel_color_with_alpha + (f32(result_uniforms.pass_index) / (1.0 + f32(result_uniforms.pass_index)) * existing_pixel_color_with_alpha);
    textureStore(texture, texture_coords, averaged_pixel_color_with_alpha);
    return averaged_pixel_color_with_alpha;
}
