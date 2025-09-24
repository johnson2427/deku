use subxt::{OnlineClient, Config};
use subxt::config::SubstrateConfig;

use subxt::ext::sp_core::sr25519::Pair as Sr25519Pair;
use subxt::{OnlineClient, PolkadotConfig};
use subxt::tx::PairSigner;
use kube::{Api, Client, api::PostParams};
use k8s_openapi::api::core::v1::Pod;
use serde_yaml;
use std::time::Duration;
use tokio::time::sleep;

let api = OnlineClient::<SubstrateConfig>::new().await?;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod runtime {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let signer = PairSigner::new(Sr25519Pair::from_string("//Alice", None)?);
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    loop {
        let job = api
            .tx()
            .contracts()
            .call(
                /* contract address */,
                0, // value
                100_000_000_000u64, // gas limit
                runtime::runtime_types::your::types::GetJob(0), // placeholder
            )
            .sign_and_submit_then_watch(&signer)
            .await?
            .wait_for_finalized_success()
            .await?;

        // Decode manifest
        if let Some(job_bytes) = job.result {
            let yaml: String = String::from_utf8(job_bytes.manifest)?;
            let pod: Pod = serde_yaml::from_str(&yaml)?;
            let kube_client = Client::try_default().await?;
            let pods: Api<Pod> = Api::default_namespaced(kube_client);
            pods.create(&PostParams::default(), &pod).await?;

            // Mark job as complete
            api.tx()
                .contracts()
                .call(
                    /* contract address */,
                    0,
                    100_000_000_000u64,
                    runtime::runtime_types::your::types::CompleteJob(job_bytes.id),
                )
                .sign_and_submit_then_watch(&signer)
                .await?
                .wait_for_finalized_success()
                .await?;
        }

        sleep(Duration::from_secs(10)).await;
    }
}


