/// MetaReviewAgent: aggregates and synthesizes agent outputs into a final report.
///
/// Responsibilities:
/// - Combine findings from multiple agents
/// - Deduplicate and normalize findings
/// - Create a `ReviewReport` summarizing total findings and highest severity
pub struct MetaReviewAgent;

impl MetaReviewAgent {
    pub fn run(results: Vec<crate::models::AgentResult>) -> crate::review::ReviewReport {
        let combined = crate::review::combine_results(results);
        let findings = crate::review::deduplicate(combined);

        let highest = findings.iter().map(|f| f.severity).max().unwrap_or(0);

        let avg_confidence =
            if findings.is_empty() {
                1.0
            } else {
                findings.iter().map(|f| f.confidence).sum::<f32>() / findings.len() as f32
            };

        let summary = if findings.is_empty() {
            "No issues found. Code looks solid.".to_string()
        } else {
            format!(
                "{} issues found. Highest severity: {}.",
                findings.len(),
                highest
            )
        };

        crate::review::ReviewReport {
            total_findings: findings.len(),
            highest_severity: highest,
            average_confidence: avg_confidence,
            summary,
            findings,
        }
    }
}
