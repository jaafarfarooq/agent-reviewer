mod agent;
mod models;
mod llm;
mod orchestrator;
mod review;

use std::sync::Arc;
use agent::{
    style::StyleAgent,
    bug::BugAgent,
    performance::PerformanceAgent,
    meta_agent::MetaReviewAgent,
};
use orchestrator::Orchestrator;
use models::AgentInput;
use llm::client::LlmClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Agent Reviewer Initialized");

    let input = AgentInput {
        diff: "diff --git a/main.rs b/main.rs".to_string(),
        pr_title: "Add feature X".to_string(),
        pr_description: "Initial implementation".to_string(),
    };

    let llm = LlmClient::new(
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")
    );

    let agents: Vec<Arc<dyn agent::r#trait::ReviewAgent + Send + Sync>> = vec![
        Arc::new(StyleAgent::new(llm.clone())),
        Arc::new(BugAgent::new(llm.clone())),
        Arc::new(PerformanceAgent::new(llm.clone())),
    ];

    let orchestrator = Orchestrator::new(agents);

    // unwrap Result with ?
    let results = orchestrator.run(input).await?;

    println!("\n=== Agent Results ===");
    for result in &results {
        println!("{:#?}", result);
    }

    let report = MetaReviewAgent::run(results);

    println!("\n=== Final Review Report ===");
    println!("{:#?}", report);

    Ok(())
}
