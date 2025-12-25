use gpui::prelude::*;

use gpui::*;

use crate::kubernetes::ResourceItem;
use crate::ui::glass::{GlassExt, GlassStyle};

pub struct DetailView;

impl DetailView {
    pub fn new(
        selected: Option<ResourceItem>,
        glass_style: GlassStyle,
        on_view_logs: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        let mut base = div()
            .flex()
            .flex_col()
            .size_full()
            .glass_panel(glass_style)
            .child(Self::render_header());

        if let Some(resource) = selected {
            base = base.child(
                div()
                    .flex_1()
                    .p_4()
                    .child(Self::render_resource_detail(resource, on_view_logs)),
            );
        }

        base
    }

    fn render_header() -> impl IntoElement {
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
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xffffff))
                    .child("Details"),
            )
            .child(
                div()
                    .px_2()
                    .py_1()
                    .text_sm()
                    .text_color(rgb(0xcccccc))
                    .hover(|style| style.text_color(rgb(0xffffff)))
                    .cursor(CursorStyle::PointingHand)
                    .child("✕"),
            )
    }

    fn render_resource_detail(
        resource: ResourceItem,
        on_view_logs: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
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
            .child(Self::render_info_section(title_str, items));

        // Add Logs button for Pods
        if resource.kind == crate::kubernetes::ResourceKind::Pod {
            let pod_name = resource.name.clone();
            content = content.child(
                div().flex().justify_end().child(
                    div()
                        .id("view-logs-btn")
                        .px_4()
                        .py_2()
                        .bg(rgb(0x3e3e3e))
                        .rounded_md()
                        .text_sm()
                        .text_color(rgb(0xffffff))
                        .cursor(CursorStyle::PointingHand)
                        .hover(|style| style.bg(rgb(0x505050)))
                        .on_click(move |_, win, cx| on_view_logs(pod_name.clone(), win, cx))
                        .child("View Logs"),
                ),
            );
        }

        content.child(Self::render_yaml_section(&resource))
    }

    fn render_info_section(title: String, items: Vec<(String, String)>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xffffff))
                    .pb_2()
                    .border_b_1()
                    .border_color(rgb(0x3e3e3e))
                    .child(title),
            )
            .children(items.into_iter().map(|(label, value)| {
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(div().w(px(120.0)).text_color(rgb(0x888888)).child(label))
                    .child(div().flex_1().text_color(rgb(0xcccccc)).child(value))
            }))
    }

    fn render_yaml_section(resource: &ResourceItem) -> impl IntoElement {
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
                    .text_color(rgb(0xffffff))
                    .pb_2()
                    .border_b_1()
                    .border_color(rgb(0x3e3e3e))
                    .child("YAML"),
            )
            .child(
                div()
                    .p_3()
                    .bg(rgb(0x252526))
                    .rounded_md()
                    .text_xs()
                    .font_family("'JetBrains Mono', 'Fira Code', 'SF Mono', Menlo, Monaco, 'Courier New', monospace")
                    .text_color(rgb(0xcccccc))
                    .max_h(px(500.0))
                    .line_height(relative(1.6))
                    .child(yaml),
            )
    }
}
