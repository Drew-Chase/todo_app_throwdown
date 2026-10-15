use crate::app::components::icon_button;
use crate::app::components::icon_button::IconButton;
use crate::app::theme::color;
use crate::app::{TodoApp, TodoAppMessage};
use aurora_leaf::Icon;
use iced::widget::{Text, container, mouse_area, row};
use iced::{Background, Element, Fill, Padding, Task, window};
use tracing::{debug, info};

#[derive(Debug, Clone, Default)]
pub struct Titlebar;
#[derive(Debug, Clone)]
pub enum TitlebarMessage {
    Close,
    ToggleMaximized,
    ToggleMinimize,
    StartDragWindow,
}

impl Titlebar {
    pub fn update(
        &mut self,
        message: TitlebarMessage,
        window_id: window::Id,
    ) -> Task<TodoAppMessage> {
        match message {
            TitlebarMessage::ToggleMaximized => {
                info!("Toggling maximize on window {window_id}");
                window::toggle_maximize(window_id)
            }
            TitlebarMessage::ToggleMinimize => {
                info!("Minimizing window {window_id}");
                window::minimize(window_id, true)
            }
            TitlebarMessage::Close => {
                info!("Closing window {window_id}");
                window::close(window_id)
            }
            TitlebarMessage::StartDragWindow => {
                debug!("Starting drag on window {window_id}");
                window::drag(window_id)
            }
        }
    }
    pub fn view(&self) -> Element<'_, TitlebarMessage> {
        container(row![
            mouse_area(row![
                container(
                    Text::new(TodoApp::TITLE)
                        .size(18)
                        .font(crate::app::theme::fonts::Roboto::BOLD)
                )
                .padding(Padding::new(0.0).left(8).right(8).top(4))
                .width(Fill)
                .height(Fill),
            ])
            .on_double_click(TitlebarMessage::ToggleMaximized)
            .on_press(TitlebarMessage::StartDragWindow),
            IconButton::new(Icon::material_symbols().minimize_rounded())
                .width(47)
                .height(Fill)
                .style(icon_button::SECONDARY_BUTTON_STYLE)
                .on_press(TitlebarMessage::ToggleMinimize),
            IconButton::new(Icon::material_symbols().square_outline_rounded())
                .width(47)
                .icon_size(11.5)
                .height(Fill)
                .style(icon_button::SECONDARY_BUTTON_STYLE)
                .on_press(TitlebarMessage::ToggleMaximized),
            IconButton::new(Icon::material_symbols().close_rounded())
                .width(47)
                .height(Fill)
                .style(icon_button::DANGER_BUTTON_STYLE)
                .on_press(TitlebarMessage::Close)
        ])
        .width(Fill)
        .height(32)
        .style(|_| container::Style {
            background: Some(Background::from(color::Surface::TITLEBAR)),
            ..Default::default()
        })
        .into()
    }
}
