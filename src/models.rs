/// Shared data models used across the agent-reviewer crate.
///
/// Responsibilities:
/// - Define shape of inputs (`AgentInput`) passed to agents
/// - Define the agent result types (`AgentFinding`, `AgentResult`)
/// - Provide serde derives so models can be serialized/deserialized

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentInput {
    pub diff: String,          // The git diff or file content to analyze
    pub pr_title: String,      // Optional PR title
    pub pr_description: String // Optional PR description
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentFinding {
    pub category: String,      // e.g., "style", "bug", "performance"
    pub message: String,       // Human-readable feedback
    pub severity: u8           // 1-5 scale
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentResult{
    pub agent_name: String,
    pub findings: Vec<AgentFinding>
}