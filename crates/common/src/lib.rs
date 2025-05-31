use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadSpec {
    pub id: String,
    pub manifest: String, // Raw Kubernetes YAML
}
