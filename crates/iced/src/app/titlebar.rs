use crate::app::TodoAppMessage;
use crate::app::theme::color;
use crate::app::theme::utility::icon;
use aurora_leaf::Icon;
use iced::widget::{container, mouse_area, row, text};
use iced::{Background, Color, Element, Fill, Task, window};

#[derive(Debug, Clone, Default)]
pub struct Titlebar {}
#[derive(Debug, Clone)]
pub enum TitlebarMessage {
    Close,
    ToggleMaximize,
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
            TitlebarMessage::ToggleMaximize => window::toggle_maximize(window_id),
            TitlebarMessage::ToggleMinimize => window::minimize(window_id, true),
            TitlebarMessage::Close => window::close(window_id),
            TitlebarMessage::StartDragWindow => window::drag(window_id),
        }
    }
    pub fn view(&self) -> Element<'_, TitlebarMessage> {
        row![
            mouse_area(row![
                container(text!("Todo App"))
                    .width(Fill)
                    .height(32)
                    .style(|_theme| container::Style {
                        background: Some(Background::from(Color::from_rgb8(0xff, 0x00, 0x00))),
                        ..Default::default()
                    }),
            ])
            .on_press(TitlebarMessage::StartDragWindow),
            mouse_area(container(icon(
                Icon::material_symbols().close(),
                25,
                Some((color::Text::DEFAULT, color::Text::PLACEHOLDER))
            )))
            .on_press(TitlebarMessage::Close)
        ]
        .into()
    }
}
