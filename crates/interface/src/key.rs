#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyNumber(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    Pressed,
    Released,
}
