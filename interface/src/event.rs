use crate::io::KeyEvent;

pub enum Event {
    Connected,
    Disconnected,
    Key(KeyEvent),
}
