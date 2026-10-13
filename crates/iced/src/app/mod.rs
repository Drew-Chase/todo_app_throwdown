pub mod components;
pub mod theme;
mod titlebar;

use crate::app::theme::color::Surface;
use crate::app::titlebar::{Titlebar, TitlebarMessage};
use iced::widget::{column, container, text};
use iced::window::{Id, Level, Position};
use iced::{Element, Fill, Padding, Size, Task, window};

#[derive(Default)]
pub struct TodoApp {
    id: Option<Id>,
    titlebar: Titlebar,
}

#[derive(Debug, Clone)]
pub enum TodoAppMessage {
    WindowCreation(Option<Id>),
    TitlebarMessage(TitlebarMessage),
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
                if let Some(id) = self.id {
                    self.titlebar.update(message, id)
                } else {
                    Task::none()
                }
            }
            TodoAppMessage::WindowCreation(id) => {
                self.id = id;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, TodoAppMessage> {
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
            .center_x(Fill)
        ]
        .into()
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
                drag_and_drop: true,
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
