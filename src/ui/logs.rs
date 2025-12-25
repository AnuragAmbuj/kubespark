use crate::ui::glass::{GlassExt, GlassStyle};
use gpui::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LogView {
    pub pod_name: String,
    pub namespace: String,
    pub logs: Vec<String>,
    pub glass_style: GlassStyle,
}

impl LogView {
    pub fn new(pod_name: String, namespace: String, glass_style: GlassStyle) -> Self {
        Self {
            pod_name,
            namespace,
            logs: vec!["Loading logs...".to_string()],
            glass_style,
        }
    }

    pub fn set_logs(&mut self, logs: Vec<String>) {
        self.logs = logs;
    }

    pub fn render(
        &self,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .glass_panel(self.glass_style)
            .child(self.render_header(on_close))
            .child(self.render_log_content())
    }

    fn render_header(
        &self,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(48.0))
            .px_4()
            .border_b_1()
            .border_color(rgb(0x3e3e3e))
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .child("Logs"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x888888))
                            .child(self.pod_name.clone()),
                    ),
            )
            .child(
                div()
                    .id("close-logs")
                    .px_2()
                    .py_1()
                    .text_sm()
                    .text_color(rgb(0xcccccc))
                    .hover(|style| style.text_color(rgb(0xffffff)))
                    .cursor(CursorStyle::PointingHand)
                    .on_click(on_close)
                    .child("Close"),
            )
    }

    fn render_log_content(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .p_4()
            // .overflow_y_scroll() // Temporarily removed to fix build
            .bg(hsla(0.0, 0.0, 0.12, 0.8))
            .child(
                div()
                    .font_family("'JetBrains Mono', 'Fira Code', monospace")
                    .text_xs()
                    .text_color(rgb(0xcccccc))
                    .line_height(relative(1.5))
                    .children(self.logs.iter().map(|line| div().child(line.clone()))),
            )
    }
}
