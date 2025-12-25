use crate::kubernetes::{ResourceItem, ResourceKind};
use crate::ui::glass::{GlassExt, GlassStyle};
use gpui::prelude::*;
use gpui::*;

pub struct ResourceListView;

impl ResourceListView {
    pub fn new(
        selected_kind: Option<ResourceKind>,
        resources: Vec<ResourceItem>,
        glass_style: GlassStyle,
        on_select: impl Fn(ResourceItem, &mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .glass_panel(glass_style)
            .child(Self::render_header(selected_kind.clone(), resources.len()))
            .child(Self::render_table_header())
            .child(
                div().flex().flex_col().flex_1().children(
                    resources
                        .into_iter()
                        .enumerate()
                        .map(|(i, r)| Self::render_row(i, r, on_select.clone())),
                ),
            )
    }

    fn render_header(selected_kind: Option<ResourceKind>, count: usize) -> impl IntoElement {
        let title = selected_kind
            .as_ref()
            .map(|k| k.display_name().to_string())
            .unwrap_or_else(|| "Select a resource".to_string());

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
                            .child(title),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x888888))
                            .child(format!("({})", count)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(Self::render_button("Refresh".to_string())),
            )
    }

    fn render_button(label: String) -> impl IntoElement {
        div()
            .px_3()
            .py_1()
            .text_sm()
            .text_color(rgb(0xcccccc))
            .bg(rgb(0x3e3e3e))
            .rounded_md()
            .hover(|style| style.bg(rgb(0x505050)))
            .cursor(CursorStyle::PointingHand)
            .child(label)
    }

    fn render_table_header() -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .h(px(32.0))
            .px_4()
            .bg(rgb(0x2d2d30))
            .border_b_1()
            .border_color(rgb(0x3e3e3e))
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(0x888888))
            .child(div().flex_1().child("NAME"))
            .child(div().w(px(150.0)).child("NAMESPACE"))
            .child(div().w(px(120.0)).child("STATUS"))
            .child(div().w(px(80.0)).child("AGE"))
    }

    fn render_row(
        index: usize,
        resource: ResourceItem,
        on_select: impl Fn(ResourceItem, &mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        let is_even = index % 2 == 0;
        let bg_color = if is_even {
            rgb(0x252526)
        } else {
            rgb(0x1e1e1e)
        };

        let resource_clone = resource.clone();
        let resource_id = resource.name.clone();

        div()
            .id(resource_id)
            .flex()
            .items_center()
            .h(px(36.0))
            .px_4()
            .bg(bg_color)
            .border_b_1()
            .border_color(hsla(0.0, 0.0, 1.0, 0.05))
            .text_sm()
            .text_color(rgb(0xcccccc))
            .cursor(CursorStyle::PointingHand)
            .hover(|style| style.bg(rgb(0x2a2d2e)).text_color(rgb(0xffffff)))
            .on_click(move |_, win, app| on_select(resource_clone.clone(), win, app))
            .child(div().flex_1().child(resource.name.clone()))
            .child(
                div()
                    .w(px(150.0))
                    .text_color(rgb(0xaaaaaa))
                    .child(resource.namespace.clone().unwrap_or_default()),
            )
            .child(
                div()
                    .w(px(120.0))
                    .child(Self::render_status(resource.status.clone())),
            )
            .child(div().w(px(80.0)).child(resource.age.clone()))
    }

    fn render_status(status: String) -> impl IntoElement {
        let color = match status.as_str() {
            "Running" | "Ready" | "Succeeded" => rgb(0x4ade80), // Green
            "Pending" | "ContainerCreating" => rgb(0xfacc15),   // Yellow
            "Failed" | "Error" | "CrashLoopBackOff" => rgb(0xf87171), // Red
            _ => rgb(0xcccccc),
        };

        div().text_color(color).child(status)
    }
}
