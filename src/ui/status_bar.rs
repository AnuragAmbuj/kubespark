use crate::app::ConnectionStatus;
use crate::ui::glass::{GlassExt, GlassStyle};
use gpui::*;

#[allow(dead_code)]
pub struct StatusBar;

impl StatusBar {
    pub fn new(connection_status: ConnectionStatus, glass_style: GlassStyle) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .size_full()
            .px_4()
            .glass_panel(glass_style)
            .border_t_1()
            .border_color(rgb(0x3e3e3e))
            .text_color(rgb(0xffffff))
            .child(Self::render_connection_status(&connection_status))
            .child(Self::render_info())
    }

    fn render_connection_status(status: &ConnectionStatus) -> impl IntoElement {
        let (text, color) = match status {
            ConnectionStatus::Disconnected => ("Disconnected", rgb(0xf48771)),
            ConnectionStatus::Connecting => ("Connecting...", rgb(0xdcdcaa)),
            ConnectionStatus::Connected => ("Connected", rgb(0x4ec9b0)),
            ConnectionStatus::Error(msg) => {
                return div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_xs()
                    .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(rgb(0xf48771)))
                    .child(format!("Error: {}", msg))
            }
        };

        div()
            .flex()
            .items_center()
            .gap_2()
            .text_xs()
            .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(color))
            .child(text)
    }

    fn render_info() -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_4()
            .text_xs()
            .child("KubeSpark v0.1.0")
    }
}
