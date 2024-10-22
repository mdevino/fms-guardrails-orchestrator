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

use std::sync::Arc;

use fms_guardrails_orchestr8::{
    config::OrchestratorConfig, orchestrator::Orchestrator, server::ServerState,
};
use tokio::sync::OnceCell;

/// Async lazy initialization of shared state using tokio::sync::OnceCell
pub static ONCE: OnceCell<Arc<ServerState>> = OnceCell::const_new();

/// The actual async function that initializes the shared state if not already initialized
pub async fn shared_state() -> Arc<ServerState> {
    let config = OrchestratorConfig::load("tests/test.config.yaml")
        .await
        .unwrap();
    let orchestrator = Orchestrator::new(config, false).await.unwrap();
    Arc::new(ServerState::new(orchestrator))
}
