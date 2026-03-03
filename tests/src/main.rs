use anyhow::Result;
use engine_core::{Application, WindowSettings};

fn main() -> Result<()> {
    let window_settings = WindowSettings::default();
    let mut app = Application::new(window_settings);

    app.run()?;

    println!("Done");
    Ok(())
}
