use crate::models::{AgentFinding, AgentResult as ModelsAgentResult};

#[derive(Debug)]
pub struct ReviewReport {
    pub total_findings: usize,
    pub highest_severity: u8,
    pub summary: String,
    pub findings: Vec<AgentFinding>,
}

pub fn combine_results(results: Vec<ModelsAgentResult>) -> Vec<AgentFinding> {
    let mut all: Vec<AgentFinding> = results
        .into_iter()
        .flat_map(|r| r.findings)
        .collect();

    all.sort_by(|a, b| b.severity.cmp(&a.severity));
    all
}
