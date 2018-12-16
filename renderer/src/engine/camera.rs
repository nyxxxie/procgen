extern crate glm;
extern crate sdl2;

use sdl2::event::Event;

const MOVEMENT_SPEED: f32 = 3.0;
const MOUSE_SENSITIVITY: f32 = 0.1;
const CAMERA_PITCH: f32 = 0.0;
const CAMERA_PITCH_CONSTRAINT: f32 = 89.0;
const CAMERA_YAW: f32 = -90.0;

pub struct Camera {
    front: glm::Vector3<f32>,
    right: glm::Vector3<f32>,
    up: glm::Vector3<f32>,
    pos: glm::Vector3<f32>,
    pitch: f32,
    yaw: f32
}

impl Camera {
    pub fn new(pos: glm::Vector3<f32>) -> Camera {
        return Camera {
            front: glm::vec3(0.0, 0.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            up: glm::vec3(0.0, 0.0, 0.0),
            pos,
            pitch: CAMERA_PITCH,
            yaw: CAMERA_YAW
        };
    }

    /// Processes camera for this frame
    pub fn process_event(&self, _time: f32, event: &Event) {
        match event {
            Event::MouseMotion{ x, y, xrel, yrel, .. } => {
                println!("x:{}, y:{} xrel:{}, yrel:{}", x, y, xrel, yrel);
            },
            _ => {},
        }
    }

    /// Returns view and perspective matrices to transform vectors by in order
    /// to see them from this camera's perspective.
    pub fn get_view_matrix(self) -> glm::Matrix4<f32> {
        return glm::ext::look_at(self.pos, self.pos + self.front, self.up);
    }

    fn recalc_vectors(mut self) {
        // TODO: explain math
        self.front = glm::normalize(glm::vec3(
                glm::cos(glm::radians(self.yaw)) * glm::cos(glm::radians(self.pitch)),
                glm::sin(glm::radians(self.pitch)),
                glm::sin(glm::radians(self.yaw)) * glm::cos(glm::radians(self.pitch))));
        self.right = glm::normalize(glm::cross(self.front, glm::vec3(0.0, 1.0, 0.0)));
        self.up = glm::normalize(glm::cross(self.right, self.front));
    }
}
