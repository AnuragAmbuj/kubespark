use gpui::*;

use crate::ui::glass::{GlassExt, GlassStyle};

pub struct DashboardView {
    glass_style: GlassStyle,
}

impl DashboardView {
    pub fn new(glass_style: GlassStyle) -> Self {
        Self { glass_style }
    }

    pub fn render(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .glass_panel(self.glass_style)
            .child(self.render_header())
            .child(self.render_stats())
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_4()
                    .p_6()
                    .pt_0()
                    .child(self.render_resource_chart())
                    .child(self.render_recent_events()),
            )
    }

    fn render_header(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .h(px(48.0))
            .px_4()
            .border_b_1()
            .border_color(rgb(0x3e3e3e))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xffffff))
                    .child("Cluster Overview"),
            )
    }

    fn render_stats(&self) -> impl IntoElement {
        div()
            .flex()
            .gap_4()
            .p_6()
            .child(self.render_stat_card("Nodes", "3", "Ready"))
            .child(self.render_stat_card("Pods", "12", "Running"))
            .child(self.render_stat_card("CPU", "45%", "Usage"))
            .child(self.render_stat_card("Memory", "2.1 GB", "Usage"))
    }

    fn render_stat_card(&self, title: &str, value: &str, subtitle: &str) -> impl IntoElement {
        div()
            .w(px(200.0))
            .h(px(100.0))
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .glass_card(self.glass_style)
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xaaaaaa))
                    .mb_1()
                    .child(title.to_string()),
            )
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child(value.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x4ade80)) // Green success color
                    .mt_1()
                    .child(subtitle.to_string()),
            )
    }

    fn render_resource_chart(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .glass_card(self.glass_style)
            .p_4()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0xcccccc))
                    .mb_4()
                    .child("Resource Usage History"),
            )
            .child(
                div()
                    .flex()
                    .items_end()
                    .justify_between()
                    .gap_2()
                    .h_full()
                    .pb_2()
                    .children((0..20).map(|i| {
                        let height = 20.0 + (i as f32 * 7.0 % 60.0) + (i as f32 * 3.0 % 20.0);
                        div().w_full().h(px(height)).rounded_t_sm().bg(hsla(
                            210.0 / 360.0,
                            0.6,
                            0.6,
                            0.5,
                        )) // Blue-ish bars
                    })),
            )
    }

    fn render_recent_events(&self) -> impl IntoElement {
        let events = vec![
            (
                "Normal",
                "Scheduled",
                "pod/nginx-deployment-5d59d67564-abcde",
                "Successfully assigned default/nginx to node-1",
            ),
            (
                "Normal",
                "Pulling",
                "pod/nginx-deployment-5d59d67564-abcde",
                "Pulling image 'nginx:latest'",
            ),
            (
                "Normal",
                "Created",
                "pod/nginx-deployment-5d59d67564-abcde",
                "Created container nginx",
            ),
            (
                "Normal",
                "Started",
                "pod/nginx-deployment-5d59d67564-abcde",
                "Started container nginx",
            ),
            (
                "Warning",
                "BackOff",
                "pod/crash-loop-pod",
                "Back-off restarting failed container",
            ),
        ];

        div()
            .flex()
            .flex_col()
            .flex_1()
            .glass_card(self.glass_style)
            .p_4()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0xcccccc))
                    .mb_4()
                    .child("Recent Events"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(events.into_iter().map(|(kind, reason, object, message)| {
                        div()
                            .flex()
                            .flex_col()
                            .gap_px()
                            .p_2()
                            .rounded_md()
                            .bg(hsla(0.0, 0.0, 1.0, 0.03))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(if kind == "Warning" {
                                                rgb(0xffaa00)
                                            } else {
                                                rgb(0xcccccc)
                                            })
                                            .child(reason),
                                    )
                                    .child(div().text_xs().text_color(rgb(0x666666)).child(object)),
                            )
                            .child(div().text_xs().text_color(rgb(0xaaaaaa)).child(message))
                    })),
            )
    }
}
