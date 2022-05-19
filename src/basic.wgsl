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

// Ray
struct Ray {
    origin: vec3<f32>;
    direction: vec3<f32>;
};

fn ray_at(ray: ptr<function,Ray>, t: f32) -> vec3<f32> {
    return (*ray).origin + (*ray).direction * t;
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
fn hit_sphere(center: ptr<function, vec3<f32>>, radius: f32, ray: ptr<function, Ray>) -> f32 {
    var oc = (*ray).origin - *center;
    var a = dot((*ray).direction, (*ray).direction);
    var b = 2.0 * dot(oc, (*ray).direction);
    var c = dot(oc, oc) - radius * radius;
    var discriminant = b * b - 4.0 * a * c;
    if (discriminant < 0.0) {
        return -1.0;
    } else {
        return (-b - sqrt(discriminant)) / (2.0 * a);
    }
}

// Ray trace
fn ray_color(ray: ptr<function, Ray>) -> vec3<f32> {
    var center = vec3<f32>(0.0, 0.0, 1.0);
    var t = hit_sphere(&center, 0.25, ray);
    if (t > 0.0) {
        var n = normalize(ray_at(ray, t) - center);
        return 0.5 * vec3<f32>(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    var norm_dir = normalize((*ray).direction);
    t = norm_dir.y + 1.0;
    return (1.0 - t) * vec3<f32>(1.0, 1.0, 1.0) + t * vec3<f32>(0.5, 0.7, 1.0);
}


[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var camera = new_camera();
    var ray = Ray(camera.origin, vec3<f32>(in.tex_coords.x - 0.5, in.tex_coords.y - 0.5, camera.focal_length));
    var color_pixel_color = ray_color(&ray);
    return vec4<f32>(color_pixel_color, 1.0);
}
