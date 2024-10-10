use axum_test::TestServer;
use fms_guardrails_orchestr8::server::get_health_app;
use hyper::StatusCode;

mod common;
use common::{shared_state, ONCE};

#[tokio::test]
async fn test_health() {
    // Initialize orchestrator
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let server = TestServer::new(get_health_app(shared_state)).unwrap();

    let response = server.get("/health").await;
    response.assert_status(StatusCode::OK);
}
