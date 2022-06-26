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
    /// WoS Tolerance Distance
    wos_tolerance: f32;
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

fn safe_inf_mult(a: f32, b: f32) -> f32 {
    if (a == constants.infinity || b == constants.infinity) {
        return constants.infinity;
    } else {
        return a * b;
    }
}

fn safe_inf_vec3_mult(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        safe_inf_mult(a.x, b.x),
        safe_inf_mult(a.y, b.y),
        safe_inf_mult(a.z, b.z),
    );
}

fn safe_inf_div(a: f32, b: f32) -> f32 {
    if (b == 0.0) {
        return constants.infinity;
    } else {
        return a / b;
    }
}

fn safe_inf_vec3_div(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        safe_inf_div(a.x, b.x),
        safe_inf_div(a.y, b.y),
        safe_inf_div(a.z, b.z),
    );
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
    /// 0: lambertian, 1: metal, 2: dielectric, 3: emissive, 4: isotropic medium, 5, wos albedo blend
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
};

struct Cuboid {
    /// Reference to the material index in the scene materials
    material_index: u32; 
    /// World to object space transform
    txx: mat4x4<f32>;
    /// Object to world space transform
    txi: mat4x4<f32>;
};



struct ConstantMedium {
    /// 0: BvhNode, 1: Sphere, 2: Cuboid, 3: ConstantMedium
    boundary_geometry_type: u32;
    /// Given the geometry type, the actual data is stored at the following index in the linear_scene_bvh vector (for the appropriate type).
    boundary_scene_index: u32;
    /// Index of the material in the linear scene bvh (know as phase function)
    material_index: u32;
    /// Negative inverse of the density of the medium
    neg_inv_density: f32;
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
    /// 0: BvhNode, 1: Sphere, 2: Cuboid, 3: ConstantMedium
    geometry_type: u32;
    /// Given the geometry type, the actual data is stored at the following index in the linear_scene_bvh vector (for the appropriate type).
    scene_index: u32;
};

/// Check if the linear hittables is a primitive
fn is_primitive(geometry_type: u32) -> bool {
    return geometry_type == 1u || geometry_type == 2u;
}

// Releated to Hittable
let bvh_node_null_ptr: u32 = 4294967295u;

// Scene Linear Arrays
struct SceneInteractiveTransform {
    val: mat4x4<f32>;
};

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

struct SceneConstantMediums {
    vals: array<ConstantMedium>;
};

[[group(2), binding(0)]]
var<storage, read> scene_background: Material;

// [[group(2), binding(1)]]
// var<storage, read> scene_interactive_transform: SceneInteractiveTransform;

[[group(2), binding(1)]]
var<storage, read> scene_materials: SceneLinearMaterials;

[[group(2), binding(2)]]
var<storage, read> scene_hittables: SceneLinearHittables;

[[group(2), binding(3)]]
var<storage, read> scene_bvh_nodes: SceneLinearBvhNodes;

[[group(2), binding(4)]]
var<storage, read> scene_spheres: SceneLinearSpheres;

[[group(2), binding(5)]]
var<storage, read> scene_cuboids: SceneLinearCuboids;

[[group(2), binding(6)]]
var<storage, read> scene_constant_mediums: SceneConstantMediums;

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
    t: f32; // ray length until intersection
    front_face: bool;

    /// 0: lambertian, 1: metal, 2: dielectric, 3: emissive, 4: isotropic medium, 5, wos albedo blend
    material_type: u32;
    /// Ray bounce coloring
    albedo: vec3<f32>;
    /// Roughness for metals
    fuzz: f32;
    /// Refraction index for dielectrics
    refraction_index: f32;

    /// Track the number of bvh hits this ray has made
    number_bvh_hits: u32;
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

fn set_material_data(hit_record: ptr<function, HitRecord>, material: ptr<function, Material>) {
    (*hit_record).material_type = (*material).material_type;
    (*hit_record).albedo = (*material).albedo;
    (*hit_record).fuzz = (*material).fuzz;
    (*hit_record).refraction_index = (*material).refraction_index;
}

// Signed distance Functions

/// Signed distance from point to aabb.
/// Attribution: https://iquilezles.org/articles/distfunctions/
fn aabb_sd(hittables_bvh_node_index: u32, point: vec3<f32>) -> f32 {
    var aabb = scene_bvh_nodes.vals[ scene_hittables.vals[hittables_bvh_node_index].scene_index ].aabb;

    // Box is defined by half lengths.
    var b = (aabb.max - aabb.min) / 2.0; 
    // Set point relative to center of aabb.
    var p = point - b - aabb.min;

    var q = abs(p) - b;
    return length(max(q, vec3<f32>(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

fn sphere_sd(sphere_index: u32, point: vec3<f32>, hit_record: ptr<function, HitRecord>) -> f32 {
    var sphere = scene_spheres.vals[sphere_index];
    var material = scene_materials.vals[sphere.material_index];

    // Quick hack - don't check with materials that are the wos albedo blend material
    if (material.material_type == 5u) {
        return constants.infinity;
    }

    set_material_data(hit_record, &material);
    // TODO - why this 0.9 offset to test (e.g. 1.0 offset needed?)
    return (length(point - sphere.center) - sphere.radius);
}

fn cuboid_sd(cuboid_index: u32, point: vec3<f32>, hit_record: ptr<function, HitRecord>) -> f32 {
    var cuboid = scene_cuboids.vals[cuboid_index];
    var material = scene_materials.vals[cuboid.material_index];

    // Quick hack - don't check with materials that are the wos albedo blend material
    if (material.material_type == 5u) {
        return constants.infinity;
    }

    set_material_data(hit_record, &material);

    // TODO - test - this is very similar to aabb - except that we use the cuboid.txi to support arbitary rotations/tranlsations
    var p_c = (cuboid.txi * vec4<f32>(point, 0.0)).xyz;
    var b = 0.5;
    var p = point - b;

    var q = abs(p) - b;
    return length(max(q, vec3<f32>(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

fn primitive_sd(primitive_geometry_type: u32, primitive_scene_index: u32, point: vec3<f32>, hit_record: ptr<function, HitRecord>) -> f32 {
    switch (primitive_geometry_type) {
        case 1u: {
            // Sphere
            return sphere_sd(primitive_scene_index, point, hit_record);
        }
        case 2u: {
            // Cuboid
            return cuboid_sd(primitive_scene_index, point, hit_record);
        }
        default: {
            return constants.infinity; // Non-primitive geometry type - TODO - better error.
        }
    }
}

/// Global signed distance function for all scene primatives (using bvh stack traversal)
/// Uses the hitRecord to store domain boundary data.
/// Return the signed distance from the point to the closest primitive.
fn scene_sd(point: vec3<f32>, rec: ptr<function, HitRecord>) -> f32 {
    var closest_so_far = constants.infinity;

    // Precondition, return early if scene is empty
    if (arrayLength(&scene_hittables.vals) == 0u) {
        return closest_so_far;
    }

    // Use a basic stack data structure frfom a fixed array (the stack value is the index of the scene hittable)
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
            return constants.infinity; // TODO - add better error signal
        }

        // Get hittable from top of stack 
        var current_hittable = scene_hittables.vals[ stack[stack_top] ];

        // If BVH 
        if (current_hittable.geometry_type == 0u) {
            var bvh = scene_bvh_nodes.vals[ current_hittable.scene_index ];

            // What is the distance to this bvh node?
            var dist = aabb_sd(stack[stack_top], point);

            // Pop the stack (aabb hit check done).
            stack_top = stack_top - 1;

            if (dist < closest_so_far) {
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
            continue;
        } 

        // Is this a primitive
        if (is_primitive(current_hittable.geometry_type)) {
            // Primitive
            var temp_hit_record = new_hit_record();
            var dist = primitive_sd(current_hittable.geometry_type, scene_hittables.vals[ stack[stack_top] ].scene_index, point, &temp_hit_record);

            // Pop the stack primitive hit check done.
            stack_top = stack_top - 1;
            if (dist < closest_so_far) {
                // If this is the closest so far, update the closest measure and hit record
                closest_so_far = dist;
                (*rec).albedo = temp_hit_record.albedo;
            }
            continue;
        }

        // Is this a constant medium
        if (current_hittable.geometry_type == 3u) {
            // Constant Medium
            // TODO - not sure if this should/could be implemented logically

            // Pop the stack (constant medium hit check done).
            stack_top = stack_top - 1;
            continue;
        }

        // Should never get here
        return constants.infinity; // TODO - better error signal :(
    }

    return closest_so_far;
}

/// Walk on Spheres
/// Recusively walk on spheres sampled from the test point and return a hitrecord which contains the boundary surface data. 
fn wos(point: vec3<f32>, entropy: u32) -> HitRecord {
    var dist = constants.infinity;
    var curr_point = point;
    var hr = new_hit_record();
    for (var i = 0; i < 32; i = i + 1) {
        dist = scene_sd(curr_point, &hr);
        if (dist < constants.wos_tolerance) {
            break;
        }
        curr_point = curr_point + dist * normalize(random_in_unit_sphere(hash(entropy + u32(i))));
    }

    return hr;
} 

// Ray Hit/Intersection Functions 

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

fn sphere_hit(sphere_index: u32, ray: ptr<function, Ray>, t_min: f32, t_max: f32, hit_record: ptr<function, HitRecord>) -> bool {
    var sphere = scene_spheres.vals[sphere_index];
    var material = scene_materials.vals[sphere.material_index];

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

    set_material_data(hit_record, &material);

    return true;
} 

/// Attribution: https://iquilezles.org/articles/boxfunctions/
/// WIP: Need to fix the issues with dielectric exit normals.
////     - Essentially, this hit function doesn't *appear* to work if the ray starts inside the cuboid.
fn cuboid_hit(cuboid_index: u32, ray: ptr<function, Ray>, t_min: f32, t_max: f32, hit_record: ptr<function, HitRecord>) -> bool {
    var cuboid = scene_cuboids.vals[cuboid_index];
    var material = scene_materials.vals[cuboid.material_index];

    // convert from world to box space
    var rd = (cuboid.txx * vec4<f32>((*ray).direction, 0.0)).xyz;
    var ro = (cuboid.txx * vec4<f32>((*ray).origin, 1.0)).xyz;

    // ray-box intersection in box space
    var m = safe_inf_vec3_div(vec3<f32>(1.0), rd);

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

    // Prevent overflow
    var t1 = safe_inf_vec3_mult(m, -ro + s);
    var t2 = safe_inf_vec3_mult(m, -ro - s);

    var tN = max(max(t1.x, t1.y), t1.z);
    var tF = min(min(t2.x, t2.y), t2.z);

    // check for hit with cuboid
    if (tN > tF || tF < 0.0) {
        return false;
    }

    if (tN > -constants.epsilon) {
        // Ray originates from outside cuboid
        // check hit is in allowed range
        if (tN < t_min || tN > t_max) {
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
    } else if (tF > constants.epsilon) { 
        // Ray originates from inside cuboid - ** THIS IS NOT WORKING **
        // check hit is in allowed range 
        if (tF < t_min || tF > t_max) {
            return false;
        }

        // compute normal (in world space)
        // WHY IS THING WRONG?! - verifyed on paper to be correct.
        if (t2.x < t2.y && t2.x < t2.z) {
            (*hit_record).normal = cuboid.txi[0].xyz * s.x * -1.0;
        } else if (t2.y < t2.z) {
            (*hit_record).normal = cuboid.txi[1].xyz * s.y * -1.0;
        } else {
            (*hit_record).normal = cuboid.txi[2].xyz * s.z * -1.0;
        }

        // intersection point (in world space)
        (*hit_record).p = (cuboid.txi * vec4<f32>((ro + (rd * tF)), 1.0)).xyz;

        // distance to intersection point (in world space)
        (*hit_record).t = tF;
    } else {
        return false;
    }

    set_material_data(hit_record, &material);

    return true;
}

fn primitive_hit(primitive_geometry_type: u32, primitive_scene_index: u32, ray: ptr<function, Ray>, t_min: f32, t_max: f32, hit_record: ptr<function, HitRecord>) -> bool {
    switch (primitive_geometry_type) {
        case 1u: {
            // Sphere
            return sphere_hit(primitive_scene_index, ray, t_min, t_max, hit_record);
        }
        case 2u: {
            // Cuboid
            return cuboid_hit(primitive_scene_index, ray, t_min, t_max, hit_record);
        }
        default: {
            return false; // Non-primitive geometry type
        }
    }
}

// Based on https://raytracing.github.io/books/RayTracingTheNextWeek.html (Chapter 9 Volumes)
fn constant_medium_hit(constant_medium_index: u32, ray: ptr<function, Ray>, t_min: f32, t_max: f32, hit_record: ptr<function, HitRecord>, entropy: u32) -> bool {
    var constant_medium = scene_constant_mediums.vals[constant_medium_index];
    var material = scene_materials.vals[constant_medium.material_index];

    // Check if within boundary
    // Assuming convex primitive
    var rec_1 = new_hit_record();
    var rec_2 = new_hit_record();

    if (!primitive_hit(constant_medium.boundary_geometry_type, constant_medium.boundary_scene_index, ray, -constants.infinity, constants.infinity, &rec_1)) {
        return false;
    }
    if (!primitive_hit(constant_medium.boundary_geometry_type, constant_medium.boundary_scene_index, ray, rec_1.t + constants.epsilon, constants.infinity, &rec_2)) {
        return false;
    }

    if (rec_1.t < t_min) {
        rec_1.t = t_min;
    }
    if (rec_2.t > t_max) {
        rec_2.t = t_max;
    }
    if (rec_1.t >= rec_2.t) {
        return false;
    }
    if (rec_1.t < 0.0) {
        rec_1.t = 0.0;
    }

    var ray_length = length((*ray).direction);
    var distance_inside_boundary = (rec_2.t - rec_1.t) * ray_length;
    var hit_distance = constant_medium.neg_inv_density * log(random_float(entropy));

    if (hit_distance > distance_inside_boundary) {
        return false;
    }

    (*hit_record).t = rec_1.t + hit_distance / ray_length;
    (*hit_record).p = ray_at(ray, (*hit_record).t);

    set_material_data(hit_record, &material);

    return true;
}



/// Global ray hit function for all scene primitives (using bvh stack traversal).
fn scene_hits(ray: ptr<function, Ray>, t_min: f32, t_max: f32, rec: ptr<function, HitRecord>, entropy: u32) -> bool {
    var hit_anything = false;
    var closest_so_far = t_max;

    // Precondition, return early if scene is empty
    if (arrayLength(&scene_hittables.vals) == 0u) {
        return hit_anything;
    }

    // Use a basic stack data structure frfom a fixed array (the stack value is the index of the scene hittable)
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

        // If BVH 
        if (current_hittable.geometry_type == 0u) {
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
            continue;
        } 

        // Is this a primitive
        if (is_primitive(current_hittable.geometry_type)) {
            // Primitive
            var hit = primitive_hit(current_hittable.geometry_type, scene_hittables.vals[ stack[stack_top] ].scene_index, ray, t_min, closest_so_far, rec);

            // Pop the stack primitive hit check done.
            stack_top = stack_top - 1;
            if (hit) {
                hit_anything = true;
                closest_so_far = (*rec).t;
            }
            continue;
        }

        // Is this a constant medium
        if (current_hittable.geometry_type == 3u) {
            // Constant Medium
            var hit = constant_medium_hit(scene_hittables.vals[ stack[stack_top] ].scene_index, ray, t_min, closest_so_far, rec, hash(entropy + u32(scene_hittables.vals[ stack[stack_top] ].scene_index)));

            // Pop the stack (constant medium hit check done).
            stack_top = stack_top - 1;

            if (hit) {
                hit_anything = true;
                closest_so_far = (*rec).t;
            }
            continue;
        }

        // Should never get here
        return false; // :(
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
        var hit = scene_hits(&current_ray, 0.001, constants.infinity, &hit_record, hash(entropy + u32(i)));

        // For rendering the bvh (only care about number of bvh intersections on before first bounce)hash(entropy + u32(i))
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
            } else if (hit_record.material_type == 4u) {
                // Isotropic medium
                var scattered = hit_record.p + random_in_unit_sphere(entropy * u32(i + 4));

                current_ray = Ray(hit_record.p, scattered - hit_record.p);
                current_ray_color = current_ray_color * hit_record.albedo;
            } else if (hit_record.material_type == 5u) {
                // WoS blend material

                // TODO call WOS algorithm (which in turn will sample nearest signed distance functions
                // var mat_sample_rec = wos(hit_record.p, entropy * u32(i + 5));

                // DEBUG - a few hardcoded wos steps
                var mat_sample_rec = new_hit_record();
                mat_sample_rec.albedo = vec3<f32>(1.0);
                var closest_dist = scene_sd(hit_record.p, &mat_sample_rec);
                var new_p = hit_record.p + closest_dist * normalize(random_in_unit_sphere(entropy * u32(i + 5)));
                var closest_dist2 = scene_sd(new_p, &mat_sample_rec);
                var new_p1 = hit_record.p + closest_dist2 * normalize(random_in_unit_sphere(entropy * u32(i + 5)));
                var closest_dist3 = scene_sd(new_p1, &mat_sample_rec);

                current_ray_color = current_ray_color * mat_sample_rec.albedo;
                // current_ray_color = current_ray_color * vec3<f32>(closest_dist);
                // current_ray_color = current_ray_color * vec3<f32>(closest_dist2);
            }
        } else {
            // No hit, return background / sky color gradient
            current_ray_color = current_ray_color * scene_background.albedo;
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


