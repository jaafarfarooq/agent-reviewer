/// StyleAgent: a review agent focused on style issues.
///
/// Responsibilities:
/// - Build a style-focused prompt from a diff
/// - Call the LLM client to get JSON-formatted findings
/// - Parse findings into `AgentFinding` and return an `AgentResult`
use crate::{
    agent::r#trait::ReviewAgent,
    llm::client::LlmClient,
    models::{AgentInput, AgentResult, AgentFinding},
};
use async_trait::async_trait;
use anyhow::Result;

pub struct StyleAgent {
    llm: LlmClient,
}

impl StyleAgent {
    pub fn new(llm: LlmClient) -> Self {
        Self { llm }
    }
}

#[async_trait]
impl ReviewAgent for StyleAgent {
    fn name(&self) -> &'static str {
        "StyleAgent"
    }

    async fn review(&self, input: AgentInput) -> Result<AgentResult> {
        let prompt = crate::agent::prompts::style_prompt(&input.diff);

        let raw = self.llm.send_prompt(&prompt).await?;

        let parsed: serde_json::Value = serde_json::from_str(&raw)?;

        let mut findings: Vec<AgentFinding> =
            serde_json::from_value(parsed["findings"].clone())?;

        // Initialize confidence to 1.0 if not provided by LLM
        for finding in &mut findings {
            if finding.confidence == 0.0 {
                finding.confidence = 1.0;
            }
        }

        Ok(AgentResult {
            agent_name: self.name().to_string(),
            findings,
        })
    }
}
