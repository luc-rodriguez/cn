use winit::{
    event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{Key, PhysicalKey},
};

#[derive(Clone)]
pub enum KeyAction {
    Down(Key),
    Os(Key),
    Up(Key),
}

#[derive(Clone)]
pub enum MouseAction {
    Down(MouseButton),
    Up(MouseButton),
}

#[derive(Clone)]
pub struct State {
    pub mouse_actions: Vec<MouseAction>,
    pub key_actions: Vec<KeyAction>,
    pub keys_held: Vec<Key>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            mouse_actions: vec![],
            key_actions: vec![],
            keys_held: vec![],
        }
    }

    pub fn new_events(&mut self) {
        self.mouse_actions.clear();
        self.key_actions.clear();
    }
}