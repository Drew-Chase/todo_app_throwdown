mod app;

use crate::app::TodoApp;
use color_eyre::Result;
use tracing::info;

pub fn run() -> Result<()> {
    info!("Starting todo app v{}#{}", TodoApp::VERSION, TodoApp::BUILD);
    iced::application(TodoApp::new, TodoApp::update, TodoApp::view)
        .title(TodoApp::TITLE)
        .window(TodoApp::window_settings())
        .centered()
        .theme(iced::Theme::Dark)
        .run()?;
    Ok(())
}
