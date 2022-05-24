use cgmath::{prelude::*, Vector3};

use super::{buffer_bindings, util, window};

// Note: Due to wgsl uniforms requiring 16 byte (4 float) spacing, we need to use a padding fields here.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraRaw {
    pub origin: [f32; 3],
    _padding1: f32,
    pub lower_left_corner: [f32; 3],
    _padding2: f32,
    pub horizontal: [f32; 3],
    _padding3: f32,
    pub vertical: [f32; 3],
    _padding4: f32,
    pub u: [f32; 3],
    _padding5: f32,
    pub v: [f32; 3],
    _padding6: f32,
    pub w: [f32; 3],
    pub lens_radius: f32,
}

impl CameraRaw {
    pub fn new(
        origin: Vector3<f32>,
        lower_left_corner: Vector3<f32>,
        horizontal: Vector3<f32>,
        vertical: Vector3<f32>,
        u: Vector3<f32>,
        v: Vector3<f32>,
        w: Vector3<f32>,
        lens_radius: f32,
    ) -> CameraRaw {
        CameraRaw {
            origin: origin.into(),
            _padding1: 0.0,
            lower_left_corner: lower_left_corner.into(),
            _padding2: 0.0,
            horizontal: horizontal.into(),
            _padding3: 0.0,
            vertical: vertical.into(),
            _padding4: 0.0,
            u: u.into(),
            _padding5: 0.0,
            v: v.into(),
            _padding6: 0.0,
            w: w.into(),
            lens_radius,
        }
    }
}

pub struct Camera {
    look_from: Vector3<f32>,
    look_at: Vector3<f32>,
    v_up: Vector3<f32>,
    v_fov: f32,
    window: window::Window,
    aperture: f32,
    focus_dist: f32,

    raw: CameraRaw,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl Camera {
    pub fn new(
        device: &wgpu::Device,
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        v_up: Vector3<f32>,
        v_fov: f32,
        window: window::Window,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let raw = Self::generate_raw(
            &look_from, &look_at, &v_up, v_fov, &window, aperture, focus_dist,
        );

        let (bind_group_layout, bind_group, buffer) = buffer_bindings::create_device_buffer_binding(
            &[raw],
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Uniform,
        );

        Camera {
            look_from,
            look_at,
            v_up,
            v_fov,
            window,
            aperture,
            focus_dist,

            raw,
            bind_group_layout,
            bind_group,
            buffer,
        }
    }

    fn generate_raw(
        look_from: &Vector3<f32>,
        look_at: &Vector3<f32>,
        v_up: &Vector3<f32>,
        v_fov: f32,
        window: &window::Window,
        aperture: f32,
        focus_dist: f32,
    ) -> CameraRaw {
        let aspect_ratio = window.width_pixels as f32 / window.height_pixels as f32;
        let theta = util::degrees_to_radians(v_fov);
        let h = std::primitive::f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = v_up.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from.clone();
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;

        CameraRaw::new(
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        )
    }

    pub fn set_window(&mut self, window: window::Window) {
        self.window = window;
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        let raw = Self::generate_raw(
            &self.look_from,
            &self.look_at,
            &self.v_up,
            self.v_fov,
            &self.window,
            self.aperture,
            self.focus_dist,
        );
        self.raw = raw;
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.raw]));
    }

    pub fn translate(&mut self, queue: &wgpu::Queue, delta: Vector3<f32>) {
        self.raw.origin[0] += delta.x;
        self.raw.origin[1] += delta.y;
        self.raw.origin[2] += delta.z;
        self.raw.lower_left_corner[0] += delta.x;
        self.raw.lower_left_corner[1] += delta.y;
        self.raw.lower_left_corner[2] += delta.z;
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.raw]));
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
