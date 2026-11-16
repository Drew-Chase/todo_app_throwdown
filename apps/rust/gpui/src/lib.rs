mod app;

use color_eyre::{Result, eyre::eyre};
use gpui::{
    App, Application, Bounds, WindowBackgroundAppearance, WindowBounds, WindowOptions, prelude::*,
    px, size,
};
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() -> Result<()> {
    let application_run_outcome: Rc<RefCell<Option<Result<()>>>> = Rc::new(RefCell::new(None));
    let shared_outcome = Rc::clone(&application_run_outcome);

    Application::new().run(move |ctx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(600.)), ctx);
        let open_window_result = ctx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(600.), px(440.))),
                window_background: WindowBackgroundAppearance::Opaque,
                ..Default::default()
            },
            |_, ctx| {
                ctx.new(|_| app::App {
                    text: "Test".into(),
                })
            },
        );

        if open_window_result.is_err() {
            *shared_outcome.borrow_mut() = Some(Err(eyre!("Failed to create a window")));
        }
    });

    if let Some(msg) = application_run_outcome.take() {
        msg
    } else {
        Ok(())
    }
}
