use core::pin::pin;
use core::task::{Context, Poll, Waker};

use interface::mock::{self, Action::*};
use interface::{Error, ErrorKind, Event, Input, KeyNumber, KeyState, Keymap, Output};

// ---------------------------------------------------------------------------
// 테스트용 최소 실행기
// ---------------------------------------------------------------------------

/// 목의 future는 즉시 완료되므로 한 번만 poll합니다.
/// `Pending`이 나오면 영원히 멈추는 대신 바로 실패시켜서 원인을 드러냅니다.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(output) => output,
        Poll::Pending => {
            panic!("future가 즉시 완료되지 않았습니다 (시나리오가 소진되었을 수 있음)")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum FKey {
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
}

struct ToyKeyMap;

impl Keymap for ToyKeyMap {
    type Code = FKey;

    fn translate(&self, key: KeyNumber) -> Option<FKey> {
        use FKey::*;
        [F14, F15, F16, F17, F18, F19, F20]
            .get(key.0 as usize)
            .copied()
    }
}

/// 실제 펌웨어에서는 `step`을 무한 반복하는 루프가 따로 있고,
/// 테스트는 루프 대신 `step`을 이벤트 수만큼만 호출합니다.
///
/// 1. `Connected`를 받으면 눌린 집합을 비운다. (report는 보내지 않음)
/// 2. 집합이 실제로 바뀔 때만 report를 보낸다.
///    따라서 집합에 없는 키의 `Released`는 무시된다.
/// 3. 전송 실패는 무시하고 다음 이벤트로 넘어간다.
struct ToyLogic<C> {
    pressed: Vec<C>,
}

impl<C: Copy + Eq> ToyLogic<C> {
    fn new() -> Self {
        Self {
            pressed: Vec::new(),
        }
    }

    async fn step<I, M, O>(&mut self, input: &mut I, map: &M, output: &mut O)
    where
        I: Input,
        M: Keymap<Code = C>,
        O: Output<Item = C>,
    {
        let key_event = match input.next_event().await {
            Event::Connected => {
                self.pressed.clear(); // 1
                return;
            }
            Event::Key(key_event) => key_event,
            _ => return,
        };

        let Some(code) = map.translate(key_event.key) else {
            return; // 매핑 없는 키는 무시
        };

        let changed = match key_event.status {
            KeyState::Pressed if !self.pressed.contains(&code) => {
                self.pressed.push(code);
                true
            }
            KeyState::Released if self.pressed.contains(&code) => {
                self.pressed.retain(|&c| c != code);
                true
            }
            _ => false, // 2
        };
        if !changed {
            return;
        }

        if let Err(e) = output.send_report(&self.pressed).await {
            match e.kind() {
                // 3: 연결되면 집합을 비우고 새로 시작하므로 그냥 버린다.
                ErrorKind::NotConnected => {}
                ErrorKind::Other => {}
            }
        }
    }
}

/// 시나리오를 끝까지 재생하고, 성공적으로 전송된 report 목록을 돌려줍니다.
/// report 안의 순서는 계약상 의미가 없으므로 정렬해서 비교합니다.
fn run(script: impl IntoIterator<Item = mock::Action>) -> Vec<Vec<FKey>> {
    let (mut input, mut output) = mock::device(script);
    let map = ToyKeyMap;
    let mut logic = ToyLogic::new();

    while !input.is_empty() {
        block_on(logic.step(&mut input, &map, &mut output));
    }

    output
        .reports()
        .iter()
        .map(|r| {
            let mut r = r.clone();
            r.sort();
            r
        })
        .collect()
}

use FKey::*;

/// 요구사항: 한 키를 누른 채 다른 키를 누르고 떼도, 먼저 누른 키는 유지된다.
/// 요구사항: report에는 그 시점에 눌려 있는 키 전체가 담긴다.
#[test]
fn held_key_survives_other_key_release() {
    let reports = run([Connect, Press(0), Press(2), Release(0)]);
    assert_eq!(reports, [vec![F14], vec![F14, F16], vec![F16]]);
}

/// 정책: 재연결 시점에 눌려 있던 키는 뗄 때까지 무시한다.
#[test]
fn key_held_across_reconnect_is_ignored() {
    let reports = run([
        Connect,
        Press(0),
        Disconnect,
        Connect,
        Press(1),   // 0번은 여전히 눌려 있지만 report에 들어가지 않는다
        Release(0), // 로직이 모르는 키 → 무시, report 없음
        Release(1),
    ]);
    assert_eq!(reports, [vec![F14], vec![F15], vec![]]);
}

/// 정책: 끊긴 동안 뗀 키가 재연결 후 눌린 채로 남지 않는다.
#[test]
fn key_released_while_disconnected_does_not_stick() {
    let reports = run([
        Connect,
        Press(0),
        Disconnect,
        Release(0), // 전송 실패 → 무시
        Connect,
        Press(1),
    ]);
    assert_eq!(reports, [vec![F14], vec![F15]]);
}

/// 정책: 첫 연결 전에 눌린 키도 뗄 때까지 무시한다.
#[test]
fn key_pressed_before_first_connect_is_ignored() {
    let reports = run([
        Press(0), // 아직 연결 전 → 전송 실패, 무시
        Connect,
        Press(1),
        Release(0), // 로직이 모르는 키 → 무시
        Release(1),
    ]);
    assert_eq!(reports, [vec![F15], vec![]]);
}
