use std::sync::Mutex;
use super::mouse_button::MouseButton;
use super::mouse_event::MouseEvent;
use super::mouse_button_event::MouseButtonEvent;
use crate::math::point2::Point2;

pub struct Mouse;

static BUTTONS_DOWN: Mutex<[bool; MouseButton::COUNT]> = Mutex::new([false; MouseButton::COUNT]);
static BUTTONS_DOWN_FRAME: Mutex<[i32; MouseButton::COUNT]> = Mutex::new([0; MouseButton::COUNT]);
static BUTTONS_DELAYED_UP: Mutex<[bool; MouseButton::COUNT]> = Mutex::new([false; MouseButton::COUNT]);
static BUTTONS_DOWN_ONCE: Mutex<[bool; MouseButton::COUNT]> = Mutex::new([false; MouseButton::COUNT]);
static BUTTONS_UP_ONCE: Mutex<[bool; MouseButton::COUNT]> = Mutex::new([false; MouseButton::COUNT]);
static MOUSE_POSITION: Mutex<Option<Point2>> = Mutex::new(None);
static MOUSE_MOVEMENT: Mutex<Point2> = Mutex::new(Point2 { x: 0, y: 0 });
static MOUSE_WHEEL_MOVEMENT: Mutex<i32> = Mutex::new(0);
static MOUSE_WHEEL_MOVEMENT_X: Mutex<i32> = Mutex::new(0);
static IS_MOUSE_VISIBLE: Mutex<bool> = Mutex::new(true);

impl Mouse {
    pub fn mouse_movement() -> Point2 {
        *MOUSE_MOVEMENT.lock().unwrap()
    }

    pub fn mouse_wheel_movement() -> i32 {
        *MOUSE_WHEEL_MOVEMENT.lock().unwrap()
    }

    pub fn mouse_wheel_movement_x() -> i32 {
        *MOUSE_WHEEL_MOVEMENT_X.lock().unwrap()
    }

    pub fn mouse_position() -> Option<Point2> {
        *MOUSE_POSITION.lock().unwrap()
    }

    pub fn set_mouse_position(x: i32, y: i32) {
        *MOUSE_POSITION.lock().unwrap() = Some(Point2::new(x, y));
    }

    pub fn is_mouse_visible() -> bool {
        *IS_MOUSE_VISIBLE.lock().unwrap()
    }

    pub fn set_mouse_visible(v: bool) {
        *IS_MOUSE_VISIBLE.lock().unwrap() = v;
    }

    pub fn is_mouse_button_down(button: MouseButton) -> bool {
        BUTTONS_DOWN.lock().unwrap()[button as usize]
    }

    pub fn is_mouse_button_down_once(button: MouseButton) -> bool {
        BUTTONS_DOWN_ONCE.lock().unwrap()[button as usize]
    }

    pub fn is_mouse_button_up_once(button: MouseButton) -> bool {
        BUTTONS_UP_ONCE.lock().unwrap()[button as usize]
    }

    pub fn clear() {
        BUTTONS_DOWN.lock().unwrap().fill(false);
        BUTTONS_DOWN_FRAME.lock().unwrap().fill(0);
        BUTTONS_DELAYED_UP.lock().unwrap().fill(false);
        BUTTONS_DOWN_ONCE.lock().unwrap().fill(false);
        BUTTONS_UP_ONCE.lock().unwrap().fill(false);
    }

    pub fn process_mouse_down(button: MouseButton, position: Point2, is_active: bool, frame_index: i32, window_scale: f32) {
        if is_active && !super::keyboard::Keyboard::is_keyboard_visible() {
            if MOUSE_POSITION.lock().unwrap().is_none() {
                Self::process_mouse_move(position, is_active, window_scale);
            }
            let idx = button as usize;
            BUTTONS_DOWN.lock().unwrap()[idx] = true;
            BUTTONS_DOWN_FRAME.lock().unwrap()[idx] = frame_index;
            BUTTONS_DOWN_ONCE.lock().unwrap()[idx] = true;
            let scaled = Point2::new(
                (position.x as f32 * window_scale).round() as i32,
                (position.y as f32 * window_scale).round() as i32,
            );
            *MOUSE_POSITION.lock().unwrap() = Some(scaled);
        }
    }

    pub fn process_mouse_up(button: MouseButton, position: Point2, is_active: bool, frame_index: i32, window_scale: f32) {
        if is_active && !super::keyboard::Keyboard::is_keyboard_visible() {
            if MOUSE_POSITION.lock().unwrap().is_none() {
                Self::process_mouse_move(position, is_active, window_scale);
            }
            let idx = button as usize;
            if BUTTONS_DOWN.lock().unwrap()[idx] && frame_index == BUTTONS_DOWN_FRAME.lock().unwrap()[idx] {
                BUTTONS_DELAYED_UP.lock().unwrap()[idx] = true;
            } else {
                BUTTONS_DOWN.lock().unwrap()[idx] = false;
                BUTTONS_UP_ONCE.lock().unwrap()[idx] = true;
            }
            let scaled = Point2::new(
                (position.x as f32 * window_scale).round() as i32,
                (position.y as f32 * window_scale).round() as i32,
            );
            *MOUSE_POSITION.lock().unwrap() = Some(scaled);
        }
    }

    pub fn process_mouse_move(position: Point2, is_active: bool, window_scale: f32) {
        if is_active && !super::keyboard::Keyboard::is_keyboard_visible() && Self::is_mouse_visible() {
            let scaled = Point2::new(
                (position.x as f32 * window_scale).round() as i32,
                (position.y as f32 * window_scale).round() as i32,
            );
            *MOUSE_POSITION.lock().unwrap() = Some(scaled);
        }
    }

    pub fn process_mouse_wheel_xy(x: f32, y: f32, is_active: bool) {
        if is_active && !super::keyboard::Keyboard::is_keyboard_visible() {
            *MOUSE_WHEEL_MOVEMENT.lock().unwrap() += (120.0 * y) as i32;
            *MOUSE_WHEEL_MOVEMENT_X.lock().unwrap() += (120.0 * x) as i32;
        }
    }

    pub fn after_frame() {
        BUTTONS_DOWN_ONCE.lock().unwrap().fill(false);
        {
            let mut delayed = BUTTONS_DELAYED_UP.lock().unwrap();
            let mut down = BUTTONS_DOWN.lock().unwrap();
            let mut up_once = BUTTONS_UP_ONCE.lock().unwrap();
            for i in 0..MouseButton::COUNT {
                if delayed[i] {
                    delayed[i] = false;
                    down[i] = false;
                    up_once[i] = true;
                } else {
                    up_once[i] = false;
                }
            }
        }
        if !Self::is_mouse_visible() {
            *MOUSE_POSITION.lock().unwrap() = None;
        }
        *MOUSE_WHEEL_MOVEMENT.lock().unwrap() = 0;
        *MOUSE_WHEEL_MOVEMENT_X.lock().unwrap() = 0;
    }
}
