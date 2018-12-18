extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Controller {
    up: bool,
    down: bool,
    forward: bool,
    back: bool,
    left: bool,
    right: bool,
    sprint: bool,
    delta_x: f32,
    delta_y: f32,
    mouse_moved: bool
}

impl Controller {
    pub fn new() -> Controller {
        return Controller {
            up: false,
            down: false,
            forward: false,
            back: false,
            left: false,
            right: false,
            sprint: false,
            delta_x: 0f32,
            delta_y: 0f32,
            mouse_moved: false
        }
    }

    pub fn reset_mouse(&mut self) {
        self.mouse_moved = false;
        self.delta_x = 0f32;
        self.delta_y = 0f32;
    }

    pub fn process_window_event(&mut self, event: &Event) {
        match event {
            Event::MouseMotion{ xrel, yrel, .. } => {
                self.mouse_moved = true;
                self.delta_x = *xrel as f32;
                self.delta_y = *yrel as f32;
            },
            Event::KeyDown{ keycode: Option::Some(Keycode::W), ..} => {
                self.forward = true;
            }
            Event::KeyUp{ keycode: Option::Some(Keycode::W), ..} => {
                self.forward = false;
            }
            Event::KeyDown{ keycode: Option::Some(Keycode::S), ..} => {
                self.back = true;
            }
            Event::KeyUp{ keycode: Option::Some(Keycode::S), ..} => {
                self.back = false;
            }
            Event::KeyDown{ keycode: Option::Some(Keycode::A), ..} => {
                self.left = true;
            }
            Event::KeyUp{ keycode: Option::Some(Keycode::A), ..} => {
                self.left = false;
            }
            Event::KeyDown{ keycode: Option::Some(Keycode::D), ..} => {
                self.right = true;
            }
            Event::KeyUp{ keycode: Option::Some(Keycode::D), ..} => {
                self.right = false;
            }
            Event::KeyDown{ keycode: Option::Some(Keycode::Space), ..} => {
                self.up = true;
            }
            Event::KeyUp{ keycode: Option::Some(Keycode::Space), ..} => {
                self.up = false;
            }
            Event::KeyDown{ keycode: Option::Some(Keycode::LShift), ..} => {
                self.sprint = true;
            }
            Event::KeyUp{ keycode: Option::Some(Keycode::LShift), ..} => {
                self.sprint = false;
            }
            Event::KeyDown{ keycode: Option::Some(Keycode::LCtrl), ..} => {
                self.down = true;
            }
            Event::KeyUp{ keycode: Option::Some(Keycode::LCtrl), ..} => {
                self.down = false;
            }
            _ => {},
        }
    }

    pub fn up_pressed(&self) -> bool {
        return self.up;
    }

    pub fn down_pressed(&self) -> bool {
        return self.down;
    }

    pub fn forward_pressed(&self) -> bool {
        return self.forward;
    }

    pub fn back_pressed(&self) -> bool {
        return self.back;
    }

    pub fn left_pressed(&self) -> bool {
        return self.left;
    }

    pub fn right_pressed(&self) -> bool {
        return self.right;
    }

    pub fn was_mouse_moved(&self) -> bool {
        return self.mouse_moved;
    }

    pub fn is_sprinting(&self) -> bool {
        return self.sprint;
    }

    pub fn get_delta_x(&self) -> f32 {
        return self.delta_x;
    }

    pub fn get_delta_y(&self) -> f32 {
        return self.delta_y;
    }
}
