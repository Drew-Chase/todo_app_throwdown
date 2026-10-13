use crate::app::components::icon_button;
use crate::app::components::icon_button::IconButton;
use crate::app::theme::color;
use crate::app::{TodoApp, TodoAppMessage};
use aurora_leaf::Icon;
use iced::widget::{Text, container, mouse_area, row};
use iced::{Background, Element, Fill, Task, window};

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
        container(row![
            mouse_area(row![
                container(
                    Text::new(TodoApp::TITLE)
                        .size(18)
                        .font(crate::app::theme::fonts::Roboto::BOLD)
                )
                .width(Fill)
                .height(Fill),
            ])
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
                .on_press(TitlebarMessage::ToggleMaximize),
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
/*
button(
    container(icon(
        Icon::material_symbols().close(),
        16,
        Some((
            color::buttons::Danger::FOREGROUND_HOVER,
            color::buttons::Danger::FOREGROUND
        ))
    ))
    .height(Fill)
    .width(Fill)
)
.height(Fill)
.width(47)
.style(|_theme, status| {
    match status {
        Status::Active => button::Style {
            background: Some(Background::from(color::buttons::Danger::BACKGROUND)),
            text_color: color::buttons::Danger::FOREGROUND,
            ..Default::default()
        },
        Status::Disabled | Status::Pressed | Status::Hovered => button::Style {
            background: Some(Background::from(
                color::buttons::Danger::BACKGROUND_HOVER,
            )),
            text_color: color::buttons::Danger::FOREGROUND_HOVER,
            ..Default::default()
        },
    }
})
.on_press(TitlebarMessage::Close)
 */
