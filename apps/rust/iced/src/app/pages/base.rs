use crate::app::components::icon_button;
use crate::app::components::icon_button::{IconButton, IconButtonStyle};
use crate::app::components::todo_composer::{TodoComposer, TodoComposerMessage};
use crate::app::theme::utility::icon;
use crate::app::{
    TodoAppMessage,
    theme::{color, fonts},
};
use aurora_leaf::Icon;
use iced::widget::checkbox::Status;
use iced::widget::{checkbox, mouse_area, rich_text, row, span};
use iced::{color, never, widget::{column, container, scrollable, space, text}, Alignment, Border, Color, Element, Fill, Padding, Task};
use todo_commonlib::{TodoItem, TodoList};
use tracing::{debug, error};

#[derive(Default)]
pub struct BasePage {
    input_form: TodoComposer,
    list: TodoList,
    hovered_item: Option<i64>,
}
#[derive(Debug, Clone)]
pub enum BasePageMessage {
    InputForm(TodoComposerMessage),
    UpdateList(TodoList),
    RefreshTodoList,
    Error(String),
    MarkCompleted(bool, i64),
    DeleteItem(i64),
    OnItemHover(i64),
    OnItemUnhover(i64),
}

impl BasePage {
    pub fn update(&mut self, message: BasePageMessage) -> Task<TodoAppMessage> {
        match message {
            BasePageMessage::InputForm(input_form) => match self.input_form.update(input_form) {
                Some(submission) => Task::perform(
                    TodoList::insert(submission.title, submission.description),
                    |result| match result {
                        Ok(_) => BasePageMessage::RefreshTodoList,
                        Err(e) => BasePageMessage::Error(e.to_string()),
                    },
                )
                .map(TodoAppMessage::BasePageMessage),
                None => Task::none(),
            },
            BasePageMessage::UpdateList(list) => {
                debug!("Todo list updated: {} items", list.list.len());
                self.list = list;
                Task::none()
            }
            BasePageMessage::RefreshTodoList => {
                Task::perform(TodoList::get(), |result| match result {
                    Ok(items) => BasePageMessage::UpdateList(items),
                    Err(e) => BasePageMessage::Error(e.to_string()),
                })
                .map(TodoAppMessage::BasePageMessage)
            }
            BasePageMessage::MarkCompleted(completed, id) => {
                if completed {
                    debug!("Marking item with id {} as completed", id);
                    Task::perform(TodoList::mark_done(id), |result| match result {
                        Ok(_) => BasePageMessage::RefreshTodoList,
                        Err(e) => BasePageMessage::Error(e.to_string()),
                    })
                    .map(TodoAppMessage::BasePageMessage)
                } else {
                    debug!("Marking item with id {} as undone", id);
                    Task::perform(TodoList::mark_undone(id), |result| match result {
                        Ok(_) => BasePageMessage::RefreshTodoList,
                        Err(e) => BasePageMessage::Error(e.to_string()),
                    })
                    .map(TodoAppMessage::BasePageMessage)
                }
            }
            BasePageMessage::DeleteItem(id) => {
                debug!("Deleting item with id {}", id);
                Task::perform(TodoList::delete(id), |result| match result {
                    Ok(_) => BasePageMessage::RefreshTodoList,
                    Err(e) => BasePageMessage::Error(e.to_string()),
                })
                .map(TodoAppMessage::BasePageMessage)
            }
            BasePageMessage::OnItemHover(id) => {
                debug!("Hovered item: {:?}", id);
                self.hovered_item = Some(id);
                Task::none()
            }
            BasePageMessage::OnItemUnhover(id) => {
                debug!("Unhovered item: {:?}", id);
                if let Some(current_hovered_id) = self.hovered_item
                    && current_hovered_id == id
                {
                    self.hovered_item = None;
                }
                Task::none()
            }
            BasePageMessage::Error(e) => {
                // TODO: Show the error to the user in some way!
                error!("{}", e);
                Task::none()
            }
        }
    }
    pub fn view(&self) -> Element<'_, BasePageMessage> {
        let items = self.list.list.iter().collect::<Vec<_>>();
        column![
            self.todo_header(&items),
            space().height(32),
            // Input form
            self.input_form.view().map(BasePageMessage::InputForm),
            space().height(22),
            self.todo_list(&items)
        ]
        .into()
    }

    fn todo_header(&self, items: &Vec<&TodoItem>) -> Element<'_, BasePageMessage> {
        let incomplete_items = items.iter().filter(|item| !item.done).collect::<Vec<_>>();

        let chip: Element<'_, BasePageMessage> = if items.is_empty() {
            row![].into()
        } else if !incomplete_items.is_empty() {
            container(
                text!("{} of {} left", incomplete_items.len(), items.len())
                    .color(color::Surface::PRIMARY_DARK)
                    .font(fonts::Roboto::MEDIUM)
                    .size(12),
            )
            .style(|_| {
                container::Style::default()
                    .border(Border::default().rounded(100))
                    .background(color::Surface::PRIMARY.scale_alpha(0.1))
            })
            .padding(Padding::new(2.0).left(8).right(8))
            .into()
        } else {

            container(
                text!("All done",)
                    .color(color!(0x2d6e44))
                    .font(fonts::Roboto::MEDIUM)
                    .size(12),
            )
                .style(|_| {
                    container::Style::default()
                        .border(Border::default().rounded(100))
                        .background(color!(0xafc6bf))
                })
                .padding(Padding::new(2.0).left(8).right(8))
                .into()
        };

        row![
            text!("Tasks")
                .size(36)
                .color(color::Text::DEFAULT)
                .font(fonts::Roboto::BOLD),
            space().width(12),
            chip
        ]
        .align_y(Alignment::Center)
        .width(Fill)
        .into()
    }

    fn todo_list(&self, items: &Vec<&TodoItem>) -> Element<'_, BasePageMessage> {
        let empty_state: Element<'_, BasePageMessage> = container(
            column![
                space().height(40),
                icon(
                    Icon::material_symbols().checklist_rounded(),
                    48,
                    Some((color::Surface::PRIMARY_DARK, color::Surface::PRIMARY_DARK))
                ),
                space().height(12),
                text!("All clear!").font(fonts::Roboto::MEDIUM).size(16),
                space().height(7),
                text!("Add a task above to get started.")
                    .font(fonts::Roboto::REGULAR)
                    .color(color::Text::MUTED)
                    .size(14),
                space().height(40),
            ]
            .width(Fill)
            .align_x(Alignment::Center),
        )
        .height(188)
        .width(Fill)
        .style(|_| {
            container::Style::default()
                .background(color::Surface::CARD)
                .border(Border::default().rounded(28.0))
        })
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .into();

        // Task List
        scrollable(if self.list.list.is_empty() {
            empty_state
        } else {
            container(column(items.iter().map(|item| self.task_item(item)))).into()
        })
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn task_item(&self, item: &TodoItem) -> Element<'_, BasePageMessage> {
        let item_id = item.id;
        let completed = item.done;
        let is_hovering = if let Some(hovered_item) = self.hovered_item {
            hovered_item == item_id
        } else {
            false
        };
        mouse_area(column![
            container(row![
                checkbox(item.done)
                    .size(20)
                    .style(|_, status| {
                        match status {
                            Status::Active { is_checked } => {
                                if is_checked {
                                    checkbox::Style {
                                        background: color::Surface::PRIMARY.into(),
                                        icon_color: color::Surface::CARD,
                                        border: Border::default().rounded(100),
                                        text_color: None,
                                    }
                                } else {
                                    checkbox::Style {
                                        background: color::Surface::APPLICATION.into(),
                                        icon_color: color::Surface::CARD,
                                        border: Border::default().rounded(100),
                                        text_color: None,
                                    }
                                }
                            }
                            Status::Hovered { is_checked } => {
                                if is_checked {
                                    checkbox::Style {
                                        background: color::Surface::PRIMARY.scale_alpha(0.5).into(),
                                        icon_color: color::Surface::CARD,
                                        border: Border::default().rounded(100),
                                        text_color: None,
                                    }
                                } else {
                                    checkbox::Style {
                                        background: color::Surface::APPLICATION
                                            .scale_alpha(0.5)
                                            .into(),
                                        icon_color: color::Surface::CARD,
                                        border: Border::default().rounded(100),
                                        text_color: None,
                                    }
                                }
                            }
                            Status::Disabled { is_checked } => {
                                if is_checked {
                                    checkbox::Style {
                                        background: color::Surface::PRIMARY_DARK
                                            .scale_alpha(0.25)
                                            .into(),
                                        icon_color: color::Surface::CARD,
                                        border: Border::default().rounded(100),
                                        text_color: None,
                                    }
                                } else {
                                    checkbox::Style {
                                        background: color::Surface::APPLICATION.into(),
                                        icon_color: color::Surface::CARD,
                                        border: Border::default().rounded(100),
                                        text_color: None,
                                    }
                                }
                            }
                        }
                    })
                    .on_toggle(move |toggled| BasePageMessage::MarkCompleted(toggled, item_id)),
                space().width(12),
                column![
                    rich_text![span(item.title.clone()).strikethrough(item.done).color(
                        if item.done {
                            color::Surface::PRIMARY_DARK
                        } else {
                            color::Text::DEFAULT
                        }
                    )]
                    .on_link_click(never)
                    .font(fonts::Roboto::MEDIUM)
                    .size(16)
                    .color(color::Text::DEFAULT.scale_alpha(if completed { 0.5 } else { 1.0 })),
                    text!("{}", item.description)
                        .font(fonts::Roboto::REGULAR)
                        .size(14)
                        .color(color::Text::MUTED.scale_alpha(if completed { 0.5 } else { 1.0 })),
                ]
                .width(Fill),
                IconButton::new(if is_hovering {
                    Icon::lucide().trash()
                } else {
                    ""
                })
                .size(32)
                .radius(100)
                .style(if is_hovering {
                    IconButtonStyle {
                        foreground_hover: color::Surface::PRIMARY,
                        background_hover: color::Surface::PRIMARY.scale_alpha(0.1),
                        ..icon_button::SECONDARY_BUTTON_STYLE
                    }
                } else {
                    IconButtonStyle {
                        background: Color::TRANSPARENT,
                        foreground: Color::TRANSPARENT,
                        ..icon_button::SECONDARY_BUTTON_STYLE
                    }
                })
                .on_press(BasePageMessage::DeleteItem(item_id))
            ])
            .style(move |_| {
                container::Style::default()
                    .background(color::Surface::CARD.scale_alpha(if completed { 0.5 } else { 1.0 }))
                    .border(Border::default().rounded(28))
            })
            .width(Fill)
            .padding(16),
            space().height(12),
        ])
        .on_enter(BasePageMessage::OnItemHover(item_id))
        .on_exit(BasePageMessage::OnItemUnhover(item_id))
        .into()
    }
}
