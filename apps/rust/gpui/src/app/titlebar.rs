use gpui::{Context, IntoElement, Render, Window, div};

#[derive(Default)]
pub struct Titlebar;

impl Render for Titlebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
