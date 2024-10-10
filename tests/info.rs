use axum_test::TestServer;
use fms_guardrails_orchestr8::server::get_health_app;
use hyper::StatusCode;
use serde_json::Value;
use tracing_test::traced_test;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

mod common;
use common::{detector_mock, shared_state, ONCE};

#[traced_test]
#[tokio::test]
async fn test_info_when_detector_is_healthy() {
    let detector_name = "content_test_detector";
    // Initialize mocked detector
    let mock_server = detector_mock("localhost", "8000").await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    // Initialize orchestrator
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let server = TestServer::new(get_health_app(shared_state)).unwrap();

    // make orchestrator request
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

    // Initialize mocked detector
    let mock_server = detector_mock("localhost", "9000").await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&mock_server)
        .await;

    // Initialize orchestrator
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let server = TestServer::new(get_health_app(shared_state)).unwrap();

    // make orchestrator request
    let response = server.get("/info").await;
    let body: Value = serde_json::from_str(response.text().as_str()).unwrap();

    // assertions
    response.assert_status(StatusCode::OK);
    assert_eq!(503, body["services"][detector_name]["code"]);
    assert_eq!("UNHEALTHY", body["services"][detector_name]["status"]);
}
