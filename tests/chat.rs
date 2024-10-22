use axum_test::TestServer;
use fms_guardrails_orchestr8::server::get_app;
use hyper::StatusCode;
use serde_json::json;
use tracing_test::traced_test;

mod common;
use common::{shared_state, ONCE};

const ORCHESTRATOR_CHAT_DETECTION_ENDPOINT: &str = "/api/v2/text/detection/chat";
const MOCKED_DETECTOR_NAME: &str = "chat_test_detector";

#[traced_test]
#[tokio::test]
async fn test_chat_detection_with_string_message() {
    // Initialize orchestrator
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let server = TestServer::new(get_app(shared_state)).unwrap();

    // make orchestrator request
    let response = server
        .post(ORCHESTRATOR_CHAT_DETECTION_ENDPOINT)
        .json(&json!({
            "messages": [
                {
                    "content": "This is the content of the message",
                    "role": "user"
                }
            ],
            "detectors": {
                MOCKED_DETECTOR_NAME: {}
            }
        }))
        .await;

    // assertions
    response.assert_status(StatusCode::OK);
    response.assert_json(&json!({
        "detections": [
            {
                "detection_type": "is_message_valid",
                "detection": "Yes",
                "score": 1.0
            }
        ]
    }));
}

#[traced_test]
#[tokio::test]
async fn test_chat_detection_with_text_content_message() {
    // Initialize orchestrator
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let server = TestServer::new(get_app(shared_state)).unwrap();

    // make orchestrator request
    let response = server
        .post(ORCHESTRATOR_CHAT_DETECTION_ENDPOINT)
        .json(&json!({
            "messages": [
                {
                    "content": [
                      {
                        "type": "text",
                        "text": "This is the content of the message"
                      }
                    ],
                    "role": "user"
                }
            ],
            "detectors": {
                MOCKED_DETECTOR_NAME: {}
            }
        }))
        .await;

    // assertions
    response.assert_status(StatusCode::OK);
    response.assert_json(&json!({
        "detections": [
            {
                "detection_type": "is_message_valid",
                "detection": "Yes",
                "score": 0.9
            }
        ]
    }));
}
