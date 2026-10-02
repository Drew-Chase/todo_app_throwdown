use iced::window::{Level, Position};
use iced::{Element, Size, Task};
use iced::widget::row;

#[derive(Default)]
pub struct TodoApp {}

#[derive(Debug, Clone)]
pub enum TodoAppMessage {}

impl TodoApp {
    pub const TITLE: &'static str = "Sift";
    /// The release major and minor versions.
    /// Ex: `0.0.0`
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    /// A unique build number that is created automatically at compile time.
    /// Ex: `20260827.032580`
    pub const BUILD: &'static str = env!("BUILD");

    pub fn new() -> (Self, Task<TodoAppMessage>) {
        let app = Self { ..Self::default() };
        (app, Task::batch([]))
    }

    pub fn update(&mut self, message: TodoAppMessage) -> Task<TodoAppMessage> {
        match message {}
    }

    pub fn view(&self) -> Element<'_, TodoAppMessage>{
        row![].into()
    }

    pub fn window_settings() -> iced::window::Settings {
        iced::window::Settings {
            size: Size::new(1280f32, 800f32),
            min_size: Some(Size::new(933f32, 776f32)),
            decorations: !cfg!(target_os = "windows"),
            resizable: true,
            #[cfg(target_os = "windows")]
            platform_specific: iced::window::settings::PlatformSpecific {
                corner_preference: iced::window::settings::platform::CornerPreference::Default,
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
