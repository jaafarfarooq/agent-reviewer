# Agent Reviewer (v0.1)

An AI-powered, multi-agent code review engine built in Rust.

Agent Reviewer analyzes real Git diffs using multiple specialized LLM-backed agents, aggregates findings deterministically, and produces structured review reports with severity prioritization.

---

## 🚀 Overview

Manual code review is inconsistent and time-consuming.  
Large Language Models (LLMs) provide useful insights but lack structure and determinism.

Agent Reviewer solves this by:

- Running multiple specialized review agents
- Enforcing strict JSON output schemas
- Aggregating results deterministically
- Deduplicating overlapping findings
- Prioritizing by severity

This creates a structured and production-ready AI review pipeline.

---

## 🧠 Architecture

CLI Input (git diff or file)
        ↓
Agent Orchestrator
        ↓
Parallel LLM-backed Agents:
  - StyleAgent
  - BugAgent
  - PerformanceAgent
        ↓
Deduplication + Severity Prioritization
        ↓
Final Structured ReviewReport

---

## ⚙️ Current Features (v0.1)

- Multi-agent architecture
- Structured JSON-only LLM outputs
- Deterministic aggregation
- Deduplication logic
- Severity prioritization
- CLI-based Git diff ingestion
- Real repository self-review

---

## 🛠 Usage

### Review current git changes

```bash
cargo run -- --git
