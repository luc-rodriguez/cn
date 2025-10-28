mod state;

use web_time::{Instant, Duration};

use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, MouseButton, WindowEvent},
    keyboard::{Key, KeyCode, PhysicalKey},
};

use crate::state::State;

#[derive(Clone)]
pub struct Cn {
    state: Option<State>,

    destroyed: bool,
    closing: bool,

    tick: Option<Instant>,
    dt: Option<Duration>,
}

impl Cn {
    pub fn new() -> Self {
        Self {
            state: Some(State::new()),
            
            closing: false,
            
            tick: None,
            dt: None,
        }
    }

    /// Call every time `ApplicationHandler.new_events()` is called.
    /// Clears all internal state.
    pub fn new_events(&mut self) {
        self.closing = false;

        self.tick.get_or_insert(Instant::now());
        self.dt = None;

        if let Some(state) = &mut self.state {
            state.new_events();
        }
    }

    pub fn window_event(&mut self, event: &WindowEvent) -> bool {
        let mut redraw_requested = false;

        match event {
            WindowEvent::CloseRequested => self.closing = true,
            WindowEvent::Destroyed => self.destroyed = true,
            WindowEvent::Focused(false) => self.state = None,
            WindowEvent::Focused(true) => {
                if self.state.is_none() {
                    self.state = Some(State::new());
                }
            }
            // ...
            WindowEvent::RedrawRequested => {
                redraw_requested = true;
            }
            _ => {}
        }

        redraw_requested
    }

    pub fn about_to_wait(&mut self) {
        self.dt = self.tick.map(|i| i.elapsed());
        self.tick = Some(Instant::now());
    }

    
}