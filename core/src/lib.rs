use glam::Vec3A;

mod app;
mod window;

pub use app::Application;
pub use window::WindowSettings;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vec3A,
    pub rotation: Vec3A,
    pub scale: Vec3A,
}

impl Transform {
    pub fn identity() -> Self {
        Transform {
            position: Vec3A::ZERO,
            rotation: Vec3A::ZERO,
            scale: Vec3A::ONE,
        }
    }
}
