use crate::settings::Theme;
use gpui::{rgb, Rgba};

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub bg_app: Rgba,
    pub bg_sidebar: Rgba,
    pub bg_panel: Rgba,
    pub bg_element: Rgba,
    pub bg_element_hover: Rgba,
    pub bg_element_active: Rgba,
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
    pub text_muted: Rgba,
    pub text_inverse: Rgba,
    pub border: Rgba,
    pub accent: Rgba,
    pub accent_hover: Rgba,
    pub status_ok: Rgba,
    pub status_error: Rgba,
    pub status_warning: Rgba,
    pub status_info: Rgba,
    pub ghost_element_hover: Rgba,
    pub traffic_light_close: Rgba,
    pub traffic_light_minimize: Rgba,
    pub traffic_light_maximize: Rgba,
}

pub trait ThemeExt {
    fn colors(&self) -> ThemeColors;
}

impl ThemeExt for Theme {
    fn colors(&self) -> ThemeColors {
        match self {
            Theme::Dark => ThemeColors {
                bg_app: rgb(0x1e1e1e),
                bg_sidebar: rgb(0x252526),
                bg_panel: rgb(0x2d2d30),
                bg_element: rgb(0x3e3e3e),
                bg_element_hover: rgb(0x505050),
                bg_element_active: rgb(0x0e639c),
                text_primary: rgb(0xe0e0e0),
                text_secondary: rgb(0xcccccc),
                text_muted: rgb(0x999999),
                text_inverse: rgb(0xffffff),
                border: rgb(0x3e3e3e),
                accent: rgb(0x0e639c),
                accent_hover: rgb(0x1177bb),
                status_ok: rgb(0x379d5c),
                status_error: rgb(0xc42b1c),
                status_warning: rgb(0xcca700),
                status_info: rgb(0x4ec9b0),
                ghost_element_hover: rgb(0x3a3d41),
                traffic_light_close: rgb(0xc42b1c),
                traffic_light_minimize: rgb(0xcccccc),
                traffic_light_maximize: rgb(0xcccccc),
            },
            Theme::Light => ThemeColors {
                bg_app: rgb(0xffffff),
                bg_sidebar: rgb(0xf3f3f3),
                bg_panel: rgb(0xf8f8f8),
                bg_element: rgb(0xe5e5e5),
                bg_element_hover: rgb(0xd0d0d0),
                bg_element_active: rgb(0x0078d4),
                text_primary: rgb(0x202020),
                text_secondary: rgb(0x444444),
                text_muted: rgb(0x666666),
                text_inverse: rgb(0xffffff),
                border: rgb(0xe5e5e5),
                accent: rgb(0x0078d4),
                accent_hover: rgb(0x006cc1),
                status_ok: rgb(0x107c10),
                status_error: rgb(0xd13438),
                status_warning: rgb(0x795e00),
                status_info: rgb(0x005a9e),
                ghost_element_hover: rgb(0xe8e8e8),
                traffic_light_close: rgb(0xd13438),
                traffic_light_minimize: rgb(0x666666),
                traffic_light_maximize: rgb(0x666666),
            },
            Theme::HighContrast => ThemeColors {
                bg_app: rgb(0x000000),
                bg_sidebar: rgb(0x000000),
                bg_panel: rgb(0x000000),
                bg_element: rgb(0x000000),
                bg_element_hover: rgb(0x1a1a1a),
                bg_element_active: rgb(0xffffff), // Active state
                text_primary: rgb(0xffffff),
                text_secondary: rgb(0xffffff),
                text_muted: rgb(0xffffff),
                text_inverse: rgb(0x000000),
                border: rgb(0xffffff),
                accent: rgb(0xffff00), // Yellow accent
                accent_hover: rgb(0xffff00),
                status_ok: rgb(0x00ff00),
                status_error: rgb(0xff0000),
                status_warning: rgb(0xffff00),
                status_info: rgb(0x00ffff),
                ghost_element_hover: rgb(0x1a1a1a),
                traffic_light_close: rgb(0xff0000),
                traffic_light_minimize: rgb(0xffffff),
                traffic_light_maximize: rgb(0xffffff),
            },
        }
    }
}
