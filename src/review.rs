/// Review utilities and report generation.
///
/// Responsibilities:
/// - Combine individual `AgentResult`s into a flat list of findings
/// - Deduplicate and normalize findings across agents
/// - Produce an internal `ReviewReport` summarizing findings
use crate::models::{AgentFinding, AgentResult as ModelsAgentResult};
use std::collections::HashMap;

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

pub fn deduplicate(findings: Vec<AgentFinding>) -> Vec<AgentFinding> {
    let mut map: HashMap<String, AgentFinding> = HashMap::new();

    for f in findings {
        let key = if f.message.contains("items[items.len()]") {
            "items_len_oob".to_string()
        } else {
            f.message.clone()
        };
        map.entry(key)
            .and_modify(|existing| {
                existing.severity = existing.severity.max(f.severity);
            })
            .or_insert(f);
    }

    map.into_values().collect()
}

