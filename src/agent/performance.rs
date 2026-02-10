use crate::{
    agent::r#trait::ReviewAgent,
    llm::client::LlmClient,
    models::{AgentInput, AgentResult, AgentFinding},
};
use async_trait::async_trait;
use anyhow::Result;

pub struct PerformanceAgent{
    llm: LlmClient,
}

impl PerformanceAgent {
    pub fn new(llm: LlmClient) -> Self {
        Self { llm }
    }
}
#[async_trait]
impl ReviewAgent for PerformanceAgent {
    fn name(&self) -> &'static str {
        "PerformanceAgent"
    }

    async fn review(&self, input: AgentInput) -> Result<AgentResult> {
        let prompt = crate::agent::prompts::performance_prompt(&input.diff);

        let raw = self.llm.send_prompt(&prompt).await?;

        let parsed: serde_json::Value = serde_json::from_str(&raw)?;

        let findings: Vec<AgentFinding> =
            serde_json::from_value(parsed["findings"].clone())?;

        Ok(AgentResult {
            agent_name: self.name().to_string(),
            findings,
        })
    }
}
