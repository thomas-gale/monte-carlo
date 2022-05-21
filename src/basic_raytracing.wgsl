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

// Buffers

// Scene
struct Sphere {
    center: vec3<f32>;
    radius: f32;
};

struct Scene {
    spheres: array<Sphere>;
};

[[group(0), binding(0)]]
var<storage, read> scene: Scene;

// Constants (not very effiecient - move to uniforms)
struct Constants {
    infinity: f32;
    pi: f32;
};

fn new_constants() -> Constants {
    return Constants(
        1.0 / 0.0,
        3.14159265358979323846264338327950288,
    );
}

// Utilities
fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * new_constants().pi / 180.0;
}

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

fn random_float(entropy: u32) -> f32 {
    return f32(hash(entropy)) / 4294967295.0;
}

fn random_float_range(entropy: u32, min: f32, max: f32) -> f32 {
    return random_float(entropy) * (max - min) + min;
}


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
};

fn new_hit_record() -> HitRecord {
    return HitRecord(
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(0.0, 0.0, 0.0),
        0.0,
        false,
    );
}

fn set_face_normal(hit_record: ptr<function, HitRecord>, r: ptr<function, Ray>, outward_normal: vec3<f32>) {
    (*hit_record).front_face = dot((*r).direction, outward_normal) < 0.0;
    if ((*hit_record).front_face) {
        (*hit_record).normal = outward_normal
    } else {
        (*hit_record).normal = -outward_normal
    };
}

// Camera
struct Camera {
    focal_length: f32;
    origin: vec3<f32>;
};

fn new_camera() -> Camera {
    return Camera(
        1.0,
        vec3<f32>(0.0, 0.0, 0.0),
    );
}

// Sphere


// Refactored into hittable_sphere
// fn hit_sphere(center: ptr<function, vec3<f32>>, radius: f32, ray: ptr<function, Ray>) -> f32 {
//     var oc = (*ray).origin - *center;
//     var a = dot((*ray).direction, (*ray).direction);
//     var half_b = dot(oc, (*ray).direction);
//     var c = dot(oc, oc) - radius * radius;
//     var discriminant = half_b * half_b - a * c;
//     if (discriminant < 0.0) {
//         return -1.0;
//     } else {
//         return (-half_b - sqrt(discriminant)) / a;
//     }
// }

// let spheres_world: array<Sphere, 2> = array<Sphere, 2>(
//     Sphere(vec3<f32>(0.5, 0.0, -1.0), 0.5),
//     Sphere(vec3<f32>(0.5, -100.5, -1.0), 100.0)
// );

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

    return true;
} 

fn sphere_hits(ray: ptr<function, Ray>, t_min: f32, t_max: f32, rec: ptr<function, HitRecord>) -> bool {
    var hit_anything = false;
    var closest_so_far = t_max;

    var num_spheres_world = i32(arrayLength(&scene.spheres)); // TODO: Move to buffer/uniform data linked to sphere world
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
fn ray_color(ray: ptr<function, Ray>) -> vec3<f32> {
    var hit_record = new_hit_record();
    if (sphere_hits(ray, 0.0, new_constants().infinity, &hit_record)) {
        return 0.5 * (hit_record.normal + vec3<f32>(1.0, 1.0, 1.0));
    }
    var unit_direction = normalize((*ray).direction);
    var t = unit_direction.y + 1.0; // TODO - why not 0.5 * ?
    return (1.0 - t) * vec3<f32>(1.0, 1.0, 1.0) + t * vec3<f32>(0.5, 0.7, 1.0);
}


[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // Create world - TODO - move to some form of input buffer
    // var spheres = array<Sphere, 2>(
    //     Sphere(vec3<f32>(0.5, 0.0, -1.0), 0.5),
    //     Sphere(vec3<f32>(0.5, -100.5, -1.0), 100)
    // );

    // Create camera - TODO - move to uniform
    var camera = new_camera();
    var ray = Ray(camera.origin, vec3<f32>(in.tex_coords.x - 0.5, in.tex_coords.y - 0.5, camera.focal_length));

    // var temp_rec = new_hit_record();
    var color_pixel_color = ray_color(&ray);
    return vec4<f32>(color_pixel_color, 1.0);
}
