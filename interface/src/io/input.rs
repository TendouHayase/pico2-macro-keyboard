use crate::{
    event::Event,
    key::{KeyNumber, KeyState},
};

pub trait Input {
    fn next_event(&mut self) -> impl Future<Output = Event>;
}

pub struct KeyEvent {
    pub key: KeyNumber,
    pub status: KeyState,
}
