use crate::agent::r#trait::ReviewAgent;
use crate::models::{AgentInput, AgentResult};
use anyhow::Result;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub struct Orchestrator {
    agents: Vec<Arc<dyn ReviewAgent + Send + Sync>>,
}

impl Orchestrator {
    pub fn new(agents: Vec<Arc<dyn ReviewAgent + Send + Sync>>) -> Self {
        Self { agents }
    }

    pub async fn run(&self, input: AgentInput) -> Result<Vec<AgentResult>> {
        let mut handles: Vec<JoinHandle<Result<AgentResult>>> = Vec::new();

        for agent in &self.agents {
            let agent_clone = Arc::clone(agent);
            let input_clone = input.clone();
            let handle = tokio::spawn(async move {
                agent_clone.review(input_clone).await
            });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await? {
                Ok(result) => results.push(result),
                Err(e) => eprintln!("Error running agent: {:?}", e),
            }
        }

        Ok(results)
    }
}