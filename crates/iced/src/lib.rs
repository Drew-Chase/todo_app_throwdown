mod app;
mod math;

use crate::app::TodoApp;
use color_eyre::Result;
use tracing::info;
use app::theme::color;

pub fn run() -> Result<()> {
    info!("Starting todo app v{}#{}", TodoApp::VERSION, TodoApp::BUILD);
    iced::application(TodoApp::new, TodoApp::update, TodoApp::view)
        .title(TodoApp::TITLE)
        .window(TodoApp::window_settings())
        .centered()
        .theme(iced::Theme::Light)
        .style(|app, theme|iced::theme::Style{
            text_color: color::Gold::_50
        })
        .run()?;
    Ok(())
}
