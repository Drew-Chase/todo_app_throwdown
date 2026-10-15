pub mod components;
pub mod theme;
mod titlebar;

use crate::app::theme::color::Surface;
use crate::app::titlebar::{Titlebar, TitlebarMessage};
use iced::widget::{column, container, stack, text};
use iced::window::{Direction, Id, Level, Position};
use iced::{Element, Fill, Padding, Size, Subscription, Task, window};
use iced_borderless_window_resize_handle_widget::{ResizeHandles, drag_resize};

#[derive(Default)]
pub struct TodoApp {
    id: Option<Id>,
    titlebar: Titlebar,
    maximized: bool,
}

#[derive(Debug, Clone)]
pub enum TodoAppMessage {
    WindowCreation(Option<Id>),
    WindowStartResize(Direction),
    WindowEndResize(Id, Size),
    TitlebarMessage(TitlebarMessage),
    WindowMaximized(bool),
}

impl TodoApp {
    pub const TITLE: &'static str = "Iced Todo App";
    /// The release major and minor versions.
    /// Ex: `0.0.0`
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    /// A unique build number that is created automatically at compile time.
    /// Ex: `20260827.032580`
    pub const BUILD: &'static str = env!("BUILD");

    pub fn new() -> (Self, Task<TodoAppMessage>) {
        let id = window::latest();
        let app = Self { ..Self::default() };
        (app, Task::batch([id.map(TodoAppMessage::WindowCreation)]))
    }

    pub fn update(&mut self, message: TodoAppMessage) -> Task<TodoAppMessage> {
        match message {
            TodoAppMessage::TitlebarMessage(message) => {
                match self.id {
                    Some(id) => self.titlebar.update(message, id),
                    None => Task::none(),
                }
            }
            TodoAppMessage::WindowCreation(id) => {
                self.id = id;
                Task::none()
            }
            TodoAppMessage::WindowStartResize(direction) => match self.id {
                Some(id) => drag_resize(id, direction),
                None => Task::none(),
            },
            TodoAppMessage::WindowEndResize(id, _size) => {
                window::is_maximized(id).map(TodoAppMessage::WindowMaximized)
            }
            TodoAppMessage::WindowMaximized(maximized) => {
                self.maximized = maximized;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, TodoAppMessage> {
        stack![
            column![
                self.titlebar.view().map(TodoAppMessage::TitlebarMessage),
                container(
                    container(column![
                        text!("Tasks")
                            .size(36)
                            .color(theme::color::Text::DEFAULT)
                            .font(theme::fonts::Roboto::BOLD)
                            .width(Fill)
                    ])
                    .width(Fill)
                    .max_width(576)
                    .height(Fill)
                    .padding(Padding {
                        left: 16.0,
                        right: 16.0,
                        top: 32.0,
                        bottom: 32.0
                    })
                )
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Surface::APPLICATION)),
                    ..Default::default()
                })
                .center_x(Fill)
            ],
            ResizeHandles::new(TodoAppMessage::WindowStartResize)
                .gutter(4.0)
                .enabled(!self.maximized)
        ]
        .width(Fill)
        .height(Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<TodoAppMessage> {
        Subscription::batch([
            window::resize_events().map(|(id, size)| TodoAppMessage::WindowEndResize(id, size))
        ])
    }

    pub fn window_settings() -> window::Settings {
        window::Settings {
            size: Size::new(600f32, 600f32),
            min_size: Some(Size::new(680f32, 440f32)),
            decorations: !cfg!(target_os = "windows"),
            resizable: true,
            #[cfg(target_os = "windows")]
            platform_specific: window::settings::PlatformSpecific {
                corner_preference: window::settings::platform::CornerPreference::Default,
                undecorated_shadow: true,
                drag_and_drop: false,
                skip_taskbar: false,
            },
            #[cfg(target_os = "macos")]
            platform_specific: iced::window::settings::PlatformSpecific {
                title_hidden: true,
                titlebar_transparent: true,
                fullsize_content_view: true,
            },
            #[cfg(target_os = "linux")]
            platform_specific: iced::window::settings::PlatformSpecific {
                #[cfg(debug_assertions)]
                application_id: String::from("todo_app-dev"),
                #[cfg(not(debug_assertions))]
                application_id: String::from("todo_app"),
                override_redirect: false,
            },
            maximized: false,
            fullscreen: false,
            position: Position::Default,
            max_size: None,
            visible: true,
            minimizable: true,
            closeable: true,
            transparent: false,
            blur: false,
            level: Level::Normal,
            icon: None,
            exit_on_close_request: true,
        }
    }
}
