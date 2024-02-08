use gpui::{
    div, rems, rgb, AnyView, DefiniteLength, Div, IntoElement, Model, ParentElement, Rems, Render,
    RenderOnce, Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::window::WindowState;

pub fn build_workspace_view<'a, 'b>(ctx: &'a mut WindowContext<'b>) -> View<Workspace> {
    ctx.new_view(|view_ctx| Workspace {})
}

pub struct Workspace {}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let state = cx.window_context().global::<WindowState>();
        div()
            .bg(state.theme.e0())
            .size((DefiniteLength::Fraction(1.0)))
            .child(TitleBar::new("Workspace".into()))
    }
}

#[derive(IntoElement)]
pub struct TitleBar {
    title: String,
}

impl TitleBar {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let state = cx.global::<WindowState>();
        div()
            .border()
            .border_color(state.theme.borders())
            .bg(state.theme.e1())
            .w(DefiniteLength::Fraction(1.0))
            .h(DefiniteLength::Absolute(gpui::AbsoluteLength::Pixels(
                gpui::Pixels(35.0),
            )))
    }
}
