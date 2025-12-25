# Project Structure

The project is organized as a standard Rust binary crate with a modular structure separating UI, logic, and configuration.

## File Tree

```
kubespark/
├── src/
│   ├── main.rs                 # Entry point: Routine setup, Window configuration
│   ├── app.rs                  # Core Application Logic: State, Render implementation
│   │
│   ├── kubernetes/             # Kubernetes Domain Layer
│   │   ├── mod.rs              # Module exports and Enum definitions (ResourceKind)
│   │   ├── client.rs           # KubeClient implementation (API calls)
│   │   └── resources.rs        # Data structs for Pods, Nodes etc.
│   │
│   ├── settings/               # Configuration & Persistence
│   │   ├── mod.rs
│   │   ├── config.rs           # Serializable Settings struct (Theme, etc.)
│   │   └── manager.rs          # Settings Manager (Load/Save logic)
│   │
│   └── ui/                     # UI Component Library
│       ├── mod.rs
│       ├── glass.rs            # Styling utilities (Glassmorphism, colors)
│       ├── sidebar.rs          # Left navigation, Context switching UI
│       ├── status_bar.rs       # Bottom status bar component
│       ├── theme.rs            # Theme definitions
│       └── ... (other views)
│
├── assets/                     # Static assets
│   └── screenshots/            # Documentation images
│
├── .docs/                      # Documentation folder (this directory)
├── Cargo.toml                  # Dependencies
└── README.md                   # GitHub landing page
```

## Module Descriptions

### `src/main.rs`
The bootstrap file. It:
1. Initializes `env_logger`.
2. Sets up the main `tokio` runtime to enable async operations globally.
3. Configures the GPUI `Application` and opens the main window with specific bounds and titlebar settings.

### `src/app.rs`
The heart of the application. It defines the `KubeSparkApp` struct which stores:
- `kube_client`: Shared reference to the Kubernetes client.
- `resources`: The list of items currently being displayed.
- `current_context`: The specific cluster context we are connected to.
- `is_sidebar_collapsed`: UI state.
It also implements `impl Render for KubeSparkApp`, which describes the root UI layout (Sidebar + Main Content).

### `src/kubernetes/`
- **`client.rs`**: Abstracts the `kube` crate. It provides easy-to-use async methods like `list_contexts()`, `connect()`, `list_pods()`. It handles the complexity of `Kubeconfig` parsing.
- **`resources.rs`**: Defines struct representations of K8s resources that are easy for the UI to consume.

### `src/ui/`
Standardized UI components to ensure consistency.
- **`sidebar.rs`**: Not just a list of links, but also handles the "Context Switcher" dropdown.
- **`glass.rs`**: Contains helper functions/traits for styling. E.g., `glass_blur()`, `glass_text()`.
- **`status_bar.rs`**: Displays the active connection status and version info.
