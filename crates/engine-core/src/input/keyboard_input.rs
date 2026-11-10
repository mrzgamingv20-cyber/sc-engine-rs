use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub struct KeyboardInput;

static CHARS: Mutex<Vec<char>> = Mutex::new(Vec::new());
static DELETE_PRESSED: AtomicBool = AtomicBool::new(false);

impl KeyboardInput {
    pub fn get_input() -> String {
        let mut chars = CHARS.lock().unwrap();
        if !chars.is_empty() {
            let s: String = chars.iter().collect();
            chars.clear();
            s
        } else {
            String::new()
        }
    }

    pub fn add_char(c: char) {
        CHARS.lock().unwrap().push(c);
    }

    pub fn delete_pressed() -> bool {
        DELETE_PRESSED.swap(false, Ordering::Relaxed)
    }

    pub fn set_delete_pressed(v: bool) {
        DELETE_PRESSED.store(v, Ordering::Relaxed);
    }
}
