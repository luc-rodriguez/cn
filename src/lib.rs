mod state;

use std::collections::HashMap;

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::state::State;
pub use state::Inputs;

pub struct Cn<App> {
    pub app: App,
    windows: HashMap<WindowId, State>,
}

impl<App> Cn<App> {
    pub fn new(app: App) -> Self {
        Self {
            app,
            windows: HashMap::new(),
        }
    }
}

pub trait Tu {
    fn iter(&mut self, event_loop: &ActiveEventLoop, input: Inputs);
}

impl<App, T> ApplicationHandler<T> for Cn<App>
where
    App: ApplicationHandler<T> + Tu,
    T: 'static,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app.resumed(event_loop);
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        self.app.new_events(event_loop, cause);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self
            .windows
            .values_mut()
            .for_each(|s| s.about_to_wait());

        self.app.about_to_wait(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if matches!(event, WindowEvent::Destroyed) {
            self.windows.remove(&window_id);
        }

        let state = self
            .windows
            .entry(window_id)
            .or_insert_with(State::new);

        state.window_event(&event);

        if matches!(event, WindowEvent::RedrawRequested) {
            let input = state.new_events();

            self.app.iter(event_loop, input);
        }

        self.app.window_event(event_loop, window_id, event);
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        self
            .windows
            .values_mut()
            .for_each(|s| s.device_event(&event));

        self.app.device_event(event_loop, device_id, event);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.app.suspended(event_loop);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: T) {
        self.app.user_event(event_loop, event);
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        self.app.exiting(event_loop);
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        self.app.memory_warning(event_loop);
    }
}