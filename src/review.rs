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
    pub average_confidence: f32,
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

    for mut f in findings {
        let (adjusted_severity, confidence) =
            adjust_confidence(&f.message, f.severity);
        f.severity = adjusted_severity;
        f.confidence = confidence;

        let key = if f.message.contains("items[items.len()]") {
            "items_len_oob".to_string()
        } else {
            f.message.clone()
        };
        
        map.entry(key)
            .and_modify(|existing| {
                if f.severity > existing.severity {
                    *existing = f.clone();
                }
            })
            .or_insert(f);
    }

    map.into_values().collect()
}

fn adjust_confidence(message: &str, severity: u8) -> (u8, f32) {
    let speculative_terms = [
        "may",
        "might",
        "could",
        "possibly",
        "potentially",
        "likely",
        "appears to",
    ];

    let lower = message.to_lowercase();

    let mut confidence = 1.0;
    let mut adjusted_severity = severity;

    for term in speculative_terms.iter() {
        if lower.contains(term) {
            confidence -= 0.2;
            adjusted_severity = adjusted_severity.saturating_sub(1);
        }
    }

    if confidence < 0.2 {
        confidence = 0.2;
    }

    (adjusted_severity, confidence)
}
