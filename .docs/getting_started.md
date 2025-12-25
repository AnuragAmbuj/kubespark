# Getting Started

## Prerequisites

Before trying to build KubeSpark, ensure you have the following installed:

1. **Rust Toolchain**: You need the latest stable Rust.
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Kubernetes Environment**:
   - `kubectl` installed.
   - A valid `~/.kube/config` file.
   - Access to at least one cluster (can be local like Minikube, Kind, or remote).

3. **System Dependencies (Linux)**:
   GPUI requires specific libraries for rendering and input.
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libfontconfig1-dev libssl-dev
   ```
   *(Note: macOS typically requires Xcode Command Line Tools).*

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/AnuragAmbuj/kubespark.git
   cd kubespark
   ```

2. **Build**:
   Compile the project. This will fetch all crates (GPUI, Tokio, Kube-rs).
   ```bash
   cargo build --release
   ```
   *The first build may take a few minutes as it compiles the GPUI framework.*

## Running the Application

To run the application locally during development:

```bash
cargo run
```

### Logging
KubeSpark uses `env_logger`. To see detailed logs (helpful for debugging Kubernetes connections):

```bash
RUST_LOG=info cargo run
# or for more verbosity
RUST_LOG=debug cargo run
```

## Troubleshooting Common Issues

**"Reactor not running" Panic**
This occurs if the Tokio runtime is not initialized correctly. Ensure `src/main.rs` contains the `tokio::runtime::Builder` block before `Application::new()`.

**Window Controls Missing (Linux)**
If minimize/maximize buttons don't work, ensure you are running a supported Window Manager (most standard GNOME/KDE/Tiling WMs work).

**Authentication Failed**
If KubeSpark says "Error" or "Disconnected":
1. Check if `kubectl get pods` works in your terminal.
2. Ensure your kubeconfig doesn't rely on external auth plugins that are missing from your path (though `kube-rs` supports most standard auth providers).
