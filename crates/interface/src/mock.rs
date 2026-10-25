extern crate alloc;

use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::Cell;

use crate::{ErrorKind, ErrorType, Event, Input, KeyEvent, KeyNumber, KeyState, Output};

/// 기기에 일어나는 일을 순서대로 적은 시나리오의 한 단계입니다.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    /// 키를 누릅니다. (디바운싱이 끝난 것으로 간주)
    Press(u8),
    /// 키를 뗍니다.
    Release(u8),
    /// 호스트가 장치 설정을 마칩니다. 로직에는 `Event::Connected`가 전달되고,
    /// 이후 `send_report`는 성공합니다.
    Connect,
    /// 호스트와의 연결이 끊깁니다. 로직에는 아무 이벤트도 전달되지 않고,
    /// 이후 `send_report`는 `ErrorKind::NotConnected`를 반환합니다.
    Disconnect,
}

/// 연결 상태를 공유하는 입력·출력 목 한 쌍을 만듭니다.
///
/// 실제 기기와 같이 처음에는 연결되지 않은 상태로 시작합니다.
pub fn device<T>(script: impl IntoIterator<Item = Action>) -> (MockInput, MockOutput<T>) {
    let connected = Rc::new(Cell::new(false));
    let input = MockInput {
        script: script.into_iter().collect(),
        connected: Rc::clone(&connected),
    };
    let output = MockOutput {
        reports: Vec::new(),
        connected,
    };
    (input, output)
}

/// 시나리오를 순서대로 재생하는 입력 목입니다.
pub struct MockInput {
    script: VecDeque<Action>,
    connected: Rc<Cell<bool>>,
}

impl MockInput {
    /// 더 이상 로직에 전달될 이벤트가 없으면 `true`입니다.
    /// (남은 단계가 `Disconnect`뿐인 경우도 포함)
    pub fn is_empty(&self) -> bool {
        self.script.iter().all(|a| *a == Action::Disconnect)
    }
}

impl Input for MockInput {
    async fn next_event(&mut self) -> Event {
        loop {
            let Some(action) = self.script.pop_front() else {
                return core::future::pending().await;
            };
            let key_event = |i, status| {
                Event::Key(KeyEvent {
                    key: KeyNumber(i),
                    status,
                })
            };
            match action {
                Action::Press(i) => return key_event(i, KeyState::Pressed),
                Action::Release(i) => return key_event(i, KeyState::Released),
                Action::Connect => {
                    self.connected.set(true);
                    return Event::Connected;
                }
                // 끊김은 로직에 전달하지 않고 상태만 바꾼 뒤 다음 단계로 넘어갑니다.
                Action::Disconnect => self.connected.set(false),
            }
        }
    }
}

/// 연결된 동안 `send_report`로 받은 집합을 그대로 기록하는 출력 목입니다.
pub struct MockOutput<T> {
    reports: Vec<Vec<T>>,
    connected: Rc<Cell<bool>>,
}

impl<T> MockOutput<T> {
    /// 지금까지 성공적으로 전송된 report를 전송 순서대로 돌려줍니다.
    pub fn reports(&self) -> &[Vec<T>] {
        &self.reports
    }
}

impl<T> ErrorType for MockOutput<T> {
    type Error = ErrorKind;
}

impl<T: Clone> Output for MockOutput<T> {
    type Item = T;

    async fn send_report(&mut self, pressed: &[T]) -> Result<(), ErrorKind> {
        if !self.connected.get() {
            return Err(ErrorKind::NotConnected);
        }
        self.reports.push(pressed.to_vec());
        Ok(())
    }
}
