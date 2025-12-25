#![allow(dead_code)]

use gpui::*;

/// Glassomorphic styling configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassStyle {
    pub enabled: bool,
    pub blur_intensity: f32,
    pub opacity: f32,
    pub border_opacity: f32,
}

impl Default for GlassStyle {
    fn default() -> Self {
        Self {
            enabled: true,
            blur_intensity: 30.0,
            opacity: 0.70,
            border_opacity: 0.15,
        }
    }
}

impl GlassStyle {
    pub fn new(enabled: bool, blur_intensity: f32, opacity: f32) -> Self {
        Self {
            enabled,
            blur_intensity,
            opacity,
            border_opacity: 0.3,
        }
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            blur_intensity: 0.0,
            opacity: 1.0,
            border_opacity: 1.0,
        }
    }

    /// Apply glassomorphic styling to a panel/card element
    pub fn apply_to_panel(&self, base_color: Hsla) -> Hsla {
        if !self.enabled {
            return base_color;
        }

        // Adjust the alpha channel based on opacity setting
        Hsla {
            h: base_color.h,
            s: base_color.s,
            l: base_color.l,
            a: base_color.a * self.opacity,
        }
    }

    /// Apply glassomorphic styling to sidebar
    pub fn apply_to_sidebar(&self, base_color: Hsla) -> Hsla {
        if !self.enabled {
            return base_color;
        }

        // Slightly more transparent for sidebar
        Hsla {
            h: base_color.h,
            s: base_color.s,
            l: base_color.l,
            a: base_color.a * (self.opacity * 0.95),
        }
    }

    /// Apply glassomorphic border styling
    pub fn apply_to_border(&self, base_color: Hsla) -> Hsla {
        if !self.enabled {
            return base_color;
        }

        Hsla {
            h: base_color.h,
            s: base_color.s,
            l: base_color.l + 0.1, // Slightly lighter
            a: base_color.a * self.border_opacity,
        }
    }

    /// Get backdrop blur value (for future GPUI support)
    pub fn backdrop_blur(&self) -> f32 {
        if !self.enabled {
            0.0
        } else {
            self.blur_intensity
        }
    }
}

/// Extension trait for applying glass effects to div elements
pub trait GlassExt: Sized {
    /// Apply glass panel styling
    fn glass_panel(self, glass_style: GlassStyle) -> Self;

    /// Apply glass sidebar styling
    fn glass_sidebar(self, glass_style: GlassStyle) -> Self;

    /// Apply glass card styling
    fn glass_card(self, glass_style: GlassStyle) -> Self;

    /// Apply glass titlebar styling
    fn glass_titlebar(self, glass_style: GlassStyle) -> Self;
}

impl GlassExt for Div {
    fn glass_panel(self, glass_style: GlassStyle) -> Self {
        if !glass_style.enabled {
            return self.bg(rgb(0x2d2d30));
        }

        // Create semi-transparent background
        let bg_color = glass_style.apply_to_panel(hsla(0.0, 0.0, 0.18, 1.0));
        let border_color = glass_style.apply_to_border(hsla(0.0, 0.0, 0.25, 1.0));

        self.bg(bg_color).border_1().border_color(border_color)
        // Note: GPUI may not support backdrop-filter yet, but we prepare for it
        // .backdrop_blur(px(glass_style.backdrop_blur()))
    }

    fn glass_sidebar(self, glass_style: GlassStyle) -> Self {
        if !glass_style.enabled {
            return self.bg(rgb(0x252526));
        }

        let bg_color = glass_style.apply_to_sidebar(hsla(0.0, 0.0, 0.15, 1.0));
        let border_color = glass_style.apply_to_border(hsla(0.0, 0.0, 0.24, 1.0));

        self.bg(bg_color).border_r_1().border_color(border_color)
    }

    fn glass_card(self, glass_style: GlassStyle) -> Self {
        if !glass_style.enabled {
            return self
                .bg(rgb(0x1e1e1e))
                .border_1()
                .border_color(rgb(0x3e3e3e));
        }

        let bg_color = glass_style.apply_to_panel(hsla(0.0, 0.0, 0.12, 1.0));
        let border_color = glass_style.apply_to_border(hsla(0.0, 0.0, 0.20, 1.0));

        self.bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded_lg()
    }

    fn glass_titlebar(self, glass_style: GlassStyle) -> Self {
        if !glass_style.enabled {
            return self
                .bg(rgb(0x2d2d30))
                .border_b_1()
                .border_color(rgb(0x3e3e3e));
        }

        // Slightly clearer than panels for better readability of window controls
        let bg_color = glass_style.apply_to_panel(hsla(0.0, 0.0, 0.15, 0.9));
        let border_color = glass_style.apply_to_border(hsla(0.0, 0.0, 0.20, 1.0));

        self.bg(bg_color).border_b_1().border_color(border_color)
    }
}

/// Helper function to create glass-styled divider
pub fn glass_divider(glass_style: GlassStyle) -> impl IntoElement {
    let color = if glass_style.enabled {
        glass_style.apply_to_border(hsla(0.0, 0.0, 0.25, 1.0))
    } else {
        rgb(0x3e3e3e).into()
    };

    div().h(px(1.0)).w_full().bg(color)
}
