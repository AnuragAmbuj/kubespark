use futures::channel::oneshot;
use gpui::prelude::*;
use gpui::{AsyncApp, Context, InteractiveElement, WeakEntity};
use gpui::*;
use log::{info, error};
use std::sync::Arc;

use crate::kubernetes::{KubeClient, ResourceItem, ResourceKind};
use crate::settings::manager::SettingsManager;
use crate::settings::ui::{SettingsPanel, SettingsTab};
use crate::theme::{ThemeColors, ThemeExt};
// Removed unused imports
use crate::ui::{
    ActiveView, DashboardView, DetailView, GlassStyle, LogView, ResourceListView, Sidebar,
};

pub struct KubeSparkApp {
    kube_client: Arc<KubeClient>,
    active_view: ActiveView,
    selected_namespace: Option<String>,
    resources: Vec<ResourceItem>,
    selected_resource: Option<ResourceItem>,
    connection_status: ConnectionStatus,
    sidebar_width: Pixels,
    is_sidebar_collapsed: bool,
    detail_width: Pixels,
    show_detail: bool,
    settings_manager: Arc<SettingsManager>,
    show_settings: bool,
    available_contexts: Vec<String>,
    current_context: String,
    show_context_menu: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

impl KubeSparkApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let kube_client = Arc::new(KubeClient::new());
        
        info!("Initializing KubeSparkApp");

        // Spawn initialization task
        let client_clone = kube_client.clone();
        let mut cx_async = cx.to_async();
        let (tx, rx) = oneshot::channel();
        
        tokio::spawn(async move {
            let contexts = KubeClient::list_contexts().await.unwrap_or_default();
            let current = KubeClient::get_current_context().await.unwrap_or_else(|_| "Error".to_string());
            let _ = client_clone.connect().await;
            let _ = tx.send((contexts, current));
        });

        cx.spawn(move |this: WeakEntity<KubeSparkApp>, _cx: &mut AsyncApp| async move {
            if let Ok((contexts, current)) = rx.await {
                let _ = this.update(&mut cx_async, |app, cx| {
                    app.available_contexts = contexts;
                    app.current_context = current;
                    cx.notify();
                });
            }
        }).detach();

        // 5. Return Initial State
        let settings_manager = Arc::new(SettingsManager::new());
        let is_sidebar_collapsed = settings_manager.get_settings().appearance.sidebar_collapsed;

        Self {
            kube_client,
            settings_manager,
            active_view: ActiveView::Dashboard,
            selected_namespace: None,
            selected_resource: None,
            connection_status: ConnectionStatus::Disconnected,
            sidebar_width: px(220.0),
            is_sidebar_collapsed,
            detail_width: px(400.0),
            show_detail: false,
            show_settings: false,
            // init_receiver: Some(rx), // Removed from struct definition? No, waiting for it in spawn. 
            // Wait, if I handle rx in spawn, I don't need to store it?
            // Ah, previous code stored it. But my new spawn logic consumes rx immediately in the async block.
            // But if `init_receiver` IS in the struct, I must provide it or None.
            // Let's assume it IS NOT in the struct anymore or set to None?
            // The previous error didn't mention missing `init_receiver`. 
            // It mentioned missing: `selected_resource`, `connection_status`, etc.
            // I'll assume `init_receiver` is NOT a field, or I'll set it to None if I see it in struct def.
            // Let's look at `impl KubeSparkApp`. 
            // I'll rely on the error message which listed missing fields. 
            // It listed: `selected_resource`, `connection_status`, `sidebar_width`, `is_sidebar_collapsed`, `detail_width`, `show_detail`, `settings_manager`, `show_settings`, `available_contexts`, `current_context`, `show_context_menu`.
            // It did NOT list `init_receiver`. So I won't include it.
            available_contexts: Vec::new(),
            current_context: "Loading...".to_string(),
            show_context_menu: false,
            resources: vec![
                ResourceItem {
                    kind: ResourceKind::Node,
                    name: "node-1".to_string(),
                    namespace: None,
                    status: "Ready".to_string(),
                    age: "5d".to_string(),
                    restart_count: None,
                    node_name: None,
                    pod_ip: None,
                    metadata: serde_json::json!({}),
                },
                ResourceItem {
                    kind: ResourceKind::Node,
                    name: "node-2".to_string(),
                    namespace: None,
                    status: "Ready".to_string(),
                    age: "5d".to_string(),
                    restart_count: None,
                    node_name: None,
                    pod_ip: None,
                    metadata: serde_json::json!({}),
                },
                ResourceItem {
                    kind: ResourceKind::Node,
                    name: "node-3".to_string(),
                    namespace: None,
                    status: "Ready".to_string(),
                    age: "5d".to_string(),
                    restart_count: None,
                    node_name: None,
                    pod_ip: None,
                    metadata: serde_json::json!({}),
                },
                ResourceItem {
                    kind: ResourceKind::Pod,
                    name: "nginx-deployment-5d59d67564-abcde".to_string(),
                    namespace: Some("default".to_string()),
                    status: "Running".to_string(),
                    age: "2h".to_string(),
                    restart_count: None,
                    node_name: None,
                    pod_ip: None,
                    metadata: serde_json::json!({}),
                },
                ResourceItem {
                    kind: ResourceKind::Pod,
                    name: "coredns-7c65d65c69-2f5s2".to_string(),
                    namespace: Some("kube-system".to_string()),
                    status: "Running".to_string(),
                    age: "5d".to_string(),
                    restart_count: None,
                    node_name: None,
                    pod_ip: None,
                    metadata: serde_json::json!({}),
                },
            ],
        }
    }

    pub fn toggle_settings(&mut self) {
        self.show_settings = !self.show_settings;
    }
    
    pub fn toggle_context_menu(&mut self) {
        info!("Toggling context menu");
        self.show_context_menu = !self.show_context_menu;
    }

    pub fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        info!("Toggling sidebar");
        self.is_sidebar_collapsed = !self.is_sidebar_collapsed;
        
        let collapsed = self.is_sidebar_collapsed;
        self.settings_manager.update_settings(move |s| {
            s.appearance.sidebar_collapsed = collapsed;
        }).ok();
        
        cx.notify();
    }

    pub fn switch_context(&mut self, ctx_name: String, cx: &mut Context<Self>) {
        info!("Switching context to: {}", ctx_name);
        self.show_context_menu = false;
        self.current_context = ctx_name.clone();
        
        let saved_ctx = ctx_name.clone();
        self.settings_manager.update_settings(move |s| {
            s.kubernetes.context = saved_ctx;
        }).ok();
        
        cx.notify();
        let client = self.kube_client.clone();
        let mut cx_async = cx.to_async();
        
        cx.spawn(move |this: WeakEntity<KubeSparkApp>, _cx: &mut AsyncApp| async move {
            let _ = client.connect_with_context(&ctx_name).await;
            let _ = this.update(&mut cx_async, |app, cx| {
                app.connection_status = ConnectionStatus::Connected;
                app.refresh(cx);
                cx.notify();
            });
        }).detach();
    }

    pub fn select_resource_kind(&mut self, kind: ResourceKind, cx: &mut Context<Self>) {
        self.active_view = ActiveView::Resources(kind);
        self.selected_resource = None;
        self.show_detail = false;
        self.refresh(cx);
    }

    pub fn select_resource(&mut self, resource: ResourceItem) {
        self.selected_resource = Some(resource);
        self.show_detail = true;
    }

    pub fn close_detail(&mut self) {
        self.show_detail = false;
        self.selected_resource = None;
    }

    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        let kind = match &self.active_view {
            ActiveView::Resources(k) => k.clone(),
            _ => return,
        };
        
        info!("Refreshing resources: {:?}", kind);
        
        let namespace = self.selected_namespace.clone();
        let client = self.kube_client.clone();
        let mut cx_async = cx.to_async();
        
        cx.spawn(move |this: WeakEntity<KubeSparkApp>, _cx: &mut AsyncApp| async move {
            match client.list_resources(kind, namespace.as_deref()).await {
                Ok(items) => {
                    this.update(&mut cx_async, |app, cx| {
                         app.resources = items;
                         cx.notify();
                    }).ok();
                }
                Err(e) => {
                    error!("Failed to list resources: {}", e);
                }
            }
        }).detach();
    }

    fn render_title_bar(&self, cx: &mut Context<Self>, colors: &ThemeColors) -> impl IntoElement {
        let is_macos = cfg!(target_os = "macos");

        div()
            .h(px(38.0))
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            // Enabling Window Dragging
            .on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(if is_macos {
                        div().w(px(70.0))
                    } else {
                        div().w(px(0.0))
                    })
                    // Sidebar Toggle
                    .child(
                        div()
                            .id("sidebar-toggle")
                            .w(px(28.0))
                            .h(px(28.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_md()
                            .cursor(CursorStyle::PointingHand)
                            .bg(colors.bg_element)
                            .hover({
                                let cloned = colors.clone();
                                move |style| style.bg(cloned.bg_element_hover)
                            })
                            .active({
                                let cloned = colors.clone();
                                move |style| style.bg(cloned.bg_element_active)
                            })
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_sidebar(cx);
                            }))
                            .child(
                                // Sidebar Icon Construction
                                div()
                                    .w(px(18.0))
                                    .h(px(14.0))
                                    .border_1()
                                    .border_color(colors.text_secondary)
                                    .rounded_sm()
                                    .flex()
                                    .child(
                                        div()
                                            .w(px(6.0))
                                            .h_full()
                                            .border_r_1()
                                            .border_color(colors.text_secondary)
                                    )
                            ),
                    ),
            )
            // Center title
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(colors.text_primary)
                    .child("KubeSpark"),
            )
            // Right side controls (Settings + Window Controls on Linux)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Settings Button
                    .child(
                        div()
                            .id("settings-button")
                            .w(px(28.0))
                            .h(px(28.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_md()
                            .cursor(CursorStyle::PointingHand)
                            .bg(colors.bg_element)
                            .hover({
                                let cloned = colors.clone();
                                move |style| style.bg(cloned.bg_element_hover)
                            })
                            .active({
                                let cloned = colors.clone();
                                move |style| style.bg(cloned.bg_element_active)
                            })
                            .child(div().text_sm().text_color(colors.text_secondary).child("⚙"))
                             // Stop propagation
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .on_click(cx.listener(|this, _, _, _| {
                                this.toggle_settings();
                            })),
                    )
                    // Window Controls for Linux (Close, Min, Max)
                    .children(if !is_macos {
                        Some(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .ml_4()
                                // Minimize
                                .child(
                                    div()
                                        .id("minimize-button")
                                        .w(px(24.0))
                                        .h(px(24.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded_md()
                                        .cursor(CursorStyle::PointingHand)
                                        .bg(colors.bg_element)
                                        .hover({
                                            let cloned = colors.clone();
                                            move |style| style.bg(cloned.bg_element_hover).text_color(cloned.text_primary)
                                        })
                                        .child(div().text_xs().text_color(colors.text_primary).mb_1().child("_"))
                                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                                        .on_click(|_, window, _| {
                                            window.minimize_window();
                                        }),
                                )
                                // Maximize / Restore
                                .child(
                                    div()
                                        .id("maximize-button")
                                        .w(px(24.0))
                                        .h(px(24.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded_md()
                                        .cursor(CursorStyle::PointingHand)
                                        .bg(colors.bg_element)
                                        .hover({
                                            let cloned = colors.clone();
                                            move |style| style.bg(cloned.bg_element_hover).text_color(cloned.text_primary)
                                        })
                                        .child(div().text_xs().text_color(colors.text_primary).child("□"))
                                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                                        .on_click(|_, window, _| {
                                            window.zoom_window();
                                        }),
                                )
                                // Close
                                .child(
                                    div()
                                        .id("close-button")
                                        .w(px(24.0))
                                        .h(px(24.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded_md()
                                        .cursor(CursorStyle::PointingHand)
                                        .bg(colors.bg_element)
                                        .hover({
                                            let cloned = colors.clone();
                                            move |style| {
                                                style.bg(cloned.status_error).text_color(cloned.text_inverse)
                                            }
                                        })
                                        .child(div().text_xs().child("✕"))
                                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                                        .on_click(cx.listener(|_, _, _, cx| cx.quit())),
                                ),
                        )
                    } else {
                        None
                    }),
            )
    }
}

impl Render for KubeSparkApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Get current settings for styling
        let settings = self.settings_manager.get_settings();
        let glass_style = GlassStyle::new(
            settings.appearance.glassomorphism_enabled,
            settings.appearance.blur_intensity,
            settings.appearance.glass_opacity,
        );

        // If settings are shown, render settings panel instead
        if self.show_settings {
            return div()
                .flex()
                .flex_col()
                .size_full()
                .bg(settings.appearance.theme.colors().bg_app)
                .text_color(settings.appearance.theme.colors().text_primary)
                .child(self.render_title_bar(cx, &settings.appearance.theme.colors()))
                .child(
                    SettingsPanel::render_panel(
                        self.settings_manager.clone(),
                        settings,
                        SettingsTab::Appearance,
                        cx.listener(|this, _, _, _| {
                            this.toggle_settings();
                        }),
                        {
                            let weak = cx.entity().downgrade();
                            move |_, cx| {
                                let _ = weak.update(cx, |_, cx| cx.notify());
                            }
                        },
                    )
                )
                .into_any_element();
        }

        let is_sidebar_collapsed = self.is_sidebar_collapsed;
        let sidebar_width = if is_sidebar_collapsed {
            px(50.0)
        } else {
            self.sidebar_width
        };
        let detail_width = self.detail_width;
        let show_detail = self.show_detail;
        let selected_resource = self.selected_resource.clone();

        let active_view = self.active_view.clone();
        let is_sidebar_collapsed = self.is_sidebar_collapsed;

        // Get theme colors
        let colors = settings.appearance.theme.colors();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(colors.bg_app)
            .text_color(colors.text_primary)
            .font_family(
                "'SF Pro Display', 'Inter', 'Segoe UI', system-ui, -apple-system, sans-serif",
            )
            .child(self.render_title_bar(cx, &colors))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .child(div().w(sidebar_width).h_full().child({
                        let weak = cx.entity().downgrade();
                        let weak_ctx_menu = weak.clone();
                        let weak_switch = weak.clone();
                        
                        Sidebar::new(
                            glass_style,
                            &settings.appearance,
                            &active_view,
                            is_sidebar_collapsed,
                            self.current_context.clone(),
                            self.available_contexts.clone(),
                            self.show_context_menu,
                            move |view, _win, cx| {
                                let _ = weak.update(cx, |this, cx| {
                                    if let ActiveView::Resources(kind) = view {
                                         this.select_resource_kind(kind, cx);
                                    } else {
                                         this.active_view = view.clone();
                                         this.selected_resource = None;
                                         this.show_detail = false;
                                         cx.notify();
                                    }
                                });
                            },
                            move |_win, cx| {
                                let _ = weak_ctx_menu.update(cx, |this, cx| {
                                    this.toggle_context_menu();
                                    cx.notify();
                                });
                            },
                            move |ctx, _win, cx| {
                                let _ = weak_switch.update(cx, |this, cx| {
                                    this.switch_context(ctx, cx);
                                    this.show_context_menu = false;
                                    cx.notify();
                                });
                            },
                        )
                    }))
                    .child(div().flex_1().h_full().child(match active_view {
                        ActiveView::Dashboard => {
                            DashboardView::new(glass_style, &colors).render().into_any_element()
                        }
                        ActiveView::Resources(kind) => {
                            let filtered = self
                                .resources
                                .iter()
                                .filter(|r| r.kind == kind)
                                .cloned()
                                .collect();

                            let weak = cx.entity().downgrade();
                            ResourceListView::new(
                                Some(kind),
                                filtered,
                                glass_style,
                                move |resource, _win, cx| {
                                    let _ = weak.update(cx, |this, cx| {
                                        this.selected_resource = Some(resource);
                                        this.show_detail = true;
                                        cx.notify();
                                    });
                                },
                                &colors,
                            )
                            .into_any_element()
                        }
                        ActiveView::Logs(log_view) => {
                            let weak = cx.entity().downgrade();
                            log_view
                                .render(move |_event, _win, cx| {
                                    let _ = weak.update(cx, |this, cx| {
                                        this.active_view = ActiveView::Resources(ResourceKind::Pod);
                                        cx.notify();
                                    });
                                }, &colors)
                                .into_any_element()
                        }
                    }))
                    .child(if show_detail {
                        let weak = cx.entity().downgrade();
                        let selected = selected_resource.clone();
                        let namespace = selected
                            .as_ref()
                            .and_then(|r| r.namespace.clone())
                            .unwrap_or_default();

                        // Create async context for spawning tasks
                        let async_cx = cx.to_async();

                        div().w(detail_width).h_full().child(DetailView::new(
                            selected_resource.clone(),
                            glass_style,
                            move |pod_name, _win, cx| {
                                // Use captured async_cx for async work
                                let ns = namespace.clone();
                                let pod = pod_name.clone();
                                let weak_for_spawn = weak.clone();

                                let _ = weak.clone().update(cx, |this, cx| {
                                    // Update view
                                    let log_view =
                                        LogView::new(pod.clone(), ns.clone(), glass_style);
                                    this.active_view = ActiveView::Logs(log_view);
                                    cx.notify();

                                    let client = this.kube_client.clone();
                                    let pod = pod.clone();
                                    let ns = ns.clone();

                                    let mut fetch_cx = async_cx.clone();
                                    let (tx, rx) = oneshot::channel();
                                    
                                    // Background fetch (Send-safe)
                                    tokio::spawn(async move {
                                        let result = client.get_pod_logs(&ns, &pod, None).await;
                                        let logs = match result {
                                            Ok(l) => l.lines().map(|s| s.to_string()).collect(),
                                            Err(e) => vec![format!("Error: {}", e)],
                                        };
                                        let _ = tx.send(logs);
                                    });

                                    // Local update (UI thread)
                                    cx.spawn(move |_this: WeakEntity<KubeSparkApp>, _cx: &mut AsyncApp| async move {
                                        if let Ok(logs) = rx.await {
                                            let _ = weak_for_spawn.update(&mut fetch_cx, |this, cx| {
                                                if let ActiveView::Logs(view) = &mut this.active_view {
                                                     view.set_logs(logs);
                                                     cx.notify();
                                                }
                                            });
                                        }
                                    }).detach();
                                });
                            },
                            &colors,
                        ))
                    } else {
                        div()
                    }),
            )
            .into_any_element()
    }
}
