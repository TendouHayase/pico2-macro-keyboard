extern crate alloc;

use alloc::collections::VecDeque;

use crate::{io::Input, io::KeyEvent};

/// This is the input for rotating the pre-inserted events in order.
///
/// To keep the contract ("wait until the next event"),
/// When the event is exhausted, `next_event` will never complete.
/// The test should determine when to terminate by checking `is_empty`.
pub struct MockInput {
    events: VecDeque<KeyEvent>,
}

impl MockInput {
    pub fn new(events: impl IntoIterator<Item = KeyEvent>) -> Self {
        Self {
            events: events.into_iter().collect(),
        }
    }

    /// If there are no events yet consumed, it is `true`.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Input for MockInput {
    async fn next_event(&mut self) -> KeyEvent {
        match self.events.pop_front() {
            Some(event) => event,
            None => core::future::pending().await,
        }
    }
}
