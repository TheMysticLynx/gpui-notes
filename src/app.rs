use crate::{
    theme::Theme,
    window::{get_window_options, WindowState},
    workspace::build_workspace_view,
};

pub fn run_app(app: gpui::App) {
    app.run(move |cx| {
        cx.set_global(WindowState {
            theme: Theme::default(),
        });
        cx.open_window(get_window_options(), build_workspace_view);
    });
}
