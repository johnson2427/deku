use subxt::{OnlineClient, PolkadotConfig};
use subxt::events::EventSubscription;
use deku::deku::events::JobSubmitted;

#[subxt::subxt(runtime_metadata_path = "../metadata.scale")]
pub mod deku {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let mut sub: EventSubscription<'_, PolkadotConfig> = api.events().subscribe().await?;

    println!("Listening for submitted jobs...");

    while let Some(ev) = sub.next().await {
        let events = ev?.events().collect::<Vec<_>>();
        for ev in events {
            if let Ok(ev) = ev.as_event::<JobSubmitted>() {
                if let Some(JobSubmitted(job_id)) = ev {
                    println!("Job submitted with id: {}", job_id);
                    // TODO: Claim and complete job
                }
            }
        }
    }

    Ok(())
}

