extern crate glm;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const MOVEMENT_SPEED: f32 = 3.0;
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

    fn calc_movement_velocity(&self, _time: f32) -> f32 {
        // TODO: handle sprint
        return _time * MOVEMENT_SPEED;
    }

    /// Processes camera for this frame
    pub fn process_event(&mut self, _time: f32, event: &Event) {
        // TODO: split this into a controller class
        match event {
            Event::MouseMotion{ xrel, yrel, .. } => {
                /* Modify pitch and yaw based on mouse motion */
                self.pitch -= *yrel as f32 * MOUSE_SENSITIVITY;
                self.yaw += *xrel as f32 * MOUSE_SENSITIVITY;

                /* Constrain pitch */
                if self.pitch > CAMERA_PITCH_CONSTRAINT {
                    self.pitch = CAMERA_PITCH_CONSTRAINT;
                } else if self.pitch < -CAMERA_PITCH_CONSTRAINT {
                    self.pitch = -CAMERA_PITCH_CONSTRAINT;
                }

                self.recalc_vectors();
            },
            Event::KeyDown{ keycode: Option::Some(Keycode::W), ..} => {
                self.pos = self.pos + self.front * self.calc_movement_velocity(_time);
            }
            Event::KeyDown{ keycode: Option::Some(Keycode::S), ..} => {
                self.pos = self.pos - self.front * self.calc_movement_velocity(_time);
            }
            Event::KeyDown{ keycode: Option::Some(Keycode::A), ..} => {
                self.pos = self.pos - self.right * self.calc_movement_velocity(_time);
            }
            Event::KeyDown{ keycode: Option::Some(Keycode::D), ..} => {
                self.pos = self.pos + self.right * self.calc_movement_velocity(_time);
            }
            _ => {},
        }
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
