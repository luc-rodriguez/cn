mod state;

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