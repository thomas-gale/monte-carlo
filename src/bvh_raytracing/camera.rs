use std::f32::consts::PI;

use cgmath::{prelude::*, Matrix4, Point3, Rad, Vector2, Vector3};

use super::{buffer_bindings, result, util, window};

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
        origin: Point3<f32>,
        lower_left_corner: Point3<f32>,
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
    look_from: Point3<f32>, // eye
    look_at: Point3<f32>,   // target
    v_up: Vector3<f32>,     // orientation of the camera
    v_fov: f32,
    window: window::Window,
    aperture: f32,
    focus_dist: f32,
    zoom_speed: f32,

    view_matrix: Matrix4<f32>,

    raw: CameraRaw,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl Camera {
    pub fn new(
        device: &wgpu::Device,
        look_from: Point3<f32>,
        look_at: Point3<f32>,
        v_up: Vector3<f32>,
        v_fov: f32,
        window: window::Window,
        aperture: f32,
        focus_dist: f32,
        zoom_speed: f32,
    ) -> Self {
        let raw = Self::generate_raw(
            &look_from, &look_at, &v_up, v_fov, window, aperture, focus_dist,
        );

        let (bind_group_layout, bind_group, buffer) = buffer_bindings::create_device_buffer_binding(
            &[raw],
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Uniform,
        );

        let mut new_cam = Camera {
            look_from,
            look_at,
            v_up,
            v_fov,
            window,
            aperture,
            focus_dist,
            zoom_speed,

            view_matrix: Matrix4::identity(),

            raw,
            bind_group_layout,
            bind_group,
            buffer,
        };
        new_cam.update_view_matrix();

        new_cam
    }

    fn generate_raw(
        look_from: &Point3<f32>,
        look_at: &Point3<f32>,
        v_up: &Vector3<f32>,
        v_fov: f32,
        window: window::Window,
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

    // pub fn set_window(&mut self, window: window::Window) {
    //     self.window = window;
    // }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        let raw = Self::generate_raw(
            &self.look_from,
            &self.look_at,
            &self.v_up,
            self.v_fov,
            self.window,
            self.aperture,
            self.focus_dist,
        );
        self.raw = raw;
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.raw]));
    }

    ///
    /// Simple zoom modification to the look_from of the camera
    ///
    pub fn zoom(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        result: &mut result::Result,
        size: winit::dpi::PhysicalSize<u32>,
        mouse_wheel_y: f32,
    ) {
        self.set_camera_view(
            self.look_from + self.zoom_speed * mouse_wheel_y * (self.look_at - self.look_from),
            self.look_at,
            self.v_up,
        );

        // Push the changes to the GPU
        self.update(queue);
        // Reset the accumulation ray color result texture
        result.reset_texture(device, queue, size);
    }

    ///
    /// Arcball implementation - attribution: https://asliceofrendering.com/camera/2019/11/30/ArcballCamera/
    ///
    pub fn rotate(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        result: &mut result::Result,
        size: winit::dpi::PhysicalSize<u32>,
        mouse_prev: Vector2<f32>,
        mouse_cur: Vector2<f32>,
    ) {
        // Homogeneous position of camera and pivot point
        let mut position = self.look_from.to_vec().extend(1.0);
        let pivot = self.look_at.to_vec().extend(1.0);

        // Calculate amount of rotation in mouse movement
        let delta_angle_x: f32 = 2.0 * PI / size.width as f32; // a movement from left to right = 2*PI = 360 deg
        let mut delta_angle_y: f32 = PI / size.height as f32; // a movement from top to bottom = PI = 180 deg

        // Extra step to handle the problem when the camera direction is the same as the up vector
        let cos_angle: f32 = self.get_view_dir().dot(self.v_up);
        if cos_angle * f32::signum(delta_angle_y) > 0.99 {
            delta_angle_y = 0.0;
        }

        // Covert mouse movement to arcball coordinates
        // Y is reversed as the screen has the origin in the top left corner
        let x_angle: f32 = (mouse_prev.x - mouse_cur.x) * delta_angle_x;
        let y_angle: f32 = (mouse_prev.y - mouse_cur.y) * delta_angle_y;

        // Rotate the camera around the pivot point on the first axis.
        let rotation_matrix_x = Matrix4::from_axis_angle(self.v_up, Rad(x_angle));
        position = rotation_matrix_x * (position - pivot) + pivot;

        // Rotate the camera around the pivot point on the second axis.
        let rotation_matrix_y = Matrix4::from_axis_angle(self.get_right_vector(), Rad(y_angle));
        position = rotation_matrix_y * (position - pivot) + pivot;

        self.set_camera_view(
            Point3::new(position.x, position.y, position.z),
            self.look_at,
            self.v_up,
        );

        // Push the changes to the GPU
        self.update(queue);
        // Reset the accumulation ray color result texture
        result.reset_texture(device, queue, size);
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn set_camera_view(
        &mut self,
        look_from: Point3<f32>,
        look_at: Point3<f32>,
        v_up: Vector3<f32>,
    ) {
        self.look_from = look_from;
        self.look_at = look_at;
        self.v_up = v_up;
        self.update_view_matrix();
    }

    fn update_view_matrix(&mut self) {
        self.view_matrix = Matrix4::look_at_rh(self.look_from, self.look_at, self.v_up);
    }

    // Camera forward is -z
    fn get_view_dir(&self) -> Vector3<f32> {
        -self.view_matrix.transpose()[2].truncate()
    }

    fn get_right_vector(&self) -> Vector3<f32> {
        self.view_matrix.transpose()[0].truncate()
    }
}
