mod state;

use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, MouseButton, WindowEvent},
    keyboard::{Key, KeyCode, PhysicalKey},
};

use crate::state::State;

#[derive(Clone)]
pub struct Cn {
    state: Option<State>,
    closing: bool,
}

impl Cn {
    pub fn new() -> Self {
        Self {
            state: Some(State::new()),
            closing: false,
        }
    }
}