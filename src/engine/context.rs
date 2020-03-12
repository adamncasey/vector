use crate::Event;
use getset::{Getters, Setters};

#[derive(Default)]
pub(super) struct EngineContext {
    pub(super) event: Option<Event>,
}

impl EngineContext {
    pub(super) fn new(event: impl Into<Option<Event>>) -> Self {
        Self {
            event: event.into(),
        }
    }
}
