use nalgebra_glm as glm;
use glm::{Mat4, Vec3, Vec2};
use crate::app::App;
use crate::glutin::event::VirtualKeyCode;

pub struct OrthographicCamera {
    projection: Mat4,
    view: Mat4,
    projection_view: Mat4,
    position: Vec3,
    rotation: f32,
}

impl OrthographicCamera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let projection = Mat4::new_orthographic(left, right, bottom, top, -1.0, 1.0);
        OrthographicCamera {
            projection,
            view: glm::identity(),
            projection_view: projection * glm::identity(),
            position: glm::vec3(0.0, 0.0,  0.0),
            rotation: 0.0,
        }
    }

    // https://github.com/dimforge/nalgebra/blob/dev/examples/transform_matrix4.rs
    pub fn recalculate_matrix(&mut self) {
        let translation = glm::translate(&glm::identity(), &self.position);
        let transform =  translation* glm::rotate(&glm::identity(), self.rotation,&glm::vec3(0.0, 0.0, 1.0));
        //nalgebra::try_invert_to(transform, &mut self.view);
        self.view = glm::inverse(&transform);
        self.projection_view = self.projection * self.view;
    }

    pub fn set_projection(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        self.projection = Mat4::new_orthographic(left, right, bottom, top,-1.0, 1.0);
        self.projection_view = self.projection * self.view;
    }

    pub fn get_projection_view(&self) -> Mat4 {
        self.projection_view
    }

    pub fn set_rotation(&mut self, value: f32) {
        self.rotation = value;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_position(&mut self, value: Vec3) {
        self.position = value;
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }
}

pub struct WasdCameraController {
    camera: OrthographicCamera,
    position: Vec3,
    rotation: f32,
    zoom: f32,
    aspect_ratio: f32,
    translation_speed: f32,
    rotation_speed: f32,
}

impl WasdCameraController {
    pub fn new(aspect_ratio: f32) -> Self {
        WasdCameraController {
            camera: OrthographicCamera::new(
                -aspect_ratio, aspect_ratio, -1.0, 1.0,
            ),
            aspect_ratio,
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: 0.0,
            zoom: 1.0,
            translation_speed: 1.0,
            rotation_speed: 1.0,
        }
    }

    pub fn set_rotation(&mut self, value: f32) {
        self.rotation = value;
        self.camera.set_rotation(value);
    }

    pub fn get_rotation(&self) -> f32 { self.rotation }

    pub fn get_camera(&mut self) -> &mut OrthographicCamera { &mut self.camera }

    pub fn set_position(&mut self, value: Vec3) {
        self.position = value;
        self.camera.set_position(value);
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_rotation_speed(&mut self, value: f32) {
        self.rotation_speed = value;
    }

    pub fn get_rotation_speed(&self) -> f32 { self.rotation_speed }

    pub fn set_translation_speed(&mut self, value: f32) {
        self.translation_speed = value;
    }

    pub fn get_translation_speed(&self) -> f32 {
        self.translation_speed
    }

    fn reset_projection(&mut self) {
        self.camera.set_projection(
            -self.aspect_ratio * self.zoom,
            self.aspect_ratio * self.zoom,
            -self.zoom,
            self.zoom
        );
    }

    pub fn on_window_resize(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
        self.reset_projection();
    }

    pub fn on_mouse_scroll(&mut self, delta: Vec2) -> bool {
        self.zoom = self.zoom - *delta.get(1).unwrap() as f32;
        self.zoom = self.zoom.max(0.25);
        self.reset_projection();
        false
    }

    pub fn on_tick(&mut self) {
        if App::is_key_pressed(VirtualKeyCode::A) {
            self.camera.set_position(self.camera.get_position() + glm::vec3(-0.01, 0.0, 0.0));
        } else if App::is_key_pressed(VirtualKeyCode::D) {
            self.camera.set_position(self.camera.get_position() + glm::vec3(0.01, 0.0, 0.0));
        } else if App::is_key_pressed(VirtualKeyCode::W) {
            self.camera.set_position(self.camera.get_position() + glm::vec3(0.0, 0.01, 0.0));
        } else if App::is_key_pressed(VirtualKeyCode::S) {
            self.camera.set_position(self.camera.get_position() + glm::vec3(0.0, -0.01, 0.0));
        }
    }
}