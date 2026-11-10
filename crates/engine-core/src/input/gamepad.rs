use std::sync::Mutex;
use super::gamepad_button::GamePadButton;
use super::gamepad_stick::GamePadStick;
use super::gamepad_trigger::GamePadTrigger;
use crate::math::vector2::Vector2;

pub struct GamePad;

pub struct GamePadState {
    pub is_connected: bool,
    pub sticks: [Vector2; 2],
    pub triggers: [f32; 2],
    pub last_triggers: [f32; 2],
    pub buttons: [bool; GamePadButton::COUNT],
    pub last_buttons: [bool; GamePadButton::COUNT],
    pub buttons_repeat: [f64; GamePadButton::COUNT],
}

impl GamePadState {
    const fn new() -> Self {
        GamePadState {
            is_connected: false,
            sticks: [Vector2::ZERO; 2],
            triggers: [0.0; 2],
            last_triggers: [0.0; 2],
            buttons: [false; GamePadButton::COUNT],
            last_buttons: [false; GamePadButton::COUNT],
            buttons_repeat: [0.0; GamePadButton::COUNT],
        }
    }
}

static STATES: Mutex<[GamePadState; 4]> = Mutex::new([
    GamePadState::new(),
    GamePadState::new(),
    GamePadState::new(),
    GamePadState::new(),
]);

pub static BUTTON_FIRST_REPEAT_TIME: f64 = 0.2;
pub static BUTTON_NEXT_REPEAT_TIME: f64 = 0.04;

impl GamePad {
    pub fn is_connected(index: usize) -> bool {
        assert!(index < 4, "gamePadIndex out of range");
        STATES.lock().unwrap()[index].is_connected
    }

    pub fn get_stick_position(index: usize, stick: GamePadStick, dead_zone: f32) -> Vector2 {
        assert!((0.0..1.0).contains(&dead_zone), "deadZone out of range");
        if Self::is_connected(index) {
            let result = STATES.lock().unwrap()[index].sticks[stick as usize];
            if dead_zone > 0.0 {
                let num = result.length();
                if num > 0.0 {
                    let num2 = Self::apply_dead_zone(num, dead_zone);
                    return result * (num2 / num);
                }
            }
            return result;
        }
        Vector2::ZERO
    }

    pub fn get_trigger_position(index: usize, trigger: GamePadTrigger, dead_zone: f32) -> f32 {
        assert!((0.0..1.0).contains(&dead_zone), "deadZone out of range");
        if Self::is_connected(index) {
            Self::apply_dead_zone(STATES.lock().unwrap()[index].triggers[trigger as usize], dead_zone)
        } else {
            0.0
        }
    }

    pub fn is_trigger_down(index: usize, trigger: GamePadTrigger, dead_zone: f32, threshold: f32) -> bool {
        assert!((0.0..1.0).contains(&dead_zone), "deadZone out of range");
        if !Self::is_connected(index) {
            return false;
        }
        let value = Self::apply_dead_zone(STATES.lock().unwrap()[index].triggers[trigger as usize], dead_zone);
        value >= threshold
    }

    pub fn is_button_down(index: usize, button: GamePadButton) -> bool {
        Self::is_connected(index) && STATES.lock().unwrap()[index].buttons[button.index()]
    }

    pub fn is_button_down_once(index: usize, button: GamePadButton) -> bool {
        if !Self::is_connected(index) {
            return false;
        }
        let state = STATES.lock().unwrap();
        let is_modifier = Self::is_modifier_button(button);
        if is_modifier {
            return !state[index].buttons[button.index()] && state[index].last_buttons[button.index()];
        }
        state[index].buttons[button.index()] && !state[index].last_buttons[button.index()]
    }

    pub fn is_button_down_repeat(index: usize, button: GamePadButton, frame_start_time: f64) -> bool {
        if Self::is_connected(index) {
            let state = STATES.lock().unwrap();
            if state[index].buttons[button.index()] && !state[index].last_buttons[button.index()] {
                return true;
            }
            let num = state[index].buttons_repeat[button.index()];
            num != 0.0 && frame_start_time >= num
        } else {
            false
        }
    }

    pub fn is_any_modifier_key_holding(index: usize, threshold: f32) -> bool {
        if !Self::is_connected(index) {
            return false;
        }
        let state = STATES.lock().unwrap();
        state[index].triggers[0] >= threshold
            || state[index].triggers[1] >= threshold
            || state[index].buttons[GamePadButton::LeftShoulder.index()]
            || state[index].buttons[GamePadButton::RightShoulder.index()]
    }

    pub fn is_modifier_button(button: GamePadButton) -> bool {
        matches!(button, GamePadButton::LeftShoulder | GamePadButton::RightShoulder)
    }

    pub fn clear() {
        let mut states = STATES.lock().unwrap();
        for state in states.iter_mut() {
            state.sticks = [Vector2::ZERO; 2];
            state.triggers = [0.0; 2];
            state.buttons = [false; GamePadButton::COUNT];
            state.buttons_repeat = [0.0; GamePadButton::COUNT];
        }
    }

    pub fn after_frame(frame_start_time: f64) {
        let mut states = STATES.lock().unwrap();
        for state in states.iter_mut() {
            for j in 0..GamePadButton::COUNT {
                if state.buttons[j] {
                    if !state.last_buttons[j] {
                        state.buttons_repeat[j] = frame_start_time + BUTTON_FIRST_REPEAT_TIME;
                    } else if frame_start_time >= state.buttons_repeat[j] {
                        state.buttons_repeat[j] = frame_start_time.max(state.buttons_repeat[j] + BUTTON_NEXT_REPEAT_TIME);
                    }
                } else {
                    state.buttons_repeat[j] = 0.0;
                }
                state.last_buttons[j] = state.buttons[j];
            }
            for k in 0..2 {
                state.last_triggers[k] = state.triggers[k];
            }
        }
    }

    pub fn handle_key_down(index: usize, button: GamePadButton) {
        if index >= 4 {
            return;
        }
        let mut states = STATES.lock().unwrap();
        if button.index() < GamePadButton::COUNT {
            states[index].buttons[button.index()] = true;
            match button {
                GamePadButton::LeftShoulder => states[index].triggers[0] = 1.0,
                GamePadButton::RightShoulder => states[index].triggers[1] = 1.0,
                _ => {}
            }
        }
    }

    pub fn handle_key_up(index: usize, button: GamePadButton) {
        if index >= 4 {
            return;
        }
        let mut states = STATES.lock().unwrap();
        if button.index() < GamePadButton::COUNT {
            states[index].buttons[button.index()] = false;
            match button {
                GamePadButton::LeftShoulder => states[index].triggers[0] = 0.0,
                GamePadButton::RightShoulder => states[index].triggers[1] = 0.0,
                _ => {}
            }
        }
    }

    pub fn handle_stick(index: usize, stick: GamePadStick, x: f32, y: f32) {
        if index < 4 {
            STATES.lock().unwrap()[index].sticks[stick as usize] = Vector2::new(x, y);
        }
    }

    pub fn handle_trigger(index: usize, trigger: GamePadTrigger, value: f32) {
        if index < 4 {
            STATES.lock().unwrap()[index].triggers[trigger as usize] = value;
        }
    }

    pub fn connect(index: usize) {
        if index < 4 {
            STATES.lock().unwrap()[index].is_connected = true;
        }
    }

    pub fn disconnect(index: usize) {
        if index < 4 {
            STATES.lock().unwrap()[index].is_connected = false;
        }
    }

    pub fn apply_dead_zone(value: f32, dead_zone: f32) -> f32 {
        let sign = if value >= 0.0 { 1.0 } else { -1.0 };
        sign * (value.abs() - dead_zone).max(0.0) / (1.0 - dead_zone)
    }
}
