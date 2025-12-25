# KubeSpark Visual Preview

## Window Layout

```
╔═══════════════════════════════════════════════════════════════════╗
║  ●  ■  [K8]  KubeSpark                                            ║ ← Custom Titlebar (40px)
╠═══════════╦═══════════════════════════════╦═══════════════════════╣
║           ║                               ║                       ║
║ RESOURCES ║  Pods                    🔄   ║  Details           ✕ ║
║           ║  ───────────────────────────  ║                       ║
║ CLUSTER   ║  NAME         NS      STATUS  ║  Basic Information    ║
║ Namespaces║  ──────────────────────────── ║  ────────────────────  ║
║ Nodes     ║                               ║  Name: nginx-pod      ║
║           ║  nginx-pod    default Running ║  Kind: Pod            ║
║ WORKLOADS ║  redis-pod    default Pending ║  Namespace: default   ║
║ Pods      ║  api-deploy   prod    Running ║  Status: Running      ║
║ Deploy... ║                               ║  Age: 2h              ║
║ StatefulS ║  Select a resource type       ║                       ║
║ DaemonSe..║  from the sidebar             ║  YAML                 ║
║ ReplicaS..║                               ║  ────────────────────  ║
║ Jobs      ║                               ║  apiVersion: v1       ║
║ CronJobs  ║                               ║  kind: Pod            ║
║           ║                               ║  metadata:            ║
║ NETWORK   ║                               ║    name: nginx-pod    ║
║ Services  ║                               ║    namespace: default ║
║ Ingresses ║                               ║  spec:                ║
║           ║                               ║    containers:        ║
║ CONFIG    ║                               ║    - name: nginx      ║
║ ConfigMap ║                               ║      image: nginx:1.2 ║
║ Secrets   ║                               ║                       ║
║           ║                               ║                       ║
╠═══════════╩═══════════════════════════════╩═══════════════════════╣
║  ● Connected                               KubeSpark v0.1.0       ║ ← Status Bar (28px)
╚═══════════════════════════════════════════════════════════════════╝
```

## Color Scheme (Dark Theme)

### Primary Colors
- Background: `#1e1e1e` ████████
- Surface: `#252526` ██████████
- Titlebar: `#2d2d30` ████████████
- Borders: `#3e3e3e` ██████████████

### Accent Colors
- Primary Blue: `#007acc` ████████████████
- Close Red: `#ff5f56` ██████████████
- Maximize Green: `#28c840` ████████████

### Status Colors
- Running/Success: `#4ec9b0` ████████ (Teal)
- Pending/Warning: `#dcdcaa` ████████ (Yellow)
- Failed/Error: `#f48771` ████████ (Red)
- Completed: `#89d185` ████████ (Green)

### Text Colors
- Primary: `#ffffff` ████████ (White)
- Secondary: `#cccccc` ████████ (Light Grey)
- Tertiary: `#888888` ████████ (Medium Grey)

## Typography Examples

### UI Text (Inter/Segoe UI/San Francisco)
```
RESOURCES          ← 12px, Bold, #cccccc
CLUSTER            ← 11px, Semibold, #888888
Pods               ← 13px, Regular, #cccccc → #ffffff (hover)
KubeSpark          ← 13px, Semibold, #ffffff
```

### Code/YAML (JetBrains Mono/Fira Code)
```yaml
apiVersion: v1      ← 11px, Monospace, #cccccc
kind: Pod           ← Line height: 1.6 for readability
metadata:
  name: nginx-pod
```

## Interactive Elements

### Window Controls
```
 ●   ■
───  ───
Red  Green
Close Maximize

Hover effects:
● → Brightens from #ff5f56 to #ff3b30
■ → Brightens from #28c840 to #20a030
```

### Sidebar Items
```
Pods           ← Normal state: #cccccc
Pods           ← Hover state: #ffffff + background #2a2d2e
Pods           ← Active state: background #094771
```

### Status Badges
```
╭──────────╮  ╭──────────╮  ╭──────────╮
│ Running  │  │ Pending  │  │ Failed   │
╰──────────╯  ╰──────────╯  ╰──────────╯
 Teal bg       Yellow bg      Red bg
 #1a3a35       #3a3a2a        #3a2a2a
```

## Layout Dimensions

- **Window**: 1400x900px (default), minimum 800x600px
- **Titlebar**: Full width × 40px height
- **Sidebar**: 220px width × full height
- **Detail Panel**: 400px width × full height (when visible)
- **Status Bar**: Full width × 28px height
- **Table Row**: Full width × 36px height
- **Table Header**: Full width × 32px height

## Spacing System

Using an 8px base grid:
- `gap_1` = 4px (0.5 units)
- `gap_2` = 8px (1 unit)
- `gap_3` = 12px (1.5 units)
- `gap_4` = 16px (2 units)
- `p_1` = 4px padding
- `p_2` = 8px padding
- `p_3` = 12px padding
- `p_4` = 16px padding

## Border Radii

- Buttons: `rounded_md` (6px)
- Window controls: `rounded_md` (6px)
- Code blocks: `rounded_md` (6px)
- Status badges: `rounded_sm` (4px)
- Maximize icon: `rounded_sm` (4px)

## Shadows & Effects

- Hover states: Slight background color change
- Active states: Distinctive background color (#094771)
- Borders: 1px solid with appropriate color
- No drop shadows (flat design)

## Responsive Behavior

- Sidebar: Fixed 220px width
- Detail panel: Fixed 400px width, can be toggled
- Main content: Flexible, fills remaining space
- Window controls: Always visible at fixed position
- Status bar: Always at bottom, full width
