use super::config::*;
use super::manager::SettingsManager;
use crate::theme::{ThemeColors, ThemeExt};
use gpui::prelude::*;
use gpui::{InteractiveElement, *};

use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    Appearance,
    Kubernetes,
    Editor,
    Window,
}

impl SettingsTab {
    pub fn all() -> Vec<SettingsTab> {
        vec![
            SettingsTab::Appearance,
            SettingsTab::Kubernetes,
            SettingsTab::Editor,
            SettingsTab::Window,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            SettingsTab::Appearance => "Appearance",
            SettingsTab::Kubernetes => "Kubernetes",
            SettingsTab::Editor => "Editor",
            SettingsTab::Window => "Window",
        }
    }
}

pub struct SettingsPanel;

impl SettingsPanel {
    pub fn new(
        settings_manager: Arc<SettingsManager>,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        on_change: impl Fn(&mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        let settings = settings_manager.get_settings();
        let active_tab = SettingsTab::Appearance;

        Self::render_panel(settings_manager, settings, active_tab, on_close, on_change)
    }

    pub fn render_panel(
        settings_manager: Arc<SettingsManager>,
        settings: AppSettings,
        active_tab: SettingsTab,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        on_change: impl Fn(&mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        let colors = settings.appearance.theme.colors();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(colors.bg_app)
            .child(Self::render_header(
                settings_manager.clone(),
                on_close,
                &colors,
            ))
            .child(Self::render_tab_bar(active_tab, &colors))
            .child(
                div().flex_1().child(match active_tab {
                    SettingsTab::Appearance => Self::render_appearance_tab(
                        settings_manager.clone(),
                        &settings.appearance,
                        on_change,
                        &colors,
                    )
                    .into_any_element(),
                    SettingsTab::Kubernetes => Self::render_kubernetes_tab(
                        settings_manager.clone(),
                        &settings.kubernetes,
                        on_change,
                        &colors,
                    )
                    .into_any_element(),
                    SettingsTab::Editor => Self::render_editor_tab(
                        settings_manager.clone(),
                        &settings.editor,
                        on_change,
                        &colors,
                    )
                    .into_any_element(),
                    SettingsTab::Window => Self::render_window_tab(
                        settings_manager.clone(),
                        &settings.window,
                        on_change,
                        &colors,
                    )
                    .into_any_element(),
                }),
            )
    }

    fn render_header(
        settings_manager: Arc<SettingsManager>,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .px_6()
            .h(px(48.0))
            .border_b_1()
            .border_color(colors.border)
            .bg(colors.bg_panel)
            // Left side with back button and title
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    // Close/Back button
                    .child(
                        div()
                            .id("close-settings")
                            .w(px(32.0))
                            .h(px(32.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_md()
                            .cursor(CursorStyle::PointingHand)
                            .bg(colors.bg_element)
                            .hover(move |style| style.bg(colors.bg_element_hover))
                            .active(move |style| style.bg(colors.bg_element_active))
                            .child(
                                div()
                                    .text_base()
                                    .text_color(colors.text_secondary)
                                    .child("←"),
                            )
                            .on_click(on_close),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors.text_primary)
                            .child("Settings"),
                    ),
            )
            .child(
                div()
                    .id("reset-defaults-button")
                    .text_sm()
                    .text_color(colors.accent)
                    .cursor(CursorStyle::PointingHand)
                    .hover(move |style| style.text_color(colors.accent_hover))
                    .child("Reset to Defaults")
                    .on_click(move |_, _, _| {
                        let _ = settings_manager.reset_to_defaults();
                    }),
            )
    }

    fn render_tab_bar(active_tab: SettingsTab, colors: &ThemeColors) -> impl IntoElement {
        let tabs = SettingsTab::all();

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_6()
            .py_4()
            .border_b_1()
            .border_color(colors.border)
            .bg(colors.bg_sidebar)
            .children(tabs.into_iter().map(move |tab| {
                let is_active = tab == active_tab;
                let tab_name = tab.name().to_string();
                // Clone colors (ThemeColors is cloneable)
                let bg_active = colors.bg_element_active;
                let bg_sidebar = colors.bg_sidebar;
                let text_active = colors.text_inverse;
                let text_inactive = colors.text_secondary;

                div()
                    .id(tab_name.clone())
                    .px_4()
                    .py_2()
                    .text_sm()
                    .font_weight(if is_active {
                        FontWeight::SEMIBOLD
                    } else {
                        FontWeight::NORMAL
                    })
                    .text_color(if is_active {
                        text_active
                    } else {
                        text_inactive
                    })
                    .bg(if is_active { bg_active } else { bg_sidebar })
                    .rounded_md()
                    .cursor(CursorStyle::PointingHand)
                    .hover(|style| {
                        if is_active {
                            style
                        } else {
                            style.bg(rgb(0x2d2d30)).text_color(rgb(0xffffff))
                        }
                    })
                    .child(tab_name)
            }))
    }

    fn render_appearance_tab(
        settings_manager: Arc<SettingsManager>,
        settings: &AppearanceSettings,
        on_change: impl Fn(&mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let sm = settings_manager.clone();
        div()
            .flex()
            .flex_col()
            .gap_6()
            .p_6()
            .child(Self::render_section(
                "Glassomorphic Effects",
                vec![
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Enable Glassomorphism",
                            "Apply blur and transparency effects to panels",
                            settings.glassomorphism_enabled,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.appearance.glassomorphism_enabled =
                                        !s.appearance.glassomorphism_enabled
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Blur Intensity",
                            "Adjust the blur amount for glass panels",
                            settings.blur_intensity,
                            0.0,
                            100.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ =
                                        sm.update_settings(|s| s.appearance.blur_intensity = val);
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Glass Opacity",
                            "Control transparency of glass panels",
                            settings.glass_opacity,
                            0.0,
                            1.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ =
                                        sm.update_settings(|s| s.appearance.glass_opacity = val);
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                ],
                colors,
            ))
            .child(Self::render_section(
                "Theme",
                vec![{
                    let sm = sm.clone();
                    let on_change = on_change.clone();
                    Self::render_theme_selector(
                        settings.theme,
                        move |theme, win, cx| {
                            let _ = sm.update_settings(|s| s.appearance.theme = theme);
                            on_change(win, cx);
                        },
                        colors,
                    )
                    .into_any_element()
                }],
                colors,
            ))
            .child(Self::render_section(
                "Typography",
                vec![
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "UI Font Size",
                            "Font size for interface elements",
                            settings.ui_font_size,
                            10.0,
                            20.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ = sm.update_settings(|s| s.appearance.ui_font_size = val);
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Code Font Size",
                            "Font size for YAML and code",
                            settings.code_font_size,
                            10.0,
                            20.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ =
                                        sm.update_settings(|s| s.appearance.code_font_size = val);
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                ],
                colors,
            ))
            .child(Self::render_section(
                "Sidebar",
                vec![
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Show Icons",
                            "Display icons next to resource names",
                            settings.show_sidebar_icons,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.appearance.show_sidebar_icons =
                                        !s.appearance.show_sidebar_icons
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Width",
                            "Sidebar width in pixels",
                            settings.sidebar_width,
                            150.0,
                            400.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ =
                                        sm.update_settings(|s| s.appearance.sidebar_width = val);
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                        .into_any_element()
                    },
                ],
                colors,
            ))
            .child(Self::render_section(
                "Animations",
                vec![
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Enable Animations",
                            "Smooth transitions and effects",
                            settings.animations_enabled,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.appearance.animations_enabled =
                                        !s.appearance.animations_enabled
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                        .into_any_element()
                    },
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Duration",
                            "Animation duration in milliseconds",
                            settings.animation_duration as f32,
                            0.0,
                            500.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ = sm.update_settings(|s| {
                                        s.appearance.animation_duration = val as u32
                                    });
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                        .into_any_element()
                    },
                ],
                colors,
            ))
    }

    fn render_kubernetes_tab(
        settings_manager: Arc<SettingsManager>,
        settings: &KubernetesSettings,
        on_change: impl Fn(&mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let sm = settings_manager.clone();
        div()
            .flex()
            .flex_col()
            .gap_6()
            .p_6()
            .child(Self::render_section(
                "Auto Refresh",
                vec![
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Interval (seconds)",
                            "How often to refresh resource lists (0 = disabled)",
                            settings.auto_refresh_interval as f32,
                            0.0,
                            300.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ = sm.update_settings(|s| {
                                        s.kubernetes.auto_refresh_interval = val as u64
                                    });
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Watch Mode",
                            "Use Kubernetes watch API for real-time updates",
                            settings.watch_mode_enabled,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.kubernetes.watch_mode_enabled =
                                        !s.kubernetes.watch_mode_enabled
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                ],
                colors,
            ))
            .child(Self::render_section(
                "Display",
                vec![
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Show System Namespaces",
                            "Display kube-system and other system namespaces",
                            settings.show_system_namespaces,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.kubernetes.show_system_namespaces =
                                        !s.kubernetes.show_system_namespaces
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Max Items Per Page",
                            "Maximum resources to display per type",
                            settings.max_items_per_page as f32,
                            10.0,
                            500.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ = sm.update_settings(|s| {
                                        s.kubernetes.max_items_per_page = val as usize
                                    });
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Show Metrics",
                            "Display CPU and memory metrics for resources",
                            settings.show_metrics,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.kubernetes.show_metrics = !s.kubernetes.show_metrics
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                ],
                colors,
            ))
            .child(Self::render_section(
                "Notifications",
                vec![{
                    let sm = sm.clone();
                    let on_change = on_change.clone();
                    Self::render_toggle(
                        "Enable Notifications",
                        "Show alerts for Kubernetes events",
                        settings.enable_notifications,
                        move |win, cx| {
                            let _ = sm.update_settings(|s| {
                                s.kubernetes.enable_notifications =
                                    !s.kubernetes.enable_notifications
                            });
                            on_change(win, cx);
                        },
                        colors,
                    )
                    .into_any_element()
                }],
                colors,
            ))
    }

    fn render_editor_tab(
        settings_manager: Arc<SettingsManager>,
        settings: &EditorSettings,
        on_change: impl Fn(&mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let sm = settings_manager.clone();
        div()
            .flex()
            .flex_col()
            .gap_6()
            .p_6()
            .child(Self::render_section(
                "Editor Features",
                vec![
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Syntax Highlighting",
                            "Color YAML syntax elements",
                            settings.syntax_highlighting,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.editor.syntax_highlighting = !s.editor.syntax_highlighting
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Line Numbers",
                            "Show line numbers in YAML editor",
                            settings.show_line_numbers,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.editor.show_line_numbers = !s.editor.show_line_numbers
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "YAML Validation",
                            "Validate YAML syntax as you type",
                            settings.yaml_validation,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.editor.yaml_validation = !s.editor.yaml_validation
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Word Wrap",
                            "Wrap long lines",
                            settings.word_wrap,
                            move |win, cx| {
                                let _ = sm
                                    .update_settings(|s| s.editor.word_wrap = !s.editor.word_wrap);
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Show Whitespace",
                            "Display spaces and tabs",
                            settings.show_whitespace,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.editor.show_whitespace = !s.editor.show_whitespace
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                ],
                colors,
            ))
            .child(Self::render_section(
                "Indentation",
                vec![
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Tab Size",
                            "Number of spaces per indentation level",
                            settings.tab_size as f32,
                            2.0,
                            8.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ =
                                        sm.update_settings(|s| s.editor.tab_size = val as usize);
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Use Spaces",
                            "Insert spaces instead of tabs",
                            settings.use_spaces,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.editor.use_spaces = !s.editor.use_spaces
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                ],
                colors,
            ))
    }

    fn render_window_tab(
        settings_manager: Arc<SettingsManager>,
        settings: &WindowSettings,
        on_change: impl Fn(&mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let sm = settings_manager.clone();
        div()
            .flex()
            .flex_col()
            .gap_6()
            .p_6()
            .child(Self::render_section(
                "Window State",
                vec![
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Remember Window State",
                            "Restore size and position on startup",
                            settings.remember_window_state,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.window.remember_window_state = !s.window.remember_window_state
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Start Fullscreen",
                            "Open in fullscreen mode",
                            settings.start_fullscreen,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.window.start_fullscreen = !s.window.start_fullscreen
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                ],
                colors,
            ))
            .child(Self::render_section(
                "Titlebar (macOS)",
                vec![
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Transparent Titlebar",
                            "Use translucent window titlebar",
                            settings.transparent_titlebar,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.window.transparent_titlebar = !s.window.transparent_titlebar
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let sm = sm.clone();
                        let on_change = on_change.clone();
                        Self::render_toggle(
                            "Show Traffic Lights",
                            "Display window control buttons",
                            settings.show_traffic_lights,
                            move |win, cx| {
                                let _ = sm.update_settings(|s| {
                                    s.window.show_traffic_lights = !s.window.show_traffic_lights
                                });
                                on_change(win, cx);
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Traffic Light X Position",
                            "Horizontal position in pixels",
                            settings.traffic_light_x,
                            0.0,
                            100.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ = sm.update_settings(|s| s.window.traffic_light_x = val);
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                    {
                        let on_change = on_change.clone();
                        Self::render_slider(
                            "Traffic Light Y Position",
                            "Vertical position in pixels",
                            settings.traffic_light_y,
                            0.0,
                            100.0,
                            {
                                let sm = sm.clone();
                                move |val, win, cx| {
                                    let _ = sm.update_settings(|s| s.window.traffic_light_y = val);
                                    on_change(win, cx);
                                }
                            },
                            colors,
                        )
                        .into_any_element()
                    },
                ],
                colors,
            ))
            .child(Self::render_section(
                "UI Elements",
                vec![{
                    let sm = sm.clone();
                    let on_change = on_change.clone();
                    Self::render_toggle(
                        "Show Status Bar",
                        "Display status bar at bottom",
                        settings.show_status_bar,
                        move |win, cx| {
                            let _ = sm.update_settings(|s| {
                                s.window.show_status_bar = !s.window.show_status_bar
                            });
                            on_change(win, cx);
                        },
                        colors,
                    )
                    .into_any_element()
                }],
                colors,
            ))
    }

    fn render_section(
        title: &str,
        children: Vec<AnyElement>,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let title = title.to_string();
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(colors.text_primary)
                    .mb_2()
                    .child(title),
            )
            .children(children)
    }

    fn render_toggle(
        label: &str,
        description: &str,
        value: bool,
        on_toggle: impl Fn(&mut Window, &mut App) + 'static,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let label = label.to_string();
        let description = description.to_string();
        div()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .py_3()
            .bg(colors.bg_panel)
            .rounded_lg()
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(MouseButton::Left, move |_, win, app| (on_toggle)(win, app))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(colors.text_primary)
                            .child(label),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(colors.text_muted)
                            .child(description),
                    ),
            )
            .child(
                div()
                    .w(px(44.0))
                    .h(px(24.0))
                    .rounded(px(12.0))
                    .bg(if value {
                        colors.accent
                    } else {
                        colors.bg_element
                    })
                    .flex()
                    .items_center()
                    .px_1()
                    .child(if value {
                        div()
                            .w(px(18.0))
                            .h(px(18.0))
                            .rounded(px(9.0))
                            .bg(colors.text_inverse)
                            .ml(px(20.0))
                    } else {
                        div()
                            .w(px(18.0))
                            .h(px(18.0))
                            .rounded(px(9.0))
                            .bg(colors.text_primary)
                    }),
            )
    }

    fn render_slider(
        label: &str,
        description: &str,
        value: f32,
        min: f32,
        max: f32,
        on_change: impl Fn(f32, &mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        let label = label.to_string();
        let description = description.to_string();
        let display_value = if max <= 1.0 {
            format!("{:.2}", value)
        } else {
            format!("{:.0}", value)
        };

        let on_change_scroll = on_change.clone();

        div()
            .flex()
            .flex_col()
            .gap_2()
            .px_4()
            .py_3()
            .bg(colors.bg_panel)
            .rounded_lg()
            .on_scroll_wheel(move |event, win, app| {
                let delta = event.delta.pixel_delta(px(10.0)).y;
                let step = (max - min) / 20.0; // 5% step
                let change = if delta > px(0.0) { -step } else { step };
                let new_value = (value + change).clamp(min, max);
                (on_change_scroll)(new_value, win, app);
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(colors.text_primary)
                            .child(label),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors.accent)
                            .child(display_value),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(colors.text_muted)
                    .child(description),
            )
            .child(
                div()
                    .w_full()
                    .h(px(4.0))
                    .bg(colors.bg_element)
                    .rounded(px(2.0))
                    .mt_2()
                    .child(
                        div()
                            .h_full()
                            .bg(colors.accent)
                            .rounded(px(2.0))
                            .w(relative((value - min) / (max - min))),
                    ),
            )
    }

    fn render_theme_selector(
        current_theme: Theme,
        on_select: impl Fn(Theme, &mut Window, &mut App) + 'static + Clone,
        colors: &ThemeColors,
    ) -> impl IntoElement {
        div()
            .flex()
            .gap_3()
            .px_4()
            .py_3()
            .bg(colors.bg_panel)
            .rounded_lg()
            .children(Theme::all().into_iter().map(move |theme| {
                let is_active = theme == current_theme;
                let theme_name = theme.name().to_string();
                let on_select = on_select.clone();
                div()
                    .id(theme_name.clone())
                    .flex_1()
                    .py_3()
                    .text_center()
                    .text_sm()
                    .font_weight(if is_active {
                        FontWeight::SEMIBOLD
                    } else {
                        FontWeight::NORMAL
                    })
                    .text_color(if is_active {
                        colors.text_inverse
                    } else {
                        colors.text_secondary
                    })
                    .bg(if is_active {
                        rgb(0x0e639c)
                    } else {
                        rgb(0x3e3e3e)
                    })
                    .rounded_md()
                    .cursor(CursorStyle::PointingHand)
                    .hover(|style| {
                        if is_active {
                            style
                        } else {
                            style.bg(rgb(0x505050))
                        }
                    })
                    .child(theme_name)
                    .on_click(move |_, win, app| on_select(theme, win, app))
            }))
    }
}
