/// ReviewAgent trait definition.
///
/// Responsibilities:
/// - Provide a small interface agents implement to participate in the pipeline
/// - `name()` returns a stable identifier for the agent
/// - `review()` runs the agent logic against an `AgentInput` and returns an `AgentResult`
use async_trait::async_trait;
use crate::models::{AgentInput, AgentResult};
use anyhow::Result;

#[async_trait]
pub trait ReviewAgent {
    fn name(&self) -> &'static str;

    async fn review(&self, input: AgentInput) -> Result<AgentResult>;
}
