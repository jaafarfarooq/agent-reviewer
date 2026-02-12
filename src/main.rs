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

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "AI Code Review Agent")]
struct Args {
    /// Use current git diff
    #[arg(long)]
    git: bool,

    /// Provide diff file manually
    #[arg(long)]
    file: Option<String>,
}

/// Entry point for the Agent Reviewer CLI.
/// 
/// Responsibilities:
/// - Parse CLI arguments
/// - Load git diff or file input
/// - Initialize agent input
/// - Execute agents
/// - Aggregate results
/// - Print final review report
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Agent Reviewer Initialized");

    let args = Args::parse();

    let diff = if args.git {
        std::process::Command::new("git")
            .args(["diff"])
            .output()
            .expect("Failed to execute git diff")
            .stdout
    } else if let Some(file_path) = args.file {
        std::fs::read(file_path).expect("Failed to read diff file")
    } else {
        panic!("Provide either --git or --file <path>");
    };

    let diff_string = String::from_utf8_lossy(&diff).to_string();

    let input = AgentInput {
        diff: diff_string,
        pr_title: "CLI Review".to_string(),
        pr_description: "Generated from CLI input".to_string(),
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
