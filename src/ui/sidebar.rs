use crate::kubernetes::ResourceKind;
use crate::settings::AppearanceSettings;
use crate::ui::glass::{GlassExt, GlassStyle};
use crate::ui::ActiveView;
use gpui::prelude::*;
use gpui::*;
use std::collections::HashMap;

pub struct Sidebar;

impl Sidebar {
    pub fn new(
        glass_style: GlassStyle,
        settings: &AppearanceSettings,
        active_view: &ActiveView,
        is_collapsed: bool,
        current_context: String,
        available_contexts: Vec<String>,
        show_context_menu: bool,
        on_select: impl Fn(ActiveView, &mut Window, &mut App) + 'static + Clone,
        on_toggle: impl Fn(&mut Window, &mut App) + 'static + Clone,
        on_toggle_context_menu: impl Fn(&mut Window, &mut App) + 'static + Clone,
        on_switch_context: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        let categories = Self::categorize_resources();
        let show_icons = settings.show_sidebar_icons;
        let active_view = active_view.clone();

        // Clone for callbacks
        let on_select_overview = on_select.clone();
        let on_toggle_menu = on_toggle_context_menu.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .glass_sidebar(glass_style)
            .pt_3()
            .pb_2()
            .px_2()
            .gap_3()
            // Header / Toggle
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .pb_2()
                    .child(if !is_collapsed {
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x888888))
                            .child("CLUSTER")
                    } else {
                        div()
                    })
                    .child(
                        div()
                            .id("sidebar-toggle")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(20.0))
                            .h(px(20.0))
                            .rounded_sm()
                            .cursor(CursorStyle::PointingHand)
                            .hover(|style| style.bg(rgb(0x3e3e3e)))
                            .child(if is_collapsed { "»" } else { "«" })
                            .on_click(move |_, win, app| on_toggle(win, app)),
                    ),
            )
            // Context Selector (only if not collapsed)
            .child(if !is_collapsed {
                let on_toggle_menu = on_toggle_menu.clone();
                div()
                    .flex()
                    .flex_col()
                    .px_3()
                    .gap_1()
                    .child(
                        div()
                            .id("context-selector")
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x2a2a2e))
                            .cursor(CursorStyle::PointingHand)
                            .hover(|style| style.bg(rgb(0x3e3e3e)))
                            .child(
                                div().flex().items_center().gap_2().child("☸").child(
                                    div()
                                        .text_sm()
                                        .text_ellipsis()
                                        .child(current_context.clone()),
                                ),
                            )
                            .child(if show_context_menu { "▲" } else { "▼" })
                            .on_click(move |_, win, app| on_toggle_menu(win, app)),
                    )
                    .children(if show_context_menu {
                        Some(
                            div()
                                .flex()
                                .flex_col()
                                .bg(rgb(0x1e1e1e))
                                .rounded_md()
                                .border_1()
                                .border_color(rgb(0x3e3e3e))
                                .p_1()
                                .mt_1()
                                .children(available_contexts.into_iter().map(move |ctx| {
                                    let on_switch = on_switch_context.clone();
                                    let ctx_name = ctx.clone();
                                    let is_current = ctx == current_context;
                                    let item = div()
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .cursor(CursorStyle::PointingHand)
                                        .hover(|style| style.bg(rgb(0x2a2a2e)))
                                        .bg(if is_current {
                                            rgb(0x3e3e3e)
                                        } else {
                                            rgb(0x00000000)
                                        }) // Transparent if not active
                                        .child(ctx);

                                    item.on_mouse_down(
                                        gpui::MouseButton::Left,
                                        move |_, win, app| on_switch(ctx_name.clone(), win, app),
                                    )
                                })),
                        )
                    } else {
                        None
                    })
            } else {
                div()
            })
            // Overview
            .child(div().flex().flex_col().gap_px().child({
                let is_active = matches!(active_view, ActiveView::Dashboard);
                let mut item = div()
                    .id("overview")
                    .flex()
                    .items_center()
                    .px_3()
                    .py_1()
                    .mx_1()
                    .gap_2()
                    .text_sm()
                    .rounded_md()
                    .cursor(CursorStyle::PointingHand);

                if is_active {
                    item = item.bg(rgb(0x37373d)).text_color(rgb(0xffffff));
                } else {
                    item = item
                        .text_color(rgb(0xcccccc))
                        .hover(|style| style.bg(rgb(0x2a2a2e)).text_color(rgb(0xffffff)));
                }

                if is_collapsed {
                    item = item.justify_center().px_0().mx_0();
                    item.child("📊")
                } else {
                    item.child(if show_icons { "📊" } else { "" })
                        .child("Overview")
                }
                .on_click(move |_, win, app| on_select_overview(ActiveView::Dashboard, win, app))
            }))
            .child(if !is_collapsed {
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x888888))
                    .px_3()
                    .pb_1()
                    .pt_2()
                    .child("RESOURCES")
            } else {
                div().pt_2()
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .children(categories.into_iter().map(move |(category, resources)| {
                        Self::render_category(
                            category.to_string(),
                            resources,
                            show_icons,
                            &active_view,
                            is_collapsed,
                            on_select.clone(),
                        )
                    })),
            )
    }

    fn categorize_resources() -> Vec<(String, Vec<ResourceKind>)> {
        let mut categories: HashMap<String, Vec<ResourceKind>> = HashMap::new();

        for kind in ResourceKind::all() {
            categories
                .entry(kind.category().to_string())
                .or_insert_with(Vec::new)
                .push(kind);
        }

        let mut result: Vec<_> = categories.into_iter().collect();
        result.sort_by_key(|(category, _)| match category.as_str() {
            "Cluster" => 0,
            "Workloads" => 1,
            "Network" => 2,
            "Config" => 3,
            "Storage" => 4,
            _ => 5,
        });

        result
    }

    fn render_category(
        category: String,
        resources: Vec<ResourceKind>,
        show_icons: bool,
        active_view: &ActiveView,
        is_collapsed: bool,
        on_select: impl Fn(ActiveView, &mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_px()
            .child(if !is_collapsed {
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x666666))
                    .px_3()
                    .pb_1()
                    .child(category.to_uppercase())
            } else {
                div().pb_1()
            })
            .children(resources.into_iter().map(move |kind| {
                Self::render_resource_item(
                    kind,
                    show_icons,
                    active_view,
                    is_collapsed,
                    on_select.clone(),
                )
            }))
    }

    fn render_resource_item(
        kind: ResourceKind,
        show_icons: bool,
        active_view: &ActiveView,
        is_collapsed: bool,
        on_select: impl Fn(ActiveView, &mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        let display_name = kind.display_name().to_string();
        let icon_str = match kind {
            ResourceKind::Namespace => "📦",
            ResourceKind::Pod => "🧊",
            ResourceKind::Deployment => "🚀",
            ResourceKind::StatefulSet => "💾",
            ResourceKind::DaemonSet => "😈",
            ResourceKind::ReplicaSet => "👯",
            ResourceKind::Service => "🔌",
            ResourceKind::Job => "⏱️",
            ResourceKind::CronJob => "⏰",
            ResourceKind::ConfigMap => "⚙️",
            ResourceKind::Secret => "🔒",
            ResourceKind::Ingress => "🌐",
            ResourceKind::PersistentVolume => "💿",
            ResourceKind::PersistentVolumeClaim => "📑",
            ResourceKind::Node => "🖥️",
        };

        let icon = if show_icons || is_collapsed {
            Some(icon_str)
        } else {
            None
        };

        let is_active = if let ActiveView::Resources(k) = active_view {
            k == &kind
        } else {
            false
        };

        let kind_clone = kind.clone();

        let mut item = div()
            .id(display_name.clone())
            .flex()
            .items_center()
            .px_3()
            .py_1()
            .mx_1()
            .gap_2()
            .text_sm()
            .rounded_md()
            .cursor(CursorStyle::PointingHand);

        if is_active {
            item = item.bg(rgb(0x37373d)).text_color(rgb(0xffffff));
        } else {
            item = item
                .text_color(rgb(0xcccccc))
                .hover(|style| style.bg(rgb(0x2a2a2e)).text_color(rgb(0xffffff)));
        }

        if is_collapsed {
            item = item.justify_center().px_0().mx_0();
            item.child(icon_str)
        } else {
            item.children(icon.map(|i| div().child(i)))
                .child(display_name)
        }
        .on_click(move |_, win, app| on_select(ActiveView::Resources(kind_clone.clone()), win, app))
    }
}
