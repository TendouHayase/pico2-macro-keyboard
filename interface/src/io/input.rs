use crate::{
    event::Event,
    key::{KeyNumber, KeyState},
};

pub trait Input {
    /// Waits for the next device event.
    ///
    /// Key events are debounced, delivered in the order they occurred,
    /// and reflect the physical state of the keys. A key that is already
    /// pressed at power-on produces no event until it has been released once.
    ///
    /// [`Event::Connected`] is produced each time the host finishes
    /// configuring the device, including the first connection after
    /// power-on. Key events around it are not altered, so after `Connected`
    /// the caller may receive a release event for a key it does not
    /// consider pressed.
    ///
    /// # Returns
    ///
    /// - `impl Future<Output = Event>` - Completes when the next event is
    ///   available. Never completes if no further event occurs.
    ///
    /// # Event loss
    ///
    /// No event is lost as long as the caller keeps calling `next_event`
    /// without stalling indefinitely. The implementation buffers events
    /// while the caller is busy (for example, waiting in
    /// [`Output::send_report`](crate::Output::send_report)) and must size
    /// that buffer so it cannot overflow under this condition.
    fn next_event(&mut self) -> impl Future<Output = Event>;
}

pub struct KeyEvent {
    pub key: KeyNumber,
    pub status: KeyState,
}
