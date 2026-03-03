use glam::UVec2;

/// Configuration options for creating a new window.
/// This struct is plain data—easy to serialize or load from a file.
#[derive(Debug, Clone)]
pub struct WindowSettings {
    pub title: String,
    pub width: u16, // does this need to be u32?
    pub height: u16,
    pub vsync: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            title: "Darcsyde Game Engine".to_string(),
            width: 1280,
            height: 720,
            vsync: true,
        }
    }
}

/// A trait representing the platform=agnostic capabilities of a window
/// The engine holds a Box<dyn Window> rather than a specific struct
#[allow(dead_code)]
pub trait Window {
    /// Returns the logical size of the window (width, height)
    fn size(&self) -> UVec2;

    /// Sets the window title dynamically
    fn set_title(&mut self, title: &str);

    /// Toggles the mouse cursor visibility and lock state
    fn set_cursor_grab(&mut self, grabbed: bool);

    /// Asks the OS to redraw the window
    fn request_redraw(&self);
}
