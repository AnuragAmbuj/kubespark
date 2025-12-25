# Code Walkthrough

This guide provides a deep dive into the critical paths of the KubeSpark codebase.

## 1. Application Initialization (`src/app.rs`)

The `KubeSparkApp::new` method is the constructor. It does more than just create a struct:

```rust
pub fn new(cx: &mut Context<Self>) -> Self {
    // 1. Initialize Clients
    let kube_client = Arc::new(KubeClient::new());
    
    // 2. Setup Async Channels for Init Data
    let (tx, rx) = oneshot::channel();
    let client_clone = kube_client.clone();
    
    // 3. Spawn a Background Task (on Tokio Runtime)
    // The 'tokio::spawn' here runs on the global runtime created in main.rs
    tokio::spawn(async move {
        // Heavy lifting: file I/O, parsing YAML, network discovery
        let contexts = KubeClient::list_contexts().await.unwrap_or_default();
        let current = KubeClient::get_current_context().await.unwrap_or_default();
        
        // Send data back
        let _ = tx.send((contexts, current));
    });

    // 4. Spawn a UI Listener
    // This runs on the UI thread but waits for the async "rx" safely
    cx.spawn(move |this, mut cx| async move {
        if let Ok((contexts, current)) = rx.await {
            // Update UI state inside the 'update' closure
            this.update(&mut cx, |app, cx| {
                app.available_contexts = contexts;
                app.current_context = current;
                cx.notify(); // Triggers a re-render
            });
        }
    }).detach();

    // 5. Return Initial State
    Self { ... }
}
```

**Key Takeaway**: We leverage `tokio::spawn` for the actual work and `cx.spawn` to handle the *result* and update the UI.

## 2. Context Switching (`src/app.rs`)

When a user clicks a context in the Sidebar dropdown:

```rust
pub fn switch_context(&mut self, ctx_name: String, cx: &mut Context<Self>) {
    // 1. Optimistic UI Update
    self.show_context_menu = false;
    self.current_context = ctx_name.clone();
    cx.notify(); // Re-renders the sidebar immediately with new text
    
    // 2. Async Connection
    let client = self.kube_client.clone();
    
    cx.spawn(move |this, mut cx| async move {
        // Blocks on background thread, not UI
        let _ = client.connect_with_context(&ctx_name).await;
        
        // 3. Confirm Connection
        this.update(&mut cx, |app, cx| {
            app.connection_status = ConnectionStatus::Connected;
            cx.notify();
        });
    }).detach();
}
```

## 3. UI Rendering (`src/app.rs` -> `src/ui/*.rs`)

The `render` loop is declarative.

```rust
impl Render for KubeSparkApp {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        // Check Platform for Titlebar Style
        let is_macos = cfg!(target_os = "macos");
        
        div()
            .flex().flex_col().h_full().w_full()
            .bg(rgb(0x18181b)) // Dark background
            // Title Bar
            .child(self.render_title_bar(cx)) 
            // Main Body
            .child(
                div().flex().h_full()
                    // Sidebar
                    .child(
                        Sidebar::new(...)
                            .on_switch_context(...) // Callback passing
                    )
                    // Content Area (Dashboard or Resource List)
                    .child(...)
            )
    }
}
```

## 4. Kubernetes Client (`src/kubernetes/client.rs`)

The client wraps `kube::Client` and `kube::config::Kubeconfig`.

- **`list_contexts`**: Reads `~/.kube/config` directly using `Kubeconfig::read()`. This prevents needing to initialize a full client just to see what clusters are available.
- **`connect_with_context`**: Re-initializes the internal `kube::Client` to point to the selected cluster. All subsequent calls (like `list_pods`) use this new active client.
