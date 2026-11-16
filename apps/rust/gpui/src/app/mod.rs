use gpui::{div, Context, IntoElement, ParentElement, Render, SharedString, Styled, Window, rgb};

pub struct App {
    pub text: SharedString,
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0xe3e3ea))
            .justify_center()
            .items_center()
            .shadow_lg()
            .text_2xl()
            .size_full()
            .child(format!("Hello, {}!", &self.text))
    }
}
