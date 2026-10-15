use crate::app::components::todo_composer::{TodoComposer, TodoComposerMessage};
use crate::app::{
    TodoAppMessage,
    theme::{color, fonts},
};
use iced::widget::{container, row, scrollable, space};
use iced::{
    Border, Element, Fill, Task,
    widget::{column, text},
};
use todo_commonlib::{TodoAppError, TodoList};
use tracing::{debug, error};

#[derive(Default)]
pub struct BasePage {
    input_form: TodoComposer,
    list: TodoList,
}
#[derive(Debug)]
pub enum BasePageMessage {
    InputForm(TodoComposerMessage),
    UpdateList(TodoList),
    RefreshTodoList,
    Error(TodoAppError),
}

impl BasePage {
    pub fn update(&mut self, message: BasePageMessage) -> Task<TodoAppMessage> {
        match message {
            BasePageMessage::InputForm(input_form) => match self.input_form.update(input_form) {
                Some(submission) => Task::perform(
                    TodoList::insert(submission.title, submission.description),
                    |result| match result {
                        Ok(_) => BasePageMessage::RefreshTodoList,
                        Err(e) => BasePageMessage::Error(e),
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
                    Err(e) => BasePageMessage::Error(e),
                })
                .map(TodoAppMessage::BasePageMessage)
            }
            BasePageMessage::Error(e) => {
                // TODO: Show the error to the user in some way!
                error!("{}", e);
                Task::none()
            }
        }
    }
    pub fn view(&self) -> Element<'_, BasePageMessage> {
        column![
            text!("Tasks")
                .size(36)
                .color(color::Text::DEFAULT)
                .font(fonts::Roboto::BOLD)
                .width(Fill),
            space().height(32),
            // Input form
            self.input_form.view().map(BasePageMessage::InputForm),
            space().height(22),
            // Task List
            container(scrollable(row![
                
            ]).width(Fill).height(Fill))
                .height(184)
                .width(Fill)
                .style(|_| container::Style::default()
                    .background(color::Surface::CARD)
                    .border(Border::default().rounded(28.0)))
        ]
        .into()
    }
}
