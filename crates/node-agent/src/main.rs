use common::WorkloadSpec;
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client, api::PostParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let pod_api: Api<Pod> = Api::default_namespaced(client);

    let job: Option<WorkloadSpec> = reqwest::get("http://localhost:8080/jobs/next")
        .await?
        .json()
        .await?;

    if let Some(job) = job {
        let manifest: Pod = serde_yaml::from_str(&job.manifest)?;
        pod_api.create(&PostParams::default(), &manifest).await?;

        reqwest::Client::new()
            .post("http://localhost:8080/jobs/done")
            .json(&job)
            .send()
            .await?;
    } else {
        println!("No jobs available");
    }

    Ok(())
}
