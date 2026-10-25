use crate::key::KeyNumber;

pub trait Keymap {
    type Code: Copy + Eq;

    fn translate(&self, key: KeyNumber) -> Option<Self::Code>;
}
