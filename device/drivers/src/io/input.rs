use interface::{Event, Input, KeyNumber, KeyState};

pub struct RawInput {
    id: KeyNumber,
    status: KeyState,
}

impl Input for RawInput {
    async fn next_event(&mut self) -> Event {
        todo!()
    }
}
