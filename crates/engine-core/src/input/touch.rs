use std::sync::Mutex;
use super::touch_location::TouchLocation;
use super::touch_location_state::TouchLocationState;
use crate::math::vector2::Vector2;

pub struct Touch;

static TOUCH_LOCATIONS: Mutex<Vec<TouchLocation>> = Mutex::new(Vec::new());
static IS_TOUCHED: Mutex<bool> = Mutex::new(false);

impl Touch {
    pub fn touch_locations() -> std::sync::MutexGuard<'static, Vec<TouchLocation>> {
        TOUCH_LOCATIONS.lock().unwrap()
    }

    pub fn is_touched() -> bool {
        *IS_TOUCHED.lock().unwrap()
    }

    pub fn clear() {
        TOUCH_LOCATIONS.lock().unwrap().clear();
    }

    pub fn process_touch_pressed(id: i32, position: Vector2, is_active: bool) {
        Self::process_touch_moved(id, position, is_active);
    }

    pub fn process_touch_moved(id: i32, position: Vector2, is_active: bool) {
        if !is_active || super::keyboard::Keyboard::is_keyboard_visible() {
            return;
        }
        *IS_TOUCHED.lock().unwrap() = true;
        let mut locations = TOUCH_LOCATIONS.lock().unwrap();
        if let Some(idx) = locations.iter().position(|t| t.id == id) {
            if locations[idx].state == TouchLocationState::Moved {
                locations[idx].position = position;
                locations[idx].state = TouchLocationState::Moved;
            }
        } else {
            locations.push(TouchLocation {
                id,
                position,
                state: TouchLocationState::Pressed,
                release_queued: false,
            });
        }
    }

    pub fn process_touch_released(id: i32, position: Vector2, is_active: bool) {
        if !is_active || super::keyboard::Keyboard::is_keyboard_visible() {
            return;
        }
        let mut locations = TOUCH_LOCATIONS.lock().unwrap();
        if let Some(idx) = locations.iter().position(|t| t.id == id) {
            if locations[idx].state == TouchLocationState::Pressed {
                locations[idx].state = TouchLocationState::Pressed;
                locations[idx].release_queued = true;
            } else {
                locations[idx].position = position;
                locations[idx].state = TouchLocationState::Released;
            }
        }
    }

    pub fn after_frame() {
        let mut locations = TOUCH_LOCATIONS.lock().unwrap();
        let mut i = 0;
        while i < locations.len() {
            if locations[i].state == TouchLocationState::Released {
                locations.remove(i);
                continue;
            }
            if locations[i].release_queued {
                locations[i].state = TouchLocationState::Released;
                locations[i].release_queued = false;
            } else if locations[i].state == TouchLocationState::Pressed {
                locations[i].state = TouchLocationState::Moved;
            }
            i += 1;
        }
    }
}
