use std::collections::HashSet;

use winit::{dpi::{PhysicalPosition, PhysicalSize}, event::{DeviceEvent, ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent}, keyboard::{Key, KeyCode, NativeKeyCode, PhysicalKey}};
use web_time::{Instant, Duration};

pub struct Inputs<'a> {
    pub mouse: Mouse<'a>,
    pub keyboard: Keyboard<'a>,
    pub size: Option<PhysicalSize<u32>>,
    pub scale_factor: Option<f64>,
    pub close_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MouseInput {
    state: ElementState,
    button: MouseButton,
}

pub struct Mouse<'a> {
    pub position: PhysicalPosition<f64>,
    /// With mouse acceleration
    pub velocity: (f64, f64),
    /// Raw, unfiltered mouse motion
    pub motion: (f64, f64),
    pub scroll: PhysicalPosition<f64>,
    pub inputs: Vec<MouseInput>,
    pub held: &'a HashSet<MouseButton>,
}

pub struct Keyboard<'a> {
    pub inputs: Vec<KeyEvent>,
    pub physical_held: &'a HashSet<PhysicalKey>,
    pub logical_held: &'a HashSet<Key>,
}

impl<'a> Mouse<'a> {
    pub fn held(&self, button: MouseButton) -> bool {
        self.held.contains(&button)
    }

    pub fn down(&self, button: MouseButton) -> bool {
        self.inputs
            .iter()
            .any(|i| i.state == ElementState::Pressed && i.button == button)
    }

    pub fn up(&self, button: MouseButton) -> bool {
        self.inputs
            .iter()
            .any(|i| i.state == ElementState::Released && i.button == button)
    }
}

impl<'a> Keyboard<'a> {
    pub fn held_l(&self, key: Key) -> bool {
        self.logical_held.contains(&key)
    }

    pub fn down_l(&self, key: Key) -> bool {
        self.inputs
            .iter()
            .any(|i| i.state == ElementState::Pressed && i.logical_key == key)
    }

    pub fn up_l(&self, key: Key) -> bool {
        self.inputs
            .iter()
            .any(|i| i.state == ElementState::Released && i.logical_key == key)
    }

    pub fn held_p(&self, key: KeyCode) -> bool {
        let key = PhysicalKey::Code(key);
        self.physical_held.contains(&key)
    }

    pub fn down_p(&self, key: KeyCode) -> bool {
        let key = PhysicalKey::Code(key);
        self.inputs
            .iter()
            .any(|i| i.state == ElementState::Pressed && i.physical_key == key)
    }

    pub fn up_p(&self, key: KeyCode) -> bool {
        let key = PhysicalKey::Code(key);
        self.inputs
            .iter()
            .any(|i| i.state == ElementState::Released && i.physical_key == key)
    }

    pub fn held_pu(&self, key: NativeKeyCode) -> bool {
        let key = PhysicalKey::Unidentified(key);
        self.physical_held.contains(&key)
    }

    pub fn down_pu(&self, key: NativeKeyCode) -> bool {
        let key = PhysicalKey::Unidentified(key);
        self.inputs
            .iter()
            .any(|i| i.state == ElementState::Pressed && i.physical_key == key)
    }

    pub fn up_pu(&self, key: NativeKeyCode) -> bool {
        let key = PhysicalKey::Unidentified(key);
        self.inputs
            .iter()
            .any(|i| i.state == ElementState::Released && i.physical_key == key)
    }

    pub fn text(&self) -> String {
        self.inputs
            .iter()
            .filter_map(|i| i.text.as_ref().map(|v| v.as_str()))
            .collect()
    }
}

pub struct State {
    from: PhysicalPosition<f64>,
    to: PhysicalPosition<f64>,

    motion: (f64, f64),
    scroll: PhysicalPosition<f64>,

    mouse_inputs: Vec<MouseInput>,
    mouse_held: HashSet<MouseButton>,
    
    key_inputs: Vec<KeyEvent>,
    p_keys_held: HashSet<PhysicalKey>,
    l_keys_held: HashSet<Key>,
    
    size: Option<PhysicalSize<u32>>,
    scale_factor: Option<f64>,

    close_requested: bool,

    tick: Option<Instant>,
    dt: Option<Duration>,
}

impl State {
    pub fn new() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            
            motion: (0.0, 0.0),
            scroll: Default::default(),
            
            mouse_inputs: Vec::new(),
            mouse_held: HashSet::new(),
            
            key_inputs: Vec::new(),
            p_keys_held: HashSet::new(),
            l_keys_held: HashSet::new(),

            size: None,
            scale_factor: None,

            close_requested: false,

            tick: None,
            dt: None,
        }
    }

    pub fn window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.to = *position;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse_inputs.push(MouseInput {
                    state: *state,
                    button: *button,
                });

                match state {
                    ElementState::Pressed => {
                        self.mouse_held.insert(*button);
                    }
                    ElementState::Released => {
                        self.mouse_held.remove(button);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                const PIXELS_PER_LINE: f64 = 38.0;

                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.scroll.x += (*x as f64) * PIXELS_PER_LINE;
                        self.scroll.y += (*y as f64) * PIXELS_PER_LINE;
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        self.scroll.x += delta.x;
                        self.scroll.y += delta.y;
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.key_inputs.push(event.clone());

                match event.state {
                    ElementState::Pressed => {
                        self.l_keys_held.insert(event.logical_key.clone());
                        self.p_keys_held.insert(event.physical_key);
                    }
                    ElementState::Released => {
                        self.l_keys_held.remove(&event.logical_key);
                        self.p_keys_held.remove(&event.physical_key);
                    }
                }
            }
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            WindowEvent::Resized(size) => {
                self.size = Some(*size);
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = Some(*scale_factor);
            }
            _ => {}
        }
    }

    pub fn device_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.motion.0 += delta.0;
                self.motion.1 += delta.1;
            }
            _ => {}
        }
    }

    pub fn new_events(&'_ mut self) -> Inputs<'_> {
        let velocity = (self.to.x - self.from.x, self.to.y - self.from.y);

        let inputs = Inputs {
            mouse: Mouse {
                position: self.to,
                velocity,
                motion: self.motion,
                scroll: self.scroll,
                inputs: self.mouse_inputs.clone(),
                held: &self.mouse_held,
            },
            keyboard: Keyboard {
                inputs: self.key_inputs.clone(),
                physical_held: &self.p_keys_held,
                logical_held: &self.l_keys_held,
            },
            size: self.size,
            scale_factor: self.scale_factor,
            close_requested: self.close_requested,
        };

        self.from = self.to;
        self.scroll = Default::default();
        self.motion = (0.0, 0.0);
        self.mouse_inputs.clear();
        self.key_inputs.clear();
        self.size = None;
        self.scale_factor = None;
        self.close_requested = false;
        self.tick.get_or_insert(Instant::now());
        self.dt = None;

        inputs
    }

    pub fn about_to_wait(&mut self) {
        self.dt = self.tick.map(|s| s.elapsed());
        self.tick = Some(Instant::now());
    }
}