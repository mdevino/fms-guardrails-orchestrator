use axum_test::TestServer;
use fms_guardrails_orchestr8::server::get_health_app;
use hyper::StatusCode;
use serde_json::json;
use tracing_test::traced_test;
use wiremock::{
    matchers::{body_json, method, path},
    Mock, ResponseTemplate,
};

mod common;
use common::{detector_mock, shared_state, ONCE};

#[traced_test]
#[tokio::test]
async fn test_detection_generated() {
    let detector_name = "generation_test_detector";

    // Initialize mocked detector
    let mock_server = detector_mock("localhost", "8001").await;
    Mock::given(method("POST"))
        .and(path("/api/v1/text/generation"))
        .and(body_json(&json!({
            "prompt": "What is the capital of Brazil?",
            "generated_text": "The capital of Brazil is Brasilia."
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(&json!([
                {
                    "detection": "relevant",
                    "detection_type": "dummy_detector_type",
                    "score": 0.89
                }
        ])))
        .mount(&mock_server)
        .await;

    // Initialize orchestrator
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let server = TestServer::new(get_health_app(shared_state)).unwrap();

    // make orchestrator request
    let response = server
        .post("/api/v2/text/detection/generated")
        .json(&json!({
            "prompt": "What is the capital of Brazil?",
            "generated_text": "The capital of Brazil is Brasilia.",
            "detectors": {
                detector_name: {}
            }
        }))
        .await;

    // assertions
    response.assert_status(StatusCode::OK);
    response.assert_json(&json!({
        "detections": [
            {
                "detection": "relevant",
                "detection_type": "dummy_detector_type",
                "score": 0.89
            }
        ]
    }));
}
