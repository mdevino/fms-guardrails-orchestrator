use std::sync::Arc;

use fms_guardrails_orchestr8::{
    config::OrchestratorConfig, orchestrator::Orchestrator, server::ServerState,
};
use tokio::sync::OnceCell;
use wiremock::MockServer;

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

#[inline]
pub async fn detector_mock(host: &str, port: &str) -> MockServer {
    let listener = std::net::TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    MockServer::builder().listener(listener).start().await
}
