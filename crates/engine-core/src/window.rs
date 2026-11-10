use crate::window_mode::WindowMode;

/// Window position and size
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}

/// Window size
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WindowSize {
    pub width: i32,
    pub height: i32,
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

/// Window display info
#[derive(Debug, Clone)]
pub struct WindowDisplayInfo {
    pub bounds: Rectangle,
    pub client_bounds: Rectangle,
    pub position: WindowPosition,
    pub size: WindowSize,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
    pub is_visible: bool,
}

use crate::math::rectangle::Rectangle;

/// Window abstraction
pub struct Window {
    pub title: String,
    pub mode: WindowMode,
    pub position: WindowPosition,
    pub size: WindowSize,
    pub is_visible: bool,
    pub is_cursor_visible: bool,
    pub is_cursor_grabbed: bool,
    pub display_index: usize,
}

impl Window {
    pub fn new(title: &str, mode: WindowMode, width: i32, height: i32) -> Self {
        Window {
            title: title.to_string(),
            mode,
            position: WindowPosition::default(),
            size: WindowSize { width, height },
            is_visible: true,
            is_cursor_visible: true,
            is_cursor_grabbed: false,
            display_index: 0,
        }
    }

    pub fn client_size(&self) -> WindowSize {
        self.size
    }

    pub fn client_position(&self) -> WindowPosition {
        self.position
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn set_mode(&mut self, mode: WindowMode) {
        self.mode = mode;
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = WindowPosition { x, y };
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.size = WindowSize { width, height };
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.is_cursor_visible = visible;
    }

    pub fn set_cursor_grabbed(&mut self, grabbed: bool) {
        self.is_cursor_grabbed = grabbed;
    }
}
