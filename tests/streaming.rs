/*
 Copyright FMS Guardrails Orchestrator Authors

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.

*/

use axum_test::TestServer;
use fms_guardrails_orchestr8::server::get_app;
use hyper::StatusCode;
use serde_json::json;
use tracing_test::traced_test;

mod common;
use common::{shared_state, ONCE};

const ORCHESTRATOR_STREAMING_ENDPOINT: &str =
    "/api/v1/task/server-streaming-classification-with-text-generation";
const MOCKED_DETECTOR_NAME: &str = "content_test_detector";

#[traced_test]
#[tokio::test]
async fn test_streaming_no_detectors() {
    // Initialize orchestrator
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let server = TestServer::new(get_app(shared_state)).unwrap();

    // make orchestrator request
    let response = server
        .post(ORCHESTRATOR_STREAMING_ENDPOINT)
        .json(&json!({
            "model_id": "potato-70B",
            "inputs": "This is my input",
            "guardrails_config": {
                "input": {
                    "models": {}
                },
                "output": {
                    "models": {}
                }
            }
        }))
        .await;

    // assertions
    response.assert_status(StatusCode::OK);
    tracing::info!("{:#?}", response);
    // response.assert_json(&json!({
    //     "generated_text": ":\nI have a table with a column with a value of 1 and a column with a value",
    //     "token_classification_results": {},
    //     "finish_reason": "MAX_TOKENS",
    //     "generated_token_count": 20,
    //     "seed": 0,
    //     "input_token_count": 4
    // }));
}
