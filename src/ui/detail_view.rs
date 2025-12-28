use gpui::prelude::*;

use gpui::*;

use crate::kubernetes::ResourceItem;
use crate::theme::ThemeColors;
use crate::ui::glass::{GlassExt, GlassStyle};

pub struct DetailView;

impl DetailView {
    pub fn new(
        selected: Option<ResourceItem>,
        glass_style: GlassStyle,
        on_view_logs: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let mut base = div()
            .flex()
            .flex_col()
            .size_full()
            .glass_panel(glass_style, colors)
            .child(Self::render_header(colors));

        if let Some(resource) = selected {
            base = base.child(div().flex_1().p_4().child(Self::render_resource_detail(
                resource,
                on_view_logs,
                colors,
            )));
        }

        base
    }

    fn render_header(colors: &ThemeColors) -> impl IntoElement {
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
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(colors.text_primary)
                    .child("Details"),
            )
            .child(
                div()
                    .px_2()
                    .py_1()
                    .text_sm()
                    .text_color(colors.text_secondary)
                    .hover(|style| style.text_color(rgb(0xffffff)))
                    .cursor(CursorStyle::PointingHand)
                    .child("✕"),
            )
    }

    fn render_resource_detail(
        resource: ResourceItem,
        on_view_logs: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let title_str = "Basic Information".to_string();
        let label_name = "Name".to_string();
        let label_kind = "Kind".to_string();
        let label_namespace = "Namespace".to_string();
        let label_status = "Status".to_string();
        let label_age = "Age".to_string();

        let items = vec![
            (label_name, resource.name.clone()),
            (label_kind, resource.kind.display_name().to_string()),
            (
                label_namespace,
                resource
                    .namespace
                    .clone()
                    .unwrap_or_else(|| "-".to_string()),
            ),
            (label_status, resource.status.clone()),
            (label_age, resource.age.clone()),
        ];

        let mut content = div()
            .flex()
            .flex_col()
            .gap_4()
            .child(Self::render_info_section(title_str, items, colors));

        // Add Logs button for Pods
        if resource.kind == crate::kubernetes::ResourceKind::Pod {
            let pod_name = resource.name.clone();
            content = content.child(
                div().flex().justify_end().child(
                    div()
                        .id("view-logs-btn")
                        .px_4()
                        .py_2()
                        .bg(colors.bg_element)
                        .rounded_md()
                        .text_sm()
                        .text_color(colors.text_primary)
                        .cursor(CursorStyle::PointingHand)
                        .hover({
                            let cloned = colors.clone();
                            move |style| style.bg(cloned.bg_element_hover)
                        })
                        .on_click(move |_, win, cx| on_view_logs(pod_name.clone(), win, cx))
                        .child("View Logs"),
                ),
            );
        }

        content.child(Self::render_yaml_section(&resource, colors))
    }

    fn render_info_section(
        title: String,
        items: Vec<(String, String)>,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(colors.text_primary)
                    .pb_2()
                    .border_b_1()
                    .border_color(colors.border)
                    .child(title),
            )
            .children(items.into_iter().map({
                let colors = colors.clone();
                move |(label, value)| {
                    div()
                        .flex()
                        .gap_4()
                        .text_sm()
                        .child(
                            div()
                                .w(px(120.0))
                                .text_color(colors.text_muted)
                                .child(label),
                        )
                        .child(
                            div()
                                .flex_1()
                                .text_color(colors.text_secondary)
                                .child(value),
                        )
                }
            }))
    }

    fn render_yaml_section(resource: &ResourceItem, colors: &ThemeColors) -> impl IntoElement {
        let yaml = serde_yaml::to_string(&resource.metadata)
            .unwrap_or_else(|_| "Failed to serialize".to_string());

        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(colors.text_primary)
                    .pb_2()
                    .border_b_1()
                    .border_color(colors.border)
                    .child("YAML"),
            )
            .child(
                div()
                    .p_3()
                    .bg(if Hsla::from(colors.bg_app).l > 0.5 {
                        Hsla::from(colors.bg_element_active).opacity(0.1) // Light mode, slight tint
                    } else {
                        Hsla::from(colors.bg_sidebar) // Dark mode
                    })
                    .rounded_md()
                    .text_xs()
                    .font_family("'JetBrains Mono', 'Fira Code', 'SF Mono', Menlo, Monaco, 'Courier New', monospace")
                    .text_color(colors.text_secondary)
                    .max_h(px(500.0))
                    .line_height(relative(1.6))
                    .child(yaml),
            )
    }
}
