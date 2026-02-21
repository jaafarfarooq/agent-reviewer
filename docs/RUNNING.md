# Agent Reviewer – Build & Execution Guide

This document covers building, running, and maintaining the Agent Reviewer CLI after the repository has already been cloned and Rust has been installed.

---

# 1. Build the Project

From the project root directory:

```bash
cargo build

```
What this does:

Resolves and downloads dependencies
Compiles the project
Generates a debug binary at:
target/debug/agent-reviewer

If the build succeeds, you will see:
Finished `dev` profile [unoptimized + debuginfo]

# 2. Run the Application

To execute the CLI tool in development mode:

```bash
cargo run
```

Or, if already built:

```bash
./target/debug/agent-reviewer
```

Expected Output Structure:

```bash
Agent Reviewer Initialized

=== Agent Results ===
...

=== Final Review Report ===
...
```

The output includes:

Individual agent findings
Deduplicated results
Severity adjustments
Confidence scoring
Summary metrics

# 3.Run in Release Mode (Optimized)

For performance-optimized execution:

```bash
cargo build --release
```

Then run:

```bash
./target/release/agent-reviewer
```

Release mode:

Enables compiler optimizations
Produces faster binaries
Disables debug symbols
Recommended for benchmarking or production demos.