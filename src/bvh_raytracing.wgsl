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
    /// Maximum depth of bounced ray.
    max_depth: i32;
    /// Number of vertical subdivision for single frame passes.
    vertical_render_slices: i32;
    /// 0: Off, 1: On
    draw_vertical_render_slice_region: u32;
    /// 0: Off, 1: On
    draw_bvh: u32;
    /// Fraction of light attenuated by each bvh traversed - bit hacky (larger scenes will need values like 0.999 and small scenes 0.9)
    draw_bvh_attenuation: f32;
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
struct Material {
    /// 0: lambertian, 1: metal, 2: dielectric, 3: emissive
    material_type: u32; 
    /// Roughness for metals
    fuzz: f32; 
    /// Refraction index for dielectrics
    refraction_index: f32; 
    /// Ray bounce color
    albedo: vec3<f32>;
};

struct Sphere {
    center: vec3<f32>;
    radius: f32;
    /// Reference to the material index in the scene materials
    material_index: u32; 
    /// TODO - about to be refactored to material index, 0 = lambertian, 1 = metal, 2 = dielectric
    // material_type: u32; 
    // fuzz: f32; // Roughness for metals
    // refraction_index: f32; // Refraction index for dielectrics
    // albedo: vec3<f32>; // Ray bounce color
};

struct Cuboid {
    /// Centroid of the cuboid (Is this needed? or will the txx/txi suffice)
    // center: vec3<f32>;
    /// Axis aligned 'radius' (half edge length) of the cuboid
    radius: vec3<f32>;
    /// Reference to the material index in the scene materials
    material_index: u32; 
    /// World to object space transform
    txx: mat4x4<f32>;
    /// Object to world space transform
    txi: mat4x4<f32>;
};

/// Axis aligned bounding box.
struct Aabb {
    min: vec3<f32>;
    max: vec3<f32>;
};

struct BvhNode {
    /// Pointer to left hittable
    left_hittable: u32;  
    /// Pointer to right hittable
    right_hittable: u32; 
    /// Aabb pre-computed in rust before sending accross buffer.
    aabb: Aabb;
};

/// Experimental data structure to hold all bvh compatible data for a single hittable geometry to compose into the bvh tree
struct LinearHittable {
    /// 0: BvhNode, 1: Sphere, 2: Cuboid
    geometry_type: u32;
    /// Given the geometry type, the actual data is stored at the following index in the linear_scene_bvh vector (for the appropriate type).
    scene_index: u32;
    // TODO - These two below are about to be refactored into indexes of the primitive type in the LinearScene
    // bvh_node: BvhNode;
    // sphere: Sphere;
};

// Releated to Hittable
let bvh_node_null_ptr: u32 = 4294967295u;

// Scene Linear Arrays
struct SceneLinearMaterials {
    vals: array<Material>;
};

struct SceneLinearHittables {
    vals: array<LinearHittable>;
};

struct SceneLinearBvhNodes {
    vals: array<BvhNode>;
};

struct SceneLinearSpheres {
    vals: array<Sphere>;
};

struct SceneLinearCuboids {
    vals: array<Cuboid>;
};

[[group(2), binding(0)]]
var<storage, read> scene_materials: SceneLinearMaterials;

[[group(2), binding(1)]]
var<storage, read> scene_hittables: SceneLinearHittables;

[[group(2), binding(2)]]
var<storage, read> scene_bvh_nodes: SceneLinearBvhNodes;

[[group(2), binding(3)]]
var<storage, read> scene_spheres: SceneLinearSpheres;

[[group(2), binding(4)]]
var<storage, read> scene_cuboids: SceneLinearCuboids;

// Ray
struct Ray {
    origin: vec3<f32>;
    direction: vec3<f32>;
};

fn ray_at(ray: ptr<function,Ray>, t: f32) -> vec3<f32> {
    return (*ray).origin + (*ray).direction * t;
}

// Bvh helpers

// Attribution: https://gamedev.stackexchange.com/a/18459
// t is length of ray until intersection
fn aabb_hit(hittables_bvh_node_index: u32, ray: ptr<function, Ray>, t: ptr<function, f32>) -> bool {
    var aabb = scene_bvh_nodes.vals[ scene_hittables.vals[hittables_bvh_node_index].scene_index ].aabb;

    var dir_frac = vec3<f32>(1.0 / (*ray).direction.x, 1.0 / (*ray).direction.y, 1.0 / (*ray).direction.z);
    var t_1 = (aabb.min.x - (*ray).origin.x) * dir_frac.x;
    var t_2 = (aabb.max.x - (*ray).origin.x) * dir_frac.x;
    var t_3 = (aabb.min.y - (*ray).origin.y) * dir_frac.y;
    var t_4 = (aabb.max.y - (*ray).origin.y) * dir_frac.y;
    var t_5 = (aabb.min.z - (*ray).origin.z) * dir_frac.z;
    var t_6 = (aabb.max.z - (*ray).origin.z) * dir_frac.z;

    var t_min = max(max((min(t_1, t_2)), (min(t_3, t_4))), (min(t_5, t_6)));
    var t_max = min(min((max(t_1, t_2)), (max(t_3, t_4))), (max(t_5, t_6)));

    // If tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us.
    if (t_max < 0.0) {
        (*t) = t_max;
        return false;
    }

    // If tmin > tmax, ray doesn't intersect AABB.
    if (t_min > t_max) {
        (*t) = t_max;
        return false;
    }

    (*t) = t_min;
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

    number_bvh_hits: u32; // Track the number of bvh hits this ray has made
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
        0u,
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
fn sphere_hit(hittables_sphere_index: u32, ray: ptr<function, Ray>, t_min: f32, t_max: f32, hit_record: ptr<function, HitRecord>) -> bool {
    var sphere = scene_spheres.vals[ scene_hittables.vals[hittables_sphere_index].scene_index ];
    var material = scene_materials.vals[ sphere.material_index ];

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
    (*hit_record).material_type = material.material_type;
    (*hit_record).albedo = material.albedo;
    (*hit_record).fuzz = material.fuzz;
    (*hit_record).refraction_index = material.refraction_index;

    return true;
} 

/// Attribution: https://iquilezles.org/articles/boxfunctions/
fn cuboid_hit(hittables_cuboid_index: u32, ray: ptr<function, Ray>, t_min: f32, t_max: f32, hit_record: ptr<function, HitRecord>) -> bool {
    var cuboid = scene_cuboids.vals[ scene_hittables.vals[hittables_cuboid_index].scene_index ];
    var material = scene_materials.vals[ cuboid.material_index ];

    // convert from world to box space
    var rd = (cuboid.txx * vec4<f32>((*ray).direction, 0.0)).xyz;
    var ro = (cuboid.txx * vec4<f32>((*ray).origin, 1.0)).xyz;

    // ray-box intersection in box space
    var m = 1.0 / rd;

    var s_x = -1.0;
    if (rd.x < 0.0) {
        s_x = 1.0;
    }
    var s_y = -1.0;
    if (rd.y < 0.0) {
        s_y = 1.0;
    }
    var s_z = -1.0;
    if (rd.z < 0.0) {
        s_z = 1.0;
    }
    var s = vec3<f32>(s_x, s_y, s_z);

    var t1 = m * (-ro + s * cuboid.radius);
    var t2 = m * (-ro - s * cuboid.radius);

    var tN = max(max(t1.x, t1.y), t1.z);
    var tF = min(min(t2.x, t2.y), t2.z);

    // check hit is in allowed range 
    if (tN > t_max || tF < t_min) {
        return false;
    }

    // check for hit with cuboid
    if (tN > tF || tF < 0.0) {
        return false;
    }

    // compute normal (in world space)
    if (t1.x > t1.y && t1.x > t1.z) {
        (*hit_record).normal = cuboid.txi[0].xyz * s.x * 1.0;
    } else if (t1.y > t1.z) {
        (*hit_record).normal = cuboid.txi[1].xyz * s.y * 1.0;
    } else {
        (*hit_record).normal = cuboid.txi[2].xyz * s.z * 1.0;
    }

    // intersection point (in world space)
    (*hit_record).p = (cuboid.txi * vec4<f32>((ro + (rd * tN)), 1.0)).xyz;

    // distance to intersection point (in world space)
    (*hit_record).t = tN;

    // material data
    (*hit_record).material_type = material.material_type;
    (*hit_record).albedo = material.albedo;
    (*hit_record).fuzz = material.fuzz;
    (*hit_record).refraction_index = material.refraction_index;

    return true;
}

fn scene_hits(ray: ptr<function, Ray>, t_min: f32, t_max: f32, rec: ptr<function, HitRecord>) -> bool {
    var hit_anything = false;
    var closest_so_far = t_max;

    // Precondition, return early if scene is empty
    if (arrayLength(&scene_hittables.vals) == 0u) {
        return hit_anything;
    }

    // Use a basic stack data structure from a fixed array (the stack value is the index of the scene hittable)
    // Max depth is 32 (which means the scene can contain maximum of approximatly 2^32 hittables)
    var stack: array<u32, 32>;

    // Track the top of the stack
    var stack_top = 0;

    // Push the root node index onto the stack (which is the first value in the scene array)
    stack[stack_top] = 0u;

    // While the stack is not empty
    for (;stack_top >= 0;) {
        // Check for stack depth exceeded
        if (stack_top >= 32) {
            return false; // TODO - add better error signal
        }

        // Get hittable from top of stack 
        var current_hittable = scene_hittables.vals[ stack[stack_top] ];
        // Check the type of this hittable
        switch (current_hittable.geometry_type) {
            case 0u: {
                // Bvh
                var bvh = scene_bvh_nodes.vals[ current_hittable.scene_index ];

                // Does this BVH node intersect the ray?
                var t = 0.0;
                // var hit = aabb_hit(stack[stack_top], ray, t_min, closest_so_far, &t);
                var hit = aabb_hit(stack[stack_top], ray, &t);

                // Pop the stack (aabb hit check done).
                stack_top = stack_top - 1;

                if (hit) {
                    // Track the number of bvh hits for bvh debug rendering purposes
                    (*rec).number_bvh_hits = (*rec).number_bvh_hits + 1u;

                    // Push the left and right children onto the stack (if they exist)
                    if (bvh.left_hittable != bvh_node_null_ptr) {
                        stack_top = stack_top + 1;
                        stack[stack_top] = bvh.left_hittable;
                    }
                    if (bvh.right_hittable != bvh_node_null_ptr) {
                        stack_top = stack_top + 1;
                        stack[stack_top] = bvh.right_hittable;
                    }
                }
            }
            case 1u: {
                // Sphere
                var hit = sphere_hit(stack[stack_top], ray, t_min, closest_so_far, rec);

                // Pop the stack (sphere hit check done).
                stack_top = stack_top - 1;

                if (hit) {
                    hit_anything = true;
                    closest_so_far = (*rec).t;
                }
            }
            case 2u: {
                // Cuboid
                var hit = cuboid_hit(stack[stack_top], ray, t_min, closest_so_far, rec);

                // Pop the stack (cuboid hit check done).
                stack_top = stack_top - 1;

                if (hit) {
                    hit_anything = true;
                    closest_so_far = (*rec).t;
                }
            }
            default: {
                // Error
                return false;
            }
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
    var number_bvh_hits_first_bounce = 0u;
    for (var i = 0; i < depth; i = i + 1) {
        // Check if we hit anything
        var hit = scene_hits(&current_ray, 0.001, constants.infinity, &hit_record);

        // For rendering the bvh (only care about number of bvh intersections on before first bounce)
        if (i == 0) {
            number_bvh_hits_first_bounce = hit_record.number_bvh_hits;
        }

        if (hit) {
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
            } else if (hit_record.material_type == 3u) {
                // Emmisive material
                current_ray_color = current_ray_color * hit_record.albedo;
                break;
            }
        } else {
            // No hit, return background / sky color gradient
            // var unit_direction = normalize(current_ray.direction);
            // var t = 0.5 * (unit_direction.y + 1.0);
            // current_ray_color = current_ray_color * ((1.0 - t) * vec3<f32>(1.0, 1.0, 1.0) + t * vec3<f32>(0.5, 0.7, 1.0));
            current_ray_color = current_ray_color * vec3<f32>(0.0, 0.0, 0.0);
            break;
        }
    }

    // Optional bvh rendering - darken the ray by the number of bvh hits
    if (constants.draw_bvh == 1u && number_bvh_hits_first_bounce > 0u) {
        current_ray_color = current_ray_color * pow(vec3<f32>(constants.draw_bvh_attenuation), vec3<f32>(f32(number_bvh_hits_first_bounce)));
    }

    return current_ray_color;
}

// Result storage texture  
[[group(3), binding(0)]]
var texture: texture_storage_2d<rgba32float, read_write>;

// Result uniforms  
struct ResultUniforms {
    pass_index: u32; // TODO - what happens after we reach u32 max number of passes (we would need to leave running for 136 years at 1fps though :D)?
};

[[group(3), binding(1)]]
var<uniform> result_uniforms: ResultUniforms;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // // Read the current sampled colour of the pixel from the texture
    var texture_coords = vec2<i32>(i32(in.tex_coords.x * f32(window.width_pixels)), i32(in.tex_coords.y * f32(window.height_pixels)));
    var existing_pixel_color_with_alpha = textureLoad(texture, texture_coords);

    // Return early (if we are not in the current vertical render slice region or first pass to prevent a full screen render first frame, which can be very slow for a complex scene)
    if ((result_uniforms.pass_index % u32(constants.vertical_render_slices)) != u32((1.0 - in.tex_coords.y) * f32(constants.vertical_render_slices))) {
        return existing_pixel_color_with_alpha;
    }

    // Calculate the ray for the current pixel
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
    var averaged_pixel_color_with_alpha = (1.0 / (1.0 + f32(result_uniforms.pass_index) / f32(constants.vertical_render_slices))) * new_pixel_color_with_alpha + ((f32(result_uniforms.pass_index) / f32(constants.vertical_render_slices)) / (1.0 + (f32(result_uniforms.pass_index) / f32(constants.vertical_render_slices))) * existing_pixel_color_with_alpha);
    textureStore(texture, texture_coords, averaged_pixel_color_with_alpha);

    // Optionally draw the current vertical render slice region
    if (constants.draw_vertical_render_slice_region == 1u && (result_uniforms.pass_index % u32(constants.vertical_render_slices)) == u32((1.0 - in.tex_coords.y) * f32(constants.vertical_render_slices))) {
        return vec4<f32>(1.0, 0.156, 0.949, 1.0); // Nice pink
    }

    return averaged_pixel_color_with_alpha;
}


