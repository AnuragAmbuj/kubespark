# Application Walkthrough

This guide explains how to use the specific features of KubeSpark.

## 1. The Interface Layout
- **Left Sidebar**: Primary navigation. Contains the Context Switcher at the top and Resource Categories (Cluster, Workloads, Network, etc.) below.
- **Top Titlebar**: Contains the application title, and on Linux, the window controls (Minimize, Maximize, Close). On the right, there is a Settings (Gear) icon.
- **Main Content**: Displays the Dashboard or the list of resources.
- **Bottom Status Bar**: Shows connection status (Connected/Disconnected) and version.

## 2. Managing Clusters (Contexts)

### Switching Contexts
1. Look at the top of the Sidebar. You will see the name of the current cluster (e.g., `minikube` or `docker-desktop`).
2. Click on the name. A dropdown menu will appear listing all contexts found in your `~/.kube/config`.
3. Select a new context.
   - The UI will immediately update the name.
   - The status bar might briefly blink "Connecting".
   - Once connected, resource views will refresh (Planned feature: Auto-refresh).

## 3. Viewing Resources

### Dashboard
The default view provides a summary of the cluster.
- **System Status**: Shows health of Node Controller and Scheduler.
- **Resource Usage**: (Planned) CPU/Memory capability.

### Workloads
Click on **Workloads > Pods** in the sidebar.
- You will see a table listing all Pods in the current namespace.
- **Columns**: Name, Namespace, Status (Running/Pending), Age.
- **Interaction**: Click a row to open the Detail View (Side panel) for that pod.

## 4. Window Management
- **Resize**: Drag any edge of the window.
- **Move**: Click and drag the Titlebar.
- **Minimize/Maximize**:
  - **Linux**: Use the `-` and `□` buttons in the top-right.
  - **macOS**: Use the system traffic lights in the top-left.
