use subxt::ext::sp_core::sr25519::Pair;
use subxt::{OnlineClient, PolkadotConfig};
use std::fs;

#[subxt::subxt(runtime_metadata_path = "../metadata.scale")]
pub mod deku {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let signer = Pair::from_string("//Alice", None)?;
    let manifest = fs::read("../manifests/demo-pod.yaml")?;
    let call = deku::tx().deku().submit_job(manifest);
    api.tx().sign_and_submit_then_watch_default(&call, &signer).await?;
    println!("Job submitted!");
    Ok(())
}
