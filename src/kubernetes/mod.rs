mod client;
mod resources;

pub use client::KubeClient;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceKind {
    Namespace,
    Pod,
    Deployment,
    StatefulSet,
    DaemonSet,
    ReplicaSet,
    Service,
    Job,
    CronJob,
    ConfigMap,
    Secret,
    Ingress,
    PersistentVolume,
    PersistentVolumeClaim,
    Node,
}

impl ResourceKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Namespace,
            Self::Pod,
            Self::Deployment,
            Self::StatefulSet,
            Self::DaemonSet,
            Self::ReplicaSet,
            Self::Service,
            Self::Job,
            Self::CronJob,
            Self::ConfigMap,
            Self::Secret,
            Self::Ingress,
            Self::PersistentVolume,
            Self::PersistentVolumeClaim,
            Self::Node,
        ]
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Namespace => "Namespaces",
            Self::Pod => "Pods",
            Self::Deployment => "Deployments",
            Self::StatefulSet => "StatefulSets",
            Self::DaemonSet => "DaemonSets",
            Self::ReplicaSet => "ReplicaSets",
            Self::Service => "Services",
            Self::Job => "Jobs",
            Self::CronJob => "CronJobs",
            Self::ConfigMap => "ConfigMaps",
            Self::Secret => "Secrets",
            Self::Ingress => "Ingresses",
            Self::PersistentVolume => "PersistentVolumes",
            Self::PersistentVolumeClaim => "PersistentVolumeClaims",
            Self::Node => "Nodes",
        }
    }

    pub fn category(&self) -> &str {
        match self {
            Self::Namespace => "Cluster",
            Self::Node => "Cluster",
            Self::Pod
            | Self::Deployment
            | Self::StatefulSet
            | Self::DaemonSet
            | Self::ReplicaSet => "Workloads",
            Self::Job | Self::CronJob => "Workloads",
            Self::Service | Self::Ingress => "Network",
            Self::ConfigMap | Self::Secret => "Config",
            Self::PersistentVolume | Self::PersistentVolumeClaim => "Storage",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceItem {
    pub kind: ResourceKind,
    pub name: String,
    pub namespace: Option<String>,
    pub status: String,
    pub age: String,
    pub restart_count: Option<i32>,
    pub node_name: Option<String>,
    pub pod_ip: Option<String>,
    pub metadata: serde_json::Value,
}
