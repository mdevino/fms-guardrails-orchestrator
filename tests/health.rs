use std::sync::Arc;

use axum_test::TestServer;
use fms_guardrails_orchestr8::{
    config::OrchestratorConfig,
    orchestrator::Orchestrator,
    server::{get_health_app, ServerState},
};
use hyper::StatusCode;
use serde_json::Value;
use tokio::sync::OnceCell;
use tracing_test::traced_test;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

/// Async lazy initialization of shared state using tokio::sync::OnceCell
static ONCE: OnceCell<Arc<ServerState>> = OnceCell::const_new();

/// The actual async function that initializes the shared state if not already initialized
async fn shared_state() -> Arc<ServerState> {
    let config = OrchestratorConfig::load("tests/test.config.yaml")
        .await
        .unwrap();
    let orchestrator = Orchestrator::new(config, false).await.unwrap();
    Arc::new(ServerState::new(orchestrator))
}

#[tokio::test]
async fn test_health() {
    let listener = std::net::TcpListener::bind("localhost:8000").unwrap();

    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let mock_server = MockServer::builder().listener(listener).start().await;

    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let server = TestServer::new(get_health_app(shared_state)).unwrap();
    let response = server.get("/health").await;
    response.assert_status(StatusCode::OK);
}

#[traced_test]
#[tokio::test]
async fn test_info_when_detector_is_healthy() {
    let detector_name = "content_test_detector";

    // Initialize mock detector
    let listener = std::net::TcpListener::bind("localhost:8000").unwrap();
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let mock_server = MockServer::builder().listener(listener).start().await;

    // mock detector
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    // make orchestrator request
    let server = TestServer::new(get_health_app(shared_state)).unwrap();
    let response = server.get("/info").await;
    let body: Value = serde_json::from_str(response.text().as_str()).unwrap();

    // assertions
    response.assert_status(StatusCode::OK);
    assert_eq!("HEALTHY", body["services"][detector_name]["status"]);
}

#[traced_test]
#[tokio::test]
async fn test_info_when_detector_returns_503() {
    let detector_name = "content_test_detector_error_503";

    // Initialize mock detector
    let listener = std::net::TcpListener::bind("localhost:9000").unwrap();
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let mock_server = MockServer::builder().listener(listener).start().await;

    // mock detector
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&mock_server)
        .await;

    // make orchestrator request
    let server = TestServer::new(get_health_app(shared_state)).unwrap();
    let response = server.get("/info").await;
    let body: Value = serde_json::from_str(response.text().as_str()).unwrap();

    // assertions
    response.assert_status(StatusCode::OK);
    assert_eq!(503, body["services"][detector_name]["code"]);
    assert_eq!("UNHEALTHY", body["services"][detector_name]["status"]);
}
