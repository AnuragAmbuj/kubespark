# Developer Guide

## Development Workflow

### Adding a New Resource Type
Current support covers basic Pods and Nodes. To add something like `Deployments`:

1. **Update Enum**:
   Edit `src/kubernetes/mod.rs` and add `Deployment` to the `ResourceKind` enum.

2. **Update Client**:
   In `src/kubernetes/client.rs`:
   - Import `k8s_openapi::api::apps::v1::Deployment`.
   - Add a method `list_deployments`.
   - Update the generic `list_resources(kind)` method to match `ResourceKind::Deployment` and call your new method.

3. **Update UI**:
   - `src/ui/sidebar.rs`: Ensure `Deployment` is categorized correctly (Likely under Workloads).
   - `src/ui/resource_list.rs`: If Deployments need special columns (e.g., Replicas desired/current), update the `render_row` match statement.

### Debugging

**Logs** are your best friend.
- We use `log` crate macros: `info!`, `warn!`, `error!`, `debug!`.
- Logs are output to `stdout/stderr`.
- Running `RUST_LOG=debug cargo run` will show you exactly when KubeSpark tries to connect or fetch data.

**GPUI Debugging**:
- If the UI isn't updating: Check if you called `cx.notify()`.
- If an event isn't firing: Check if `on_click` is attached to a hit-testable element (needs a background or size).
- "Reactor not running": You are trying to `await` a Future on the main thread or without entering the Tokio runtime. Use `cx.spawn` or `tokio::spawn`.

### Style Guide
- **Rust**: Follow standard `cargo fmt` (rustfmt).
- **Imports**: Group imports `std`, then external crates (`gpui`, `kube`), then internal modules (`crate::app`).
- **Dead Code**: While developing, use `#[allow(dead_code)]` sparingly. Prefer cleaning up unused methods before merging.

### Contribution Checklist
- [ ] Code builds `cargo build`.
- [ ] No warnings `cargo check`.
- [ ] Documentation updated if features changed.
- [ ] Screenshots updated if UI changed.
