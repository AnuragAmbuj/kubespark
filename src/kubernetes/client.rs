#![allow(dead_code)]

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use k8s_openapi::api::{
    apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet},
    batch::v1::{CronJob, Job},
    core::v1::{ConfigMap, Namespace, Node, Pod, Secret, Service},
    networking::v1::Ingress,
};
use kube::{
    api::{ListParams, LogParams},
    config::{KubeConfigOptions, Kubeconfig},
    Api, Client, Config,
};
use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{ResourceItem, ResourceKind};

#[derive(Clone)]
pub struct KubeClient {
    client: Arc<RwLock<Option<Client>>>,
}

impl KubeClient {
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn connect(&self) -> Result<()> {
        let client = Client::try_default().await.context(
            "Failed to create Kubernetes client. Make sure kubeconfig is properly configured.",
        )?;

        *self.client.write().await = Some(client);
        Ok(())
    }

    pub async fn list_contexts() -> Result<Vec<String>> {
        info!("Listing contexts from Kubeconfig");
        let kubeconfig = Kubeconfig::read().map_err(|e| {
            error!("Failed to read kubeconfig: {}", e);
            e
        })?;
        Ok(kubeconfig.contexts.into_iter().map(|c| c.name).collect())
    }

    pub async fn get_current_context() -> Result<String> {
        info!("Getting current context from Kubeconfig");
        let kubeconfig = Kubeconfig::read().map_err(|e| {
            error!("Failed to read kubeconfig: {}", e);
            e
        })?;
        kubeconfig.current_context.ok_or_else(|| {
            let e = anyhow!("No current context set");
            error!("{}", e);
            e
        })
    }

    pub async fn connect_with_context(&self, context_name: &str) -> Result<()> {
        let options = KubeConfigOptions {
            context: Some(context_name.to_owned()),
            ..Default::default()
        };
        let config = Kubeconfig::read()?;
        let client_config = Config::from_custom_kubeconfig(config, &options).await?;
        let client = Client::try_from(client_config)?;

        *self.client.write().await = Some(client);
        Ok(())
    }

    pub async fn is_connected(&self) -> bool {
        self.client.read().await.is_some()
    }

    async fn get_client(&self) -> Result<Client> {
        self.client
            .read()
            .await
            .clone()
            .context("Not connected to Kubernetes cluster")
    }

    pub async fn list_namespaces(&self) -> Result<Vec<String>> {
        let client = self.get_client().await?;
        let api: Api<Namespace> = Api::all(client);
        let namespaces = api.list(&ListParams::default()).await?;

        Ok(namespaces
            .items
            .iter()
            .filter_map(|ns| ns.metadata.name.clone())
            .collect())
    }

    pub async fn list_resources(
        &self,
        kind: ResourceKind,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let client = self.get_client().await?;

        match kind {
            ResourceKind::Namespace => self.list_namespaces_as_items().await,
            ResourceKind::Pod => self.list_pods(&client, namespace).await,
            ResourceKind::Deployment => self.list_deployments(&client, namespace).await,
            ResourceKind::StatefulSet => self.list_statefulsets(&client, namespace).await,
            ResourceKind::DaemonSet => self.list_daemonsets(&client, namespace).await,
            ResourceKind::ReplicaSet => self.list_replicasets(&client, namespace).await,
            ResourceKind::Service => self.list_services(&client, namespace).await,
            ResourceKind::Job => self.list_jobs(&client, namespace).await,
            ResourceKind::CronJob => self.list_cronjobs(&client, namespace).await,
            ResourceKind::ConfigMap => self.list_configmaps(&client, namespace).await,
            ResourceKind::Secret => self.list_secrets(&client, namespace).await,
            ResourceKind::Ingress => self.list_ingresses(&client, namespace).await,
            ResourceKind::Node => self.list_nodes(&client).await,
            _ => Ok(vec![]),
        }
    }

    async fn list_namespaces_as_items(&self) -> Result<Vec<ResourceItem>> {
        let client = self.get_client().await?;
        let api: Api<Namespace> = Api::all(client);
        let namespaces = api.list(&ListParams::default()).await?;

        Ok(namespaces
            .items
            .iter()
            .filter_map(|ns| {
                let name = ns.metadata.name.clone()?;
                let status = ns
                    .status
                    .as_ref()
                    .and_then(|s| s.phase.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = ns
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::Namespace,
                    name,
                    namespace: None,
                    status,
                    age,
                    metadata: serde_json::to_value(ns).ok()?,
                })
            })
            .collect())
    }

    async fn list_pods(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<Pod> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let pods = api.list(&ListParams::default()).await?;

        Ok(pods
            .items
            .iter()
            .filter_map(|pod| {
                let name = pod.metadata.name.clone()?;
                let namespace = pod.metadata.namespace.clone();
                let status = pod
                    .status
                    .as_ref()
                    .and_then(|s| s.phase.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = pod
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::Pod,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(pod).ok()?,
                })
            })
            .collect())
    }

    async fn list_deployments(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<Deployment> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let deployments = api.list(&ListParams::default()).await?;

        Ok(deployments
            .items
            .iter()
            .filter_map(|deploy| {
                let name = deploy.metadata.name.clone()?;
                let namespace = deploy.metadata.namespace.clone();
                let status = deploy
                    .status
                    .as_ref()
                    .map(|s| {
                        format!(
                            "{}/{}",
                            s.ready_replicas.unwrap_or(0),
                            s.replicas.unwrap_or(0)
                        )
                    })
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = deploy
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::Deployment,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(deploy).ok()?,
                })
            })
            .collect())
    }

    async fn list_statefulsets(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<StatefulSet> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let statefulsets = api.list(&ListParams::default()).await?;

        Ok(statefulsets
            .items
            .iter()
            .filter_map(|ss| {
                let name = ss.metadata.name.clone()?;
                let namespace = ss.metadata.namespace.clone();
                let status = ss
                    .status
                    .as_ref()
                    .map(|s| format!("{}/{}", s.ready_replicas.unwrap_or(0), s.replicas))
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = ss
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::StatefulSet,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(ss).ok()?,
                })
            })
            .collect())
    }

    async fn list_daemonsets(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<DaemonSet> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let daemonsets = api.list(&ListParams::default()).await?;

        Ok(daemonsets
            .items
            .iter()
            .filter_map(|ds| {
                let name = ds.metadata.name.clone()?;
                let namespace = ds.metadata.namespace.clone();
                let status = ds
                    .status
                    .as_ref()
                    .map(|s| format!("{}/{}", s.number_ready, s.desired_number_scheduled))
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = ds
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::DaemonSet,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(ds).ok()?,
                })
            })
            .collect())
    }

    async fn list_replicasets(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<ReplicaSet> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let replicasets = api.list(&ListParams::default()).await?;

        Ok(replicasets
            .items
            .iter()
            .filter_map(|rs| {
                let name = rs.metadata.name.clone()?;
                let namespace = rs.metadata.namespace.clone();
                let status = rs
                    .status
                    .as_ref()
                    .map(|s| format!("{}/{}", s.ready_replicas.unwrap_or(0), s.replicas))
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = rs
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::ReplicaSet,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(rs).ok()?,
                })
            })
            .collect())
    }

    async fn list_services(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<Service> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let services = api.list(&ListParams::default()).await?;

        Ok(services
            .items
            .iter()
            .filter_map(|svc| {
                let name = svc.metadata.name.clone()?;
                let namespace = svc.metadata.namespace.clone();
                let status = svc
                    .spec
                    .as_ref()
                    .and_then(|s| s.type_.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = svc
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::Service,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(svc).ok()?,
                })
            })
            .collect())
    }

    async fn list_jobs(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<Job> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let jobs = api.list(&ListParams::default()).await?;

        Ok(jobs
            .items
            .iter()
            .filter_map(|job| {
                let name = job.metadata.name.clone()?;
                let namespace = job.metadata.namespace.clone();
                let status = job
                    .status
                    .as_ref()
                    .map(|s| format!("{}/{}", s.succeeded.unwrap_or(0), s.active.unwrap_or(0)))
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = job
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::Job,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(job).ok()?,
                })
            })
            .collect())
    }

    async fn list_cronjobs(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<CronJob> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let cronjobs = api.list(&ListParams::default()).await?;

        Ok(cronjobs
            .items
            .iter()
            .filter_map(|cj| {
                let name = cj.metadata.name.clone()?;
                let namespace = cj.metadata.namespace.clone();
                let status = cj
                    .spec
                    .as_ref()
                    .map(|s| s.schedule.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = cj
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::CronJob,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(cj).ok()?,
                })
            })
            .collect())
    }

    async fn list_configmaps(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<ConfigMap> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let configmaps = api.list(&ListParams::default()).await?;

        Ok(configmaps
            .items
            .iter()
            .filter_map(|cm| {
                let name = cm.metadata.name.clone()?;
                let namespace = cm.metadata.namespace.clone();
                let status = cm
                    .data
                    .as_ref()
                    .map(|d| format!("{} keys", d.len()))
                    .unwrap_or_else(|| "0 keys".to_string());
                let age = cm
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::ConfigMap,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(cm).ok()?,
                })
            })
            .collect())
    }

    async fn list_secrets(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<Secret> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let secrets = api.list(&ListParams::default()).await?;

        Ok(secrets
            .items
            .iter()
            .filter_map(|secret| {
                let name = secret.metadata.name.clone()?;
                let namespace = secret.metadata.namespace.clone();
                let status = secret
                    .type_
                    .as_ref()
                    .map(|t| t.clone())
                    .unwrap_or_else(|| "Opaque".to_string());
                let age = secret
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::Secret,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(secret).ok()?,
                })
            })
            .collect())
    }

    async fn list_ingresses(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> Result<Vec<ResourceItem>> {
        let api: Api<Ingress> = if let Some(ns) = namespace {
            Api::namespaced(client.clone(), ns)
        } else {
            Api::all(client.clone())
        };

        let ingresses = api.list(&ListParams::default()).await?;

        Ok(ingresses
            .items
            .iter()
            .filter_map(|ing| {
                let name = ing.metadata.name.clone()?;
                let namespace = ing.metadata.namespace.clone();
                let status = ing
                    .spec
                    .as_ref()
                    .and_then(|s| s.ingress_class_name.clone())
                    .unwrap_or_else(|| "default".to_string());
                let age = ing
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::Ingress,
                    name,
                    namespace,
                    status,
                    age,
                    metadata: serde_json::to_value(ing).ok()?,
                })
            })
            .collect())
    }

    async fn list_nodes(&self, client: &Client) -> Result<Vec<ResourceItem>> {
        let api: Api<Node> = Api::all(client.clone());
        let nodes = api.list(&ListParams::default()).await?;

        Ok(nodes
            .items
            .iter()
            .filter_map(|node| {
                let name = node.metadata.name.clone()?;
                let status = node
                    .status
                    .as_ref()
                    .and_then(|s| s.conditions.as_ref())
                    .and_then(|conds| {
                        conds
                            .iter()
                            .find(|c| c.type_ == "Ready")
                            .map(|c| c.status.clone())
                    })
                    .unwrap_or_else(|| "Unknown".to_string());
                let age = node
                    .metadata
                    .creation_timestamp
                    .as_ref()
                    .map(|ts| format_age(&ts.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(ResourceItem {
                    kind: ResourceKind::Node,
                    name,
                    namespace: None,
                    status,
                    age,
                    metadata: serde_json::to_value(node).ok()?,
                })
            })
            .collect())
    }

    pub async fn get_pod_logs(
        &self,
        namespace: &str,
        pod_name: &str,
        container: Option<&str>,
    ) -> Result<String> {
        let client = self.get_client().await?;
        let api: Api<Pod> = Api::namespaced(client, namespace);

        let mut log_params = LogParams::default();
        if let Some(container_name) = container {
            log_params.container = Some(container_name.to_string());
        }

        let logs = api.logs(pod_name, &log_params).await?;
        Ok(logs)
    }
}

#[allow(dead_code)]
fn format_age(timestamp: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(*timestamp);

    if duration.num_days() > 0 {
        format!("{}d", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{}h", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{}m", duration.num_minutes())
    } else {
        format!("{}s", duration.num_seconds())
    }
}
