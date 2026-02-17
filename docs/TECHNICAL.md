# Technical Documentation

## System Components

### 1. AgentInput

Encapsulates the input context provided to all agents.

Fields:
- diff: Git diff content
- pr_title: PR title (currently placeholder)
- pr_description: PR description (currently placeholder)

---

### 2. Agents

Each agent implements independent analysis logic using LLM prompts.

Current agents:
- StyleAgent
- BugAgent
- PerformanceAgent

Each agent:
- Generates a structured prompt
- Calls the LLM client
- Parses strict JSON
- Returns AgentResult

---

### 3. LLM Client

Responsible for:
- Making HTTP requests to the LLM provider
- Enforcing JSON schema compliance
- Handling invalid responses

Future improvements:
- Retry logic
- Rate limiting
- Streaming responses

---

### 4. Aggregation Layer

Responsibilities:
- Combine findings from all agents
- Deduplicate overlapping issues
- Preserve highest severity finding
- Compute summary statistics

This layer ensures deterministic behavior independent of LLM output quality.

---

### 5. ReviewReport

Final structured output:

- total_findings
- highest_severity
- summary
- findings[]

This can later be serialized to JSON for CI integration.

---

## Execution Flow

1. CLI parses arguments
2. Git diff is loaded
3. Agents execute sequentially (async-ready)
4. Results are aggregated
5. Final report is printed
