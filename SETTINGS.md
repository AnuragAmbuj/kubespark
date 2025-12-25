# KubeSpark Settings System

## Overview

KubeSpark features a comprehensive settings system with persistent configuration and glassomorphic UI effects that can be customized to match your preferences.

## Features

### Settings Categories

#### 1. Appearance Settings
- **Glassomorphic Effects**: Toggle blur and transparency effects throughout the UI
  - Enable/disable glassomorphism
  - Adjustable blur intensity (0-100)
  - Configurable glass opacity (0-1.0)
- **Theme Selection**: Choose from Dark, Light, or High Contrast themes
- **Typography**: Customize font sizes for UI and code elements
- **Sidebar Configuration**: Toggle icons and adjust width
- **Animations**: Control transitions and their duration

#### 2. Kubernetes Settings
- **Auto Refresh**: Configure automatic resource list updates
  - Adjustable interval (0-300 seconds, 0 = disabled)
  - Real-time watch mode using Kubernetes watch API
- **Display Options**:
  - Show/hide system namespaces (kube-system, etc.)
  - Max items per page (10-500)
  - Resource metrics display (CPU, memory)
- **Notifications**: Enable/disable Kubernetes event alerts

#### 3. Editor Settings
- **YAML Features**:
  - Syntax highlighting
  - Line numbers
  - YAML validation
  - Word wrap
  - Whitespace visualization
- **Indentation**:
  - Tab size (2-8 spaces)
  - Use spaces vs tabs toggle

#### 4. Window Settings
- **Window State**:
  - Remember size and position on restart
  - Start in fullscreen option
- **macOS Titlebar**:
  - Transparent titlebar option
  - Traffic light button visibility
  - Adjustable traffic light position
- **UI Elements**: Show/hide status bar

## Usage

### Accessing Settings

- **Keyboard Shortcut**: `Cmd+,` (macOS) / `Ctrl+,` (Linux/Windows)
- **Action**: `ToggleSettings`

### Settings Persistence

Settings are automatically saved to:
- **Location**: `~/.config/kubespark/settings.json`
- **Format**: JSON
- **Auto-save**: Changes are saved immediately

### Resetting to Defaults

Click the "Reset to Defaults" button in the settings panel header to restore all settings to their default values.

## Glassomorphic Effects

The glassomorphic effect system adds a modern, translucent aesthetic to the UI:

- **Enabled by default** for a polished look
- **Configurable intensity** to match your preference
- **Performance-friendly** using GPUI's GPU acceleration
- **Applied to**:
  - Sidebar panels
  - Main content areas
  - Detail views
  - Status bar

### Glass Effect Controls

```rust
GlassStyle {
    enabled: bool,          // Toggle effects on/off
    blur_intensity: f32,    // 0.0 - 100.0
    opacity: f32,           // 0.0 - 1.0
    border_opacity: f32,    // 0.0 - 1.0 (auto-calculated)
}
```

## Architecture

### Components

1. **`settings/config.rs`**: Defines all settings structures and defaults
2. **`settings/manager.rs`**: Handles persistence and state management
3. **`settings/ui.rs`**: Renders the settings UI with tabs
4. **`ui/glass.rs`**: Implements glassomorphic styling system

### Settings Manager API

```rust
// Initialize
let settings_manager = Arc::new(SettingsManager::new());

// Get current settings
let settings = settings_manager.get_settings();

// Update settings
settings_manager.update_settings(|s| {
    s.appearance.glassomorphism_enabled = true;
    s.appearance.blur_intensity = 30.0;
})?;

// Reset to defaults
settings_manager.reset_to_defaults()?;
```

### Glass Style Extension Trait

```rust
use crate::ui::glass::{GlassExt, GlassStyle};

// Apply to any div
div()
    .glass_panel(glass_style)       // For main content panels
    .glass_sidebar(glass_style)     // For sidebar panels
    .glass_card(glass_style)        // For card-style elements
```

## Default Configuration

```json
{
  "appearance": {
    "glassomorphism_enabled": true,
    "blur_intensity": 20.0,
    "glass_opacity": 0.85,
    "theme": "Dark",
    "ui_font_size": 13.0,
    "code_font_size": 12.0,
    "ui_font_family": "'SF Pro Display', 'Inter', 'Segoe UI', system-ui, sans-serif",
    "code_font_family": "'JetBrains Mono', 'Fira Code', 'SF Mono', Menlo, monospace",
    "show_sidebar_icons": true,
    "sidebar_width": 220.0,
    "animations_enabled": true,
    "animation_duration": 200
  },
  "kubernetes": {
    "auto_refresh_interval": 30,
    "default_namespace": "default",
    "show_system_namespaces": true,
    "max_items_per_page": 100,
    "watch_mode_enabled": true,
    "kubeconfig_path": "",
    "context": "",
    "show_metrics": true,
    "enable_notifications": true
  },
  "editor": {
    "syntax_highlighting": true,
    "show_line_numbers": true,
    "tab_size": 2,
    "use_spaces": true,
    "yaml_validation": true,
    "word_wrap": false,
    "show_whitespace": false
  },
  "window": {
    "remember_window_state": true,
    "start_fullscreen": false,
    "show_traffic_lights": true,
    "traffic_light_x": 20.0,
    "traffic_light_y": 24.0,
    "transparent_titlebar": true,
    "show_status_bar": true
  }
}
```

## Future Enhancements

- [ ] Settings search functionality
- [ ] Import/export settings
- [ ] Multiple theme presets
- [ ] Per-namespace settings
- [ ] Keyboard shortcut customization
- [ ] Plugin/extension settings
- [ ] Dark/light theme auto-switching based on system
- [ ] Settings sync across machines

## Implementation Status

✅ Settings data structures
✅ Settings manager with persistence
✅ Settings UI with tabbed interface
✅ Glassomorphic effect system
✅ Integration with main app
✅ Keyboard shortcut (Cmd+,)
⏳ UI compilation fixes (minor type mismatches remaining)
⏳ Real-time settings updates without restart

## Contributing

When adding new settings:

1. Add the setting to the appropriate struct in `settings/config.rs`
2. Update the default implementation
3. Add UI controls in `settings/ui.rs` in the relevant tab
4. Document the setting in this file
5. Ensure settings are properly serialized/deserialized

## Troubleshooting

**Settings not persisting?**
- Check that `~/.config/kubespark/` directory exists and is writable
- Verify settings.json is valid JSON
- Try deleting settings.json to regenerate defaults

**Glass effects not visible?**
- Ensure `glassomorphism_enabled` is `true`
- Increase `blur_intensity` and adjust `glass_opacity`
- Check GPU acceleration is available

**Settings panel won't open?**
- Verify Cmd+, keybinding is not conflicting
- Check console for errors
- Restart the application
