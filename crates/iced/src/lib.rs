mod app;
mod math;

use crate::app::TodoApp;
use app::theme::color;
use aurora_leaf::FontFamily;
use color_eyre::Result;
use tracing::info;

pub fn run() -> Result<()> {
    info!("Starting todo app v{}#{}", TodoApp::VERSION, TodoApp::BUILD);
    iced::application(TodoApp::boot, TodoApp::update, TodoApp::view)
        .title(TodoApp::TITLE)
        .window(TodoApp::window_settings())
        .centered()
        .theme(iced::Theme::Light)
        .font(FontFamily::roboto().bold())
        .font(FontFamily::roboto().regular())
        .subscription(TodoApp::subscription)
        .style(|_app, _theme| iced::theme::Style {
            text_color: color::Text::DEFAULT,
            background_color: color::Surface::APPLICATION,
        })
        .run()?;
    Ok(())
}
