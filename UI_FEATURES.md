# KubeSpark UI Features

## macOS-Style Window Design

The application features a native macOS appearance with:

### Native Titlebar
- **Transparent titlebar** that integrates with the system
- **Traffic light buttons** (red, yellow, green) positioned at `(20px, 24px)`
- **Native window controls** for close, minimize, and maximize
- **System appearance** that matches macOS Sonoma and later

### Integrated Title Area (52px height)
- Clean, centered title display
- Seamless integration with content below
- Professional macOS-style appearance

## Typography

### Main Font Stack
```css
"'SF Pro Display', 'Inter', 'Segoe UI', system-ui, -apple-system, sans-serif"
```
This provides a native macOS feel:
- **SF Pro Display**: Apple's system font (primary)
- **Inter**: Clean fallback for other platforms
- **Segoe UI**: Windows native font
- **system-ui**: Platform default
- **-apple-system**: macOS system font

### Monospace Font (YAML/Code Display)
```css
"'JetBrains Mono', 'Fira Code', 'SF Mono', Menlo, Monaco, 'Courier New', monospace"
```
Optimized for code readability with:
- **JetBrains Mono**: Excellent for code with ligature support
- **Fira Code**: Popular developer font
- **SF Mono**: macOS developer font (preferred on Mac)
- **Menlo/Monaco**: macOS alternatives
- **monospace**: Generic fallback

### Line Height
- **Code/YAML**: 1.6 for comfortable reading
- **UI Text**: Default system line heights

## Design System

### Color Palette (Dark Theme)
- **Background**: `#1e1e1e` (Main dark grey)
- **Surface**: `#252526` (Sidebar background)
- **Title Area**: `#2d2d30` (Titlebar area)
- **Borders**: `#3e3e3e` (Subtle borders)
- **Primary**: `#007acc` (Kubernetes blue)
- **Text Primary**: `#ffffff` (White)
- **Text Secondary**: `#cccccc` (Light grey)
- **Text Tertiary**: `#888888` (Medium grey)
- **Category Headers**: `#666666` (Darker grey)

### Hover States
- **Sidebar items**: Background `#37373d` + text `#ffffff`
- **Buttons**: Lighter shade of background color
- **Interactive elements**: Cursor changes to pointer

### Status Colors
- **Success/Running**: `#4ec9b0` (Teal)
- **Warning/Pending**: `#dcdcaa` (Yellow)
- **Error/Failed**: `#f48771` (Red)
- **Info/Completed**: `#89d185` (Green)

## Layout

```
┌─────────────────────────────────────────────────────┐
│ ●●● [System titlebar - transparent]                 │
│           KubeSpark - Kubernetes Client             │ ← Title Area (52px)
├───────────┬─────────────────────┬───────────────────┤
│           │                     │                   │
│ RESOURCES │  Pods          🔄   │  Details       ✕ │
│           │  ─────────────────  │                   │
│ CLUSTER   │  NAME    NS  STATUS │  Basic Info       │
│ Namespaces│  ───────────────── │  ───────────────  │
│ Nodes     │                     │  Name: nginx-pod  │
│           │  nginx-pod  default │  Kind: Pod        │
│ WORKLOADS │            Running  │  Namespace: ...   │
│ ● Pods    │                     │                   │
│ Deploy... │                     │  YAML             │
│ StatefulS │                     │  ───────────────  │
│ DaemonSe..│                     │  apiVersion: v1   │
│           │                     │  kind: Pod        │
│ NETWORK   │                     │  metadata:        │
│ Services  │                     │    name: nginx... │
│ Ingresses │                     │                   │
│           │                     │                   │
├───────────┴─────────────────────┴───────────────────┤
│ ● Connected                  KubeSpark v0.1.0       │ ← Status Bar (28px)
└─────────────────────────────────────────────────────┘
```

## Component Highlights

### Title Area (52px height)
- Centered title text with medium font weight
- Integrates seamlessly with native titlebar
- Subtle border at bottom for separation
- macOS-style appearance and feel

### Sidebar (220px width)
- **Spacing**: 
  - Top padding: 12px
  - Section gaps: 12px
  - Item padding: 4px vertical, 12px horizontal
  - Rounded corners on hover: 6px
- **Typography**:
  - Category headers: 11px, semibold, darker grey
  - Resource items: 13px, regular, light grey
- **Hover effects**: Rounded background with smooth transition

### Resource List
- Table-based layout with fixed headers
- Color-coded status indicators with subtle backgrounds
- Clean, readable typography
- Consistent spacing throughout

### Detail Panel (400px width)
- Collapsible right panel
- Information sections with clear hierarchy
- YAML viewer with monospace font and proper line height
- Clean borders and spacing

### Status Bar (28px height)
- Connection status with colored indicators
- Version information
- Consistent with overall design theme
- Blue accent background

## macOS Integration Features

### Native Window Behavior
- **Traffic lights**: Standard macOS window controls
- **Transparent titlebar**: Seamless integration with content
- **Drag region**: Title area is draggable for window movement
- **Minimize/maximize**: Standard macOS behavior

### Visual Consistency
- **System font**: SF Pro Display for native feel
- **Rounded corners**: Consistent with macOS design language
- **Hover states**: Subtle and refined
- **Color scheme**: Professional dark theme matching macOS apps

### Platform Responsiveness
- Window resizable with minimum 800×600px
- Proper sidebar and panel widths
- Flexible content area that adapts to window size

## Spacing System

Using a 4px base grid:
- `gap_px` = 1px (dividers)
- `gap_1` = 4px
- `gap_2` = 8px  
- `gap_3` = 12px (section spacing)
- `gap_4` = 16px
- `p_1` / `py_1` = 4px
- `p_2` / `px_2` = 8px
- `p_3` / `px_3` = 12px
- `pt_3` = 12px (top padding)

## Border Radii

- Sidebar items: `rounded_md` (6px)
- Buttons: `rounded_md` (6px)
- Code blocks: `rounded_md` (6px)
- Status badges: `rounded_sm` (4px)

## Accessibility

- **Sufficient contrast ratios** for all text
- **Clear hover states** for interactive elements
- **Cursor feedback** (pointer on clickable items)
- **Keyboard navigation** support (Cmd+Q to quit)
- **Native controls** for familiar interaction patterns
