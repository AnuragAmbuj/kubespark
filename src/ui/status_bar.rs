use crate::app::ConnectionStatus;
use crate::theme::ThemeColors;
use crate::ui::glass::{GlassExt, GlassStyle};
use gpui::*;

#[allow(dead_code)]
pub struct StatusBar;

impl StatusBar {
    pub fn new(
        connection_status: &ConnectionStatus,
        glass_style: GlassStyle,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .size_full()
            .px_4()
            .glass_panel(glass_style, colors)
            .border_t_1()
            .border_color(colors.border)
            .text_color(colors.text_primary)
            .child(Self::render_connection_status(connection_status, colors))
            .child(Self::render_info(colors))
    }

    fn render_connection_status(
        status: &ConnectionStatus,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let (text, color) = match status {
            ConnectionStatus::Disconnected => ("Disconnected", colors.status_error),
            ConnectionStatus::Connecting => ("Connecting...", colors.status_warning),
            ConnectionStatus::Connected => ("Connected", colors.status_ok),
            ConnectionStatus::Error(msg) => {
                return div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_xs()
                    .child(
                        div()
                            .w(px(8.0))
                            .h(px(8.0))
                            .rounded_full()
                            .bg(colors.status_error),
                    )
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

    fn render_info(colors: &ThemeColors) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_4()
            .text_xs()
            .text_color(colors.text_muted)
            .child("KubeSpark v0.1.0")
    }
}
