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

fn ray_color(ray: ptr<function, Ray>) -> vec3<f32> {
    var norm_dir = normalize((*ray).direction);
    var t = norm_dir.y + 1.0;
    return (1.0 - t) * vec3<f32>(1.0, 1.0, 1.0) + t * vec3<f32>(0.5, 0.7, 1.0);
}

// Camera
struct Camera {
    focal_length: f32;
    origin: vec3<f32>;
};

fn new_camera() -> Camera {
    return Camera(
        1.0,
        vec3<f32>(0.5, 0.5, 0.0),
    );
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var camera = new_camera();
    var ray = Ray(camera.origin, vec3<f32>(in.tex_coords, camera.focal_length));
    var color_pixel_color = ray_color(&ray);
    return vec4<f32>(color_pixel_color, 1.0);
}
