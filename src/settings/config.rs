use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub appearance: AppearanceSettings,
    pub kubernetes: KubernetesSettings,
    pub editor: EditorSettings,
    pub window: WindowSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            appearance: AppearanceSettings::default(),
            kubernetes: KubernetesSettings::default(),
            editor: EditorSettings::default(),
            window: WindowSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    /// Enable glassomorphic effects (blur, transparency)
    pub glassomorphism_enabled: bool,

    /// Blur intensity (0.0 - 100.0)
    pub blur_intensity: f32,

    /// Background opacity for glass panels (0.0 - 1.0)
    pub glass_opacity: f32,

    /// Theme selection
    pub theme: Theme,

    /// Font size for UI elements
    pub ui_font_size: f32,

    /// Font size for code/YAML
    pub code_font_size: f32,

    /// UI font family
    pub ui_font_family: String,

    /// Code font family
    pub code_font_family: String,

    /// Show sidebar icons
    pub show_sidebar_icons: bool,

    /// Sidebar width
    pub sidebar_width: f32,

    /// Enable animations
    pub animations_enabled: bool,

    /// Animation duration in milliseconds
    pub animation_duration: u32,

    /// Is sidebar collapsed
    pub sidebar_collapsed: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            glassomorphism_enabled: true,
            blur_intensity: 20.0,
            glass_opacity: 0.85,
            theme: Theme::Dark,
            ui_font_size: 13.0,
            code_font_size: 12.0,
            ui_font_family: "'SF Pro Display', 'Inter', 'Segoe UI', system-ui, sans-serif"
                .to_string(),
            code_font_family: "'JetBrains Mono', 'Fira Code', 'SF Mono', Menlo, monospace"
                .to_string(),
            show_sidebar_icons: true,
            sidebar_width: 220.0,
            animations_enabled: true,
            animation_duration: 200,
            sidebar_collapsed: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    HighContrast,
}

impl Theme {
    pub fn all() -> Vec<Theme> {
        vec![Theme::Dark, Theme::Light, Theme::HighContrast]
    }

    pub fn name(&self) -> &str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::HighContrast => "High Contrast",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesSettings {
    /// Auto-refresh interval in seconds (0 = disabled)
    pub auto_refresh_interval: u64,

    /// Default namespace to select on startup
    pub default_namespace: String,

    /// Show system namespaces (kube-system, etc.)
    pub show_system_namespaces: bool,

    /// Maximum number of items to display per resource type
    pub max_items_per_page: usize,

    /// Enable real-time watch mode
    pub watch_mode_enabled: bool,

    /// Kubeconfig path (empty = default)
    pub kubeconfig_path: String,

    /// Context to use (empty = current context)
    pub context: String,

    /// Show resource metrics (CPU, memory)
    pub show_metrics: bool,

    /// Enable event notifications
    pub enable_notifications: bool,
}

impl Default for KubernetesSettings {
    fn default() -> Self {
        Self {
            auto_refresh_interval: 30,
            default_namespace: "default".to_string(),
            show_system_namespaces: true,
            max_items_per_page: 100,
            watch_mode_enabled: true,
            kubeconfig_path: String::new(),
            context: String::new(),
            show_metrics: true,
            enable_notifications: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    /// Enable YAML syntax highlighting
    pub syntax_highlighting: bool,

    /// Show line numbers in YAML editor
    pub show_line_numbers: bool,

    /// Tab size for YAML indentation
    pub tab_size: usize,

    /// Use spaces instead of tabs
    pub use_spaces: bool,

    /// Enable YAML validation
    pub yaml_validation: bool,

    /// Word wrap in editor
    pub word_wrap: bool,

    /// Show whitespace characters
    pub show_whitespace: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            syntax_highlighting: true,
            show_line_numbers: true,
            tab_size: 2,
            use_spaces: true,
            yaml_validation: true,
            word_wrap: false,
            show_whitespace: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    /// Remember window size and position
    pub remember_window_state: bool,

    /// Start in fullscreen
    pub start_fullscreen: bool,

    /// Show traffic light buttons (macOS)
    pub show_traffic_lights: bool,

    /// Traffic light position X
    pub traffic_light_x: f32,

    /// Traffic light position Y
    pub traffic_light_y: f32,

    /// Transparent titlebar
    pub transparent_titlebar: bool,

    /// Show status bar
    pub show_status_bar: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            remember_window_state: true,
            start_fullscreen: false,
            show_traffic_lights: true,
            traffic_light_x: 20.0,
            traffic_light_y: 24.0,
            transparent_titlebar: true,
            show_status_bar: true,
        }
    }
}
