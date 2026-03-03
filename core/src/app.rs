use std::sync::Arc;

use anyhow::{Ok, Result};
use log::error;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::window::WindowSettings;

type WinitWindow = Arc<winit::window::Window>;

/// This is very much in the winit space

pub struct Application {
    settings: WindowSettings,
    state: Option<State>,
}

impl Application {
    pub fn new(settings: WindowSettings) -> Self {
        Application {
            settings,
            state: None,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Create event loop
        let event_loop = EventLoop::with_user_event().build()?;

        // Control flow setting
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)?;

        Ok(())
    }
}

impl ApplicationHandler<State> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title(&self.settings.title)
            .with_inner_size(LogicalSize::new(self.settings.width, self.settings.height));

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("unable to create winit window"),
        );

        {
            self.state =
                Some(pollster::block_on(State::new(window)).expect("unable to create window state"))
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = &mut self.state else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => {
                let std::result::Result::Ok(width) = new_size.width.try_into() else {
                    error!("resize passed width outside u16 range");
                    return;
                };
                let std::result::Result::Ok(height) = new_size.height.try_into() else {
                    error!("resize passed width outside u16 range");
                    return;
                };
                state.resize(width, height);
            }
            WindowEvent::RedrawRequested => state.render(),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => match (code, key_state.is_pressed()) {
                (KeyCode::Escape, true) => event_loop.exit(),
                _ => {}
            },
            _ => {}
        }
    }
}

// should this be WindowState?
// should it implement trait Window?
pub struct State {
    window: WinitWindow,
}

impl State {
    pub async fn new(window: WinitWindow) -> Result<Self> {
        Ok(State { window })
    }

    pub fn resize(&mut self, _width: u16, _height: u16) {
        // something
    }

    pub fn render(&mut self) {
        self.window.request_redraw();

        // and more
    }
}
