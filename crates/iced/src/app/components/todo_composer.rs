use crate::app::components::icon_button::{IconButton, PRIMARY_BUTTON_STYLE};
use crate::app::theme::color::{Surface, Text};
use crate::app::theme::fonts;
use aurora_leaf::Icon;
use iced::widget::{column, container, row, space, text, text_editor, text_input};
use iced::{Alignment, Background, Border, Color, Element, Fill, Padding};

pub struct TodoSubmission {
    pub title: String,
    pub description: String,
}
#[derive(Default)]
pub struct TodoComposer {
    title: String,
    description: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum TodoComposerMessage {
    TitleChanged(String),
    DescriptionChanged(text_editor::Action),
    SubmitRequested,
}

impl TodoComposer {
    pub fn update(&mut self, message: TodoComposerMessage) -> Option<TodoSubmission> {
        match message {
            TodoComposerMessage::TitleChanged(value) => {
                self.title = value;
                None
            }
            TodoComposerMessage::DescriptionChanged(value) => {
                self.description.perform(value);
                None
            }
            TodoComposerMessage::SubmitRequested => {
                let submission = TodoSubmission {
                    title: std::mem::take(&mut self.title),
                    description: std::mem::take(&mut self.description.text()),
                };
                self.title = String::new();
                self.description = text_editor::Content::default();
                Some(submission)
            }
        }
    }

    pub fn view(&self) -> Element<'_, TodoComposerMessage> {
        container(column![
            text_input("What needs doing?", &self.title)
                .padding(Padding::new(8.0).left(4).right(4))
                .size(18)
                .font(fonts::Roboto::REGULAR)
                .style(|_theme, status| {
                    let default = text_input::Style {
                        background: Background::Color(Color::TRANSPARENT),
                        border: Border::default()
                            .color(Color::TRANSPARENT)
                            .width(2)
                            .rounded(12),
                        icon: Default::default(),
                        placeholder: Text::PLACEHOLDER,
                        value: Text::DEFAULT,
                        selection: Surface::PRIMARY_DARK,
                    };
                    match status {
                        text_input::Status::Focused { .. } => text_input::Style {
                            border: default.border.color(Surface::PRIMARY),
                            ..default
                        },
                        _ => default,
                    }
                })
                .line_height(1.5)
                .on_input(TodoComposerMessage::TitleChanged)
                .on_submit(TodoComposerMessage::SubmitRequested)
                .width(Fill),
            space().height(4),
            text_editor(&self.description)
                .placeholder("Add details (optional)")
                .padding(Padding::new(8.0).left(4).right(4))
                .line_height(1.5)
                .font(fonts::Roboto::REGULAR)
                .size(14)
                .height(60)
                .style(|_theme, status| {
                    let default = text_editor::Style {
                        background: Background::Color(Color::TRANSPARENT),
                        border: Border::default()
                            .color(Color::TRANSPARENT)
                            .width(2)
                            .rounded(12),
                        placeholder: Text::PLACEHOLDER,
                        value: Text::DEFAULT,
                        selection: Surface::PRIMARY_DARK,
                    };
                    match status {
                        text_editor::Status::Focused { .. } => text_editor::Style {
                            border: default.border.color(Surface::PRIMARY),
                            ..default
                        },
                        _ => default,
                    }
                })
                .on_action(TodoComposerMessage::DescriptionChanged),
            space().height(Fill),
            row![
                text!("Press Enter to add")
                    .size(12)
                    .color(Text::PLACEHOLDER)
                    .font(fonts::Roboto::REGULAR)
                    .width(Fill),
                IconButton::new(Icon::material_symbols().add_rounded())
                    .style(PRIMARY_BUTTON_STYLE)
                    .on_press(TodoComposerMessage::SubmitRequested)
                    .disabled(self.title.is_empty())
                    .width(40)
                    .height(40)
                    .radius(100)
            ]
            .align_y(Alignment::Center)
        ])
        .padding(16)
        .height(184)
        .width(Fill)
        .style(|_| {
            container::Style::default()
                .background(Surface::CARD)
                .border(Border::default().rounded(28.0))
        })
        .into()
    }
}
