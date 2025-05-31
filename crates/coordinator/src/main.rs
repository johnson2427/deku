use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum::{
    Router,
    routing::{get, post},
};
use common::WorkloadSpec;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

type Queue = Arc<Mutex<VecDeque<WorkloadSpec>>>;

#[tokio::main]
async fn main() {
    let queue: Queue = Arc::new(Mutex::new(VecDeque::new()));
    let app = Router::new()
        .route("/jobs", post(submit_job))
        .route("/jobs/next", get(next_job))
        .route("/jobs/done", post(job_done))
        .with_state(queue);

    let listener = tokio::net::TcpListener::bind("0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn submit_job(
    State(queue): State<Arc<Mutex<VecDeque<WorkloadSpec>>>>,
    Json(job): Json<WorkloadSpec>,
) -> impl IntoResponse {
    queue.lock().unwrap().push_back(job);
    StatusCode::OK
}

async fn next_job(state: axum::extract::State<Queue>) -> Json<Option<WorkloadSpec>> {
    Json(state.lock().unwrap().pop_front())
}

async fn job_done(Json(job): Json<WorkloadSpec>) {
    println!("✅ Job complete: {}", job.id);
}
