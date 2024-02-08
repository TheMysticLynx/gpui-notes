use gpui::{Global, Point, TitlebarOptions, WindowOptions};

use crate::theme::Theme;

pub fn get_window_options() -> WindowOptions {
    WindowOptions {
        titlebar: Some(get_title_bar_options()),
        ..Default::default()
    }
}

fn get_title_bar_options() -> TitlebarOptions {
    TitlebarOptions {
        title: Some("gpui-notes".into()),
        appears_transparent: true,
        traffic_light_position: Some(Point::new(gpui::Pixels(10.0), gpui::Pixels(10.0))),
        ..Default::default()
    }
}

pub struct WindowState {
    pub theme: Theme,
}

impl Global for WindowState {}
