# Installation Instructions

## System Dependencies

KubeSpark requires certain system libraries to build and run. Install them based on your operating system:

### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install -y \
    libxcb1-dev \
    libxkbcommon-dev \
    libxkbcommon-x11-dev \
    libvulkan-dev \
    libwayland-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libfontconfig1-dev \
    libssl-dev \
    pkg-config
```

### Fedora/RHEL
```bash
sudo dnf install -y \
    libxcb-devel \
    libxkbcommon-devel \
    libxkbcommon-x11-devel \
    vulkan-devel \
    wayland-devel \
    fontconfig-devel \
    openssl-devel \
    pkg-config
```

### Arch Linux
```bash
sudo pacman -S \
    libxcb \
    libxkbcommon \
    libxkbcommon-x11 \
    vulkan-icd-loader \
    wayland \
    fontconfig \
    openssl \
    pkg-config
```

## Building

After installing system dependencies:

```bash
# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Running

Make sure you have:
1. A valid kubeconfig file at `~/.kube/config`
2. Access to a Kubernetes cluster
3. Proper RBAC permissions to list resources

Then simply run:
```bash
./target/release/kubespark
```

Or during development:
```bash
cargo run
```

## Troubleshooting

### Linker Errors
If you see errors about missing libraries like `libxcb`, `libxkbcommon`, etc., make sure you've installed all the system dependencies listed above.

### Kubernetes Connection Issues
- Verify your kubeconfig: `kubectl cluster-info`
- Test cluster access: `kubectl get pods --all-namespaces`
- Check logs: `RUST_LOG=debug cargo run`

### Display Issues
KubeSpark uses GPU acceleration via Vulkan. If you encounter display issues:
- Ensure Vulkan drivers are installed
- For Intel: `sudo apt-get install mesa-vulkan-drivers`
- For NVIDIA: Install proprietary NVIDIA drivers
- For AMD: `sudo apt-get install mesa-vulkan-drivers`
