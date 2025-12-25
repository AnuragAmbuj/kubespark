# Quick Start: KubeSpark Settings

## Opening Settings

**Keyboard Shortcut:**
- macOS: `Cmd + ,`
- Linux/Windows: `Ctrl + ,`

**First Time:**
Settings will be created at `~/.config/kubespark/settings.json` with sensible defaults.

## Quick Configuration

### Enable/Disable Glassomorphic Effects

1. Open Settings (`Cmd+,`)
2. Click **Appearance** tab (should be default)
3. Toggle "Enable Glassomorphism"
4. Adjust sliders:
   - **Blur Intensity**: 0-100 (default: 20)
   - **Glass Opacity**: 0.0-1.0 (default: 0.85)

### Change Theme

1. Settings → **Appearance** tab
2. Click theme button: Dark / Light / High Contrast
3. Close settings to apply

### Configure Kubernetes Auto-Refresh

1. Settings → **Kubernetes** tab
2. Adjust "Interval (seconds)" slider (0 = disabled, default: 30)
3. Toggle "Watch Mode" for real-time updates via K8s watch API

### Customize Fonts

1. Settings → **Appearance** tab
2. Scroll to **Typography** section
3. Adjust sliders:
   - UI Font Size: 10-20px (default: 13)
   - Code Font Size: 10-20px (default: 12)

### Editor Settings

1. Settings → **Editor** tab
2. Toggle features:
   - Syntax Highlighting
   - Line Numbers
   - YAML Validation
   - Word Wrap
3. Set indentation: Tab Size + "Use Spaces" toggle

## Settings File Location

**Path:** `~/.config/kubespark/settings.json`

**Manual Edit:**
```bash
# Open in your editor
code ~/.config/kubespark/settings.json
```

**Backup:**
```bash
# Create backup
cp ~/.config/kubespark/settings.json ~/.config/kubespark/settings.backup.json
```

**Reset to Defaults:**
- Option 1: Click "Reset to Defaults" in settings header
- Option 2: Delete the file and restart: `rm ~/.config/kubespark/settings.json`

## Example Configuration

```json
{
  "appearance": {
    "glassomorphism_enabled": true,
    "blur_intensity": 30.0,
    "glass_opacity": 0.9,
    "theme": "Dark",
    "ui_font_size": 14.0,
    "code_font_size": 13.0
  },
  "kubernetes": {
    "auto_refresh_interval": 60,
    "watch_mode_enabled": true,
    "show_system_namespaces": true
  }
}
```

## Troubleshooting

**Settings won't save?**
```bash
# Check permissions
ls -la ~/.config/kubespark/
# Should be writable by your user
```

**Want to start fresh?**
```bash
# Delete settings file
rm ~/.config/kubespark/settings.json
# Restart application - defaults will be recreated
```

**Settings not applying?**
- Close and reopen settings panel
- Some settings may require app restart (window settings)

## Tips

1. **Glassomorphism Performance**: If UI feels slow, reduce blur intensity or disable glassomorphism
2. **Auto-Refresh**: Set to 0 to disable and reduce API calls
3. **Watch Mode**: More efficient than polling, but requires cluster support
4. **Font Sizes**: Adjust for accessibility or screen size preferences
5. **Animations**: Disable for a snappier feel or accessibility needs

## See Also

- Full documentation: [SETTINGS.md](SETTINGS.md)
- Implementation details: [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)
