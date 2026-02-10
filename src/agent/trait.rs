use async_trait::async_trait;
use crate::models::{AgentInput, AgentResult};
use anyhow::Result;

#[async_trait]
pub trait ReviewAgent {
    fn name(&self) -> &'static str;

    async fn review(&self, input: AgentInput) -> Result<AgentResult>;
}
