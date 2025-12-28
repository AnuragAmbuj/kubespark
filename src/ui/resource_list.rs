use crate::kubernetes::{ResourceItem, ResourceKind};
use crate::theme::ThemeColors;
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
        colors: &ThemeColors,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .glass_panel(glass_style, colors)
            .child(Self::render_header(
                selected_kind.clone(),
                resources.len(),
                colors,
            ))
            .child(Self::render_table_header(selected_kind.clone(), colors))
            .child(div().flex().flex_col().flex_1().children(
                resources.into_iter().enumerate().map({
                    let colors = colors.clone();
                    move |(i, r)| Self::render_row(i, r, on_select.clone(), &colors)
                }),
            ))
    }

    fn render_header(
        selected_kind: Option<ResourceKind>,
        count: usize,
        colors: &ThemeColors,
    ) -> impl IntoElement {
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
                            .child(title),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(colors.text_muted)
                            .child(format!("({})", count)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(Self::render_button("Refresh".to_string(), colors)),
            )
    }

    fn render_button(label: String, colors: &ThemeColors) -> impl IntoElement {
        div()
            .px_3()
            .py_1()
            .text_sm()
            .text_color(colors.text_secondary)
            .bg(colors.bg_element)
            .rounded_md()
            .hover(move |style| style.bg(colors.bg_element_hover))
            .cursor(CursorStyle::PointingHand)
            .child(label)
    }

    fn render_table_header(
        selected_kind: Option<ResourceKind>,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let is_pod = matches!(selected_kind, Some(ResourceKind::Pod));

        div()
            .flex()
            .items_center()
            .h(px(32.0))
            .px_4()
            // Using slightly darker panel background or just panel background?
            // Let's use bg_panel or sidebar color for header distinction
            .bg(colors.bg_panel)
            .border_b_1()
            .border_color(colors.border)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(colors.text_muted)
            .child(div().flex_1().child("NAME"))
            .child(div().w(px(150.0)).child("NAMESPACE"))
            .child(div().w(px(120.0)).child("STATUS"))
            .child(if is_pod {
                div().w(px(80.0)).child("RESTARTS")
            } else {
                div().w(px(0.0))
            })
            .child(div().w(px(80.0)).child("AGE"))
    }

    fn render_row(
        index: usize,
        resource: ResourceItem,
        on_select: impl Fn(ResourceItem, &mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let is_even = index % 2 == 0;
        let bg_color = if is_even {
            colors.bg_sidebar
        } else {
            colors.bg_app
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
            .border_color(Hsla::from(colors.border).opacity(0.05)) // Subtle border
            .text_sm()
            .text_color(colors.text_secondary)
            .cursor(CursorStyle::PointingHand)
            .hover({
                let cloned = colors.clone();
                move |style| {
                    style
                        .bg(cloned.bg_element_hover)
                        .text_color(cloned.text_primary)
                }
            })
            .on_click(move |_, win, app| on_select(resource_clone.clone(), win, app))
            .child(div().flex_1().child(resource.name.clone()))
            .child(
                div()
                    .w(px(150.0))
                    .text_color(colors.text_muted)
                    .child(resource.namespace.clone().unwrap_or_default()),
            )
            .child(
                div()
                    .w(px(120.0))
                    .child(Self::render_status(resource.status.clone(), colors)),
            )
            .child(if resource.kind == ResourceKind::Pod {
                div()
                    .w(px(80.0))
                    .child(resource.restart_count.unwrap_or(0).to_string())
            } else {
                div().w(px(0.0))
            })
            .child(div().w(px(80.0)).child(resource.age.clone()))
    }

    fn render_status(status: String, colors: &ThemeColors) -> impl IntoElement {
        let color = match status.as_str() {
            "Running" | "Ready" | "Succeeded" => colors.status_ok,
            "Pending" | "ContainerCreating" => colors.status_warning,
            "Failed" | "Error" | "CrashLoopBackOff" => colors.status_error,
            _ => colors.text_muted,
        };

        div().text_color(color).child(status)
    }
}
