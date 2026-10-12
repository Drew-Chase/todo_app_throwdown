pub struct Titlebar {}
pub enum TitlebarMessage {
    Close,
    ToggleMaximize,
    ToggleMinimize,
    StartDragWindow,
    EndDragWindow,
}

impl Titlebar {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update() {}
    pub fn view() {}
}
