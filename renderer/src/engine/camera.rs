extern crate glm;

use engine::controller::Controller;

const MOVEMENT_SPEED: f32 = 3.0;
const SPRINT_MODIFIER: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.1;
const CAMERA_PITCH: f32 = 0.0;
const CAMERA_PITCH_CONSTRAINT: f32 = 89.0;
const CAMERA_YAW: f32 = -90.0;
const CAMERA_FOV: f32 = 45.0;

pub struct Camera {
    front: glm::Vector3<f32>,
    right: glm::Vector3<f32>,
    up: glm::Vector3<f32>,
    pos: glm::Vector3<f32>,
    pitch: f32,
    yaw: f32,
    window_width: f32,
    window_height: f32,
}

impl Camera {
    pub fn new(pos: glm::Vector3<f32>, window_width: i32, window_height: i32) -> Camera {
        let mut camera = Camera {
            front: glm::vec3(0.0, 0.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            up: glm::vec3(0.0, 0.0, 0.0),
            pos,
            pitch: CAMERA_PITCH,
            yaw: CAMERA_YAW,
            window_width: window_width as f32,
            window_height: window_height as f32
        };

        camera.recalc_vectors();

        return camera;
    }

    fn calc_movement_velocity(&self, _time: f32, sprint: bool) -> f32 {
        // TODO: handle sprint
        return _time * MOVEMENT_SPEED * (if sprint { SPRINT_MODIFIER } else { 1.0 });
    }

    pub fn process_controller_input(&mut self, _time: f32, controller: &Controller) {
        if controller.was_mouse_moved() {
            /* Adjust yaw */
            self.yaw += controller.get_delta_x() * MOUSE_SENSITIVITY;

            /* Adjust pitch */
            self.pitch -= controller.get_delta_y() * MOUSE_SENSITIVITY;

            /* Constrain pitch */
            if self.pitch > CAMERA_PITCH_CONSTRAINT {
                self.pitch = CAMERA_PITCH_CONSTRAINT;
            } else if self.pitch < -CAMERA_PITCH_CONSTRAINT {
                self.pitch = -CAMERA_PITCH_CONSTRAINT;
            }
        }

        /* Handle vertical movement */
        if controller.up_pressed() {
            self.pos = self.pos + self.up * self.calc_movement_velocity(_time, controller.is_sprinting());
        } else if controller.down_pressed() {
            self.pos = self.pos - self.up * self.calc_movement_velocity(_time, controller.is_sprinting());
        }

        /* Handle vertical movement */
        if controller.forward_pressed() {
            self.pos = self.pos + self.front * self.calc_movement_velocity(_time, controller.is_sprinting());
        } else if controller.back_pressed() {
            self.pos = self.pos - self.front * self.calc_movement_velocity(_time, controller.is_sprinting());
        }

        /* Handle vertical movement */
        if controller.left_pressed() {
            self.pos = self.pos - self.right * self.calc_movement_velocity(_time, controller.is_sprinting());
        } else if controller.right_pressed() {
            self.pos = self.pos + self.right * self.calc_movement_velocity(_time, controller.is_sprinting());
        }

        self.recalc_vectors();
    }

    /// Returns view and perspective matrices to transform vectors by in order
    /// to see them from this camera's perspective.
    pub fn get_view_matrix(&self) -> glm::Matrix4<f32> {
        return glm::ext::look_at(self.pos, self.pos + self.front, self.up);
    }

    pub fn get_projection_matrix(&self) -> glm::Matrix4<f32> {
        return glm::ext::perspective(glm::radians(CAMERA_FOV),
                                     self.window_width/self.window_height,
                                     0.1, 1000.0);
    }

    fn recalc_vectors(&mut self) {
        // TODO: explain math
        self.front = glm::normalize(glm::vec3(
                glm::cos(glm::radians(self.yaw)) * glm::cos(glm::radians(self.pitch)),
                glm::sin(glm::radians(self.pitch)),
                glm::sin(glm::radians(self.yaw)) * glm::cos(glm::radians(self.pitch))));
        self.right = glm::normalize(glm::cross(self.front, glm::vec3(0.0, 1.0, 0.0)));
        self.up = glm::normalize(glm::cross(self.right, self.front));
    }
}
