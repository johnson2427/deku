use common::WorkloadSpec;
use std::fs;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let manifest = fs::read_to_string("manifests/demo-pod.yaml").unwrap();
    let job = WorkloadSpec {
        id: Uuid::new_v4().to_string(),
        manifest: manifest,
    };
    let res = reqwest::Client::new()
        .post("http://localhost:8080/jobs")
        .json(&job)
        .send()
        .await
        .unwrap();
    println!("Response: {:?}", res.status());
}
