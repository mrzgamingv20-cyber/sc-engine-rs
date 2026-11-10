use std::sync::Mutex;
use super::key::Key;

pub struct Keyboard;

static KEYS_DOWN: Mutex<[bool; Key::COUNT]> = Mutex::new([false; Key::COUNT]);
static KEYS_DOWN_ONCE: Mutex<[bool; Key::COUNT]> = Mutex::new([false; Key::COUNT]);
static KEYS_DOWN_REPEAT: Mutex<[f64; Key::COUNT]> = Mutex::new([0.0; Key::COUNT]);
static LAST_KEY: Mutex<Option<Key>> = Mutex::new(None);
static LAST_CHAR: Mutex<Option<char>> = Mutex::new(None);
static IS_KEYBOARD_VISIBLE: Mutex<bool> = Mutex::new(false);

pub static KEY_FIRST_REPEAT_TIME: f64 = 0.3;
pub static KEY_NEXT_REPEAT_TIME: f64 = 0.04;

impl Keyboard {
    pub fn last_key() -> Option<Key> {
        *LAST_KEY.lock().unwrap()
    }

    pub fn last_char() -> Option<char> {
        *LAST_CHAR.lock().unwrap()
    }

    pub fn is_keyboard_visible() -> bool {
        *IS_KEYBOARD_VISIBLE.lock().unwrap()
    }

    pub fn set_keyboard_visible(v: bool) {
        *IS_KEYBOARD_VISIBLE.lock().unwrap() = v;
    }

    pub fn is_key_down(key: Key) -> bool {
        KEYS_DOWN.lock().unwrap()[key.index()]
    }

    pub fn is_key_down_once(key: Key) -> bool {
        KEYS_DOWN_ONCE.lock().unwrap()[key.index()]
    }

    pub fn is_key_down_repeat(key: Key, frame_start_time: f64) -> bool {
        let num = KEYS_DOWN_REPEAT.lock().unwrap()[key.index()];
        num < 0.0 || (num != 0.0 && frame_start_time >= num)
    }

    pub fn clear() {
        *LAST_KEY.lock().unwrap() = None;
        *LAST_CHAR.lock().unwrap() = None;
        KEYS_DOWN.lock().unwrap().fill(false);
        KEYS_DOWN_ONCE.lock().unwrap().fill(false);
        KEYS_DOWN_REPEAT.lock().unwrap().fill(0.0);
    }

    pub fn process_key_down(key: Key, is_active: bool) -> bool {
        if !is_active || Self::is_keyboard_visible() {
            return false;
        }
        *LAST_KEY.lock().unwrap() = Some(key);
        let mut down = KEYS_DOWN.lock().unwrap();
        let idx = key.index();
        if !down[idx] {
            down[idx] = true;
            KEYS_DOWN_ONCE.lock().unwrap()[idx] = true;
            KEYS_DOWN_REPEAT.lock().unwrap()[idx] = -1.0;
        }
        true
    }

    pub fn process_key_up(key: Key, is_active: bool) -> bool {
        if !is_active || Self::is_keyboard_visible() {
            return false;
        }
        KEYS_DOWN.lock().unwrap()[key.index()] = false;
        true
    }

    pub fn process_character_entered(ch: char, is_active: bool) -> bool {
        if !is_active || Self::is_keyboard_visible() {
            return false;
        }
        *LAST_CHAR.lock().unwrap() = Some(ch);
        true
    }

    pub fn after_frame(frame_start_time: f64) {
        *LAST_KEY.lock().unwrap() = None;
        *LAST_CHAR.lock().unwrap() = None;
        KEYS_DOWN_ONCE.lock().unwrap().fill(false);

        let down = KEYS_DOWN.lock().unwrap();
        let mut repeat = KEYS_DOWN_REPEAT.lock().unwrap();
        for i in 0..Key::COUNT {
            if down[i] {
                if repeat[i] < 0.0 {
                    repeat[i] = frame_start_time + KEY_FIRST_REPEAT_TIME;
                } else if frame_start_time >= repeat[i] {
                    repeat[i] = frame_start_time.max(repeat[i] + KEY_NEXT_REPEAT_TIME);
                }
            } else {
                repeat[i] = 0.0;
            }
        }
    }
}
