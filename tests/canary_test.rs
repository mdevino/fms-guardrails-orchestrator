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
use fms_guardrails_orchestr8::server::get_health_app;
use hyper::StatusCode;
use serde_json::Value;
use tracing::debug;
use tracing_test::traced_test;

mod common;
use common::{shared_state, ONCE};
/// Checks if the health endpoint is working
/// NOTE: We do not currently mock client services yet, so this test is
/// superficially testing the client health endpoints on the orchestrator is accessible
/// and when the orchestrator is running (healthy) all the health endpoints return 200 OK.
/// This will happen even if the client services or their health endpoints are not found.
#[traced_test]
#[tokio::test]
async fn test_health() {
    let shared_state = ONCE.get_or_init(shared_state).await.clone();
    let server = TestServer::new(get_health_app(shared_state)).unwrap();
    let response = server.get("/health").await;
    debug!("{:#?}", response);
    let body: Value = serde_json::from_str(response.text().as_str()).unwrap();
    debug!("{}", serde_json::to_string_pretty(&body).unwrap());
    response.assert_status(StatusCode::OK);
    let response = server.get("/info").await;
    println!("{:#?}", response);
    let body: Value = serde_json::from_str(response.text().as_str()).unwrap();
    println!("{}", serde_json::to_string_pretty(&body).unwrap());
    response.assert_status(StatusCode::OK);
}
