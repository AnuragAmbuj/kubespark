use crate::theme::ThemeColors;
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
        colors: &ThemeColors,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .glass_panel(self.glass_style, colors)
            .child(self.render_header(on_close, colors))
            .child(self.render_log_content(colors))
    }

    fn render_header(
        &self,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(48.0))
            .px_4()
            .border_b_1()
            .border_color(colors.border)
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors.text_primary)
                            .child("Logs"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(colors.text_muted)
                            .child(self.pod_name.clone()),
                    ),
            )
            .child(
                div()
                    .id("close-logs")
                    .px_2()
                    .py_1()
                    .text_sm()
                    .text_color(colors.text_secondary)
                    .hover({
                        let cloned = colors.clone();
                        move |style| style.text_color(cloned.text_primary)
                    })
                    .cursor(CursorStyle::PointingHand)
                    .on_click(on_close)
                    .child("Close"),
            )
    }

    fn render_log_content(&self, colors: &ThemeColors) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .p_4()
            // Log background: distinct from panel
            .bg(if Hsla::from(colors.bg_app).l > 0.5 {
                Hsla::from(colors.bg_sidebar) // Darker than panel in light mode usually? Or just use sidebar
            } else {
                Hsla::from(colors.bg_app).opacity(0.5) // Dark mode: darker
            })
            .child(
                div()
                    .font_family("'JetBrains Mono', 'Fira Code', monospace")
                    .text_xs()
                    .text_color(colors.text_secondary)
                    .line_height(relative(1.5))
                    .children(self.logs.iter().map(|line| div().child(line.clone()))),
            )
    }
}
