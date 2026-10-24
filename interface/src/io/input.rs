use crate::{io::Level, key::KeyNumber};

pub trait Input {
    fn next_event(&mut self) -> impl Future<Output = KeyEvent>;
}

pub struct KeyEvent {
    pub key: KeyNumber,
    pub lvl: Level,
}
