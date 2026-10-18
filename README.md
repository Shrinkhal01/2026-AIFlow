<p align="center">
  <img src="assets/aiflow_icon.png" alt="AIFlow Logo" width="128" height="128" />
</p>

# AIFlow

[![Version: v0.4.0 (Operational)](https://img.shields.io/badge/Version-v0.4.0-success?style=for-the-badge)](#quickstart)
[![License: MIT / Apache 2.0](https://img.shields.io/badge/License-MIT%20%2F%20Apache%202.0-blue?style=for-the-badge)](#license)
[![Target: macOS & Linux](https://img.shields.io/badge/Platform-macOS%20(Apple%20Silicon)%20%7C%20Linux-black?style=for-the-badge&logo=apple)](#technology-stack)

> **A local developer workflow and project-tracking CLI for software engineers building with AI coding agents.**

<p align="center">
  <img src="assets/aiflow-terminal-demo.png" alt="AIFlow Terminal Interface on macOS" width="850" />
</p>

---

## Quickstart

### 1. Install AIFlow
AIFlow compiles into a single, lightning-fast (<10ms) standalone binary:

```bash
# Clone and install globally
git clone https://github.com/Shrinkhal01/AIFlow.git aiflow
cd aiflow
cargo install --path .
```

### 2. Track Any Repository
Navigate to any project directory on your machine and let AIFlow guide your development:

```bash
# Step 1: Initialize AIFlow (interactively select your AI Subscription Stack)
cd ~/Projects/my-app
aiflow init

# Step 2: Check current development phase, tasks, test evidence, and Git status
aiflow status

# Step 3: View or switch active AI Subscription Stack anytime
aiflow stack
aiflow stack set claude-coder   # Switch to ChatGPT Planner + Claude Code Pro Coder

# Step 4: Run auto-detected test suite & cache pass/fail evidence
aiflow test run

# Step 5: Get immediate advice on the next action and assigned AI role
aiflow next

# Step 6: Multi-repository fleet dashboard across your whole workspace
aiflow projects
aiflow projects --all

# Step 7: Manage tasks in .aiflow/tasks.md
aiflow task list
aiflow task done TASK-01
aiflow task add "Implement auth middleware"

# Step 8: Advance phases once approved
aiflow phase complete

# Step 9: Verify repository consistency and health
aiflow doctor
```

---

## The Vision

In 2026, AI coding agents (Claude Code, OpenAI Codex, Gemini CLI, Cursor, Windsurf, Aider) make generating code faster than ever. However, managing multiple repositories across different AI tools introduces **context fragmentation and process chaos**:

- *What project was I working on before switching contexts?*
- *What development phase is this repository in?*
- *Did the agent write code before architecture was approved?*
- *Are the tests passing? Has documentation fallen out of sync?*
- *What is the next logical action, and which AI model should I prompt for it?*

**AIFlow is NOT another AI coding agent.**  
AIFlow is the **developer's flight control system**—a blazing-fast, local-first CLI that tracks Git repositories, continuously derives project phase and readiness from Git, tests, docs, and artifacts, and guides the developer step-by-step through a disciplined development workflow.

---

## Core Philosophy

### 1. The Developer is the Final Decision Maker
AI agents propose and implement; the human developer steers, approves architecture, reviews code, and holds ultimate release authority.

### 2. Cognitive Role Specialization
Rather than using a single model for every task, AIFlow structures development around specialized roles:

```
                          ┌──────────────────────────┐
                          │     Human Developer      │
                          │   Requirements & Approvals│
                          └─────────────┬────────────┘
                                        │
                         ┌──────────────┴──────────────┐
                         ▼                             ▼
              Claude (Architect)              Codex (Implementer)
              • System Design                 • Feature Implementation
              • Technical Tradeoffs           • Refactoring & Bug Fixes
              • Architecture Review           • Routine Development
                         │                             │
                         └──────────────┬──────────────┘
                                        ▼
                              Gemini (Review & Test)
                              • Adversarial Code Review
                              • Test Suite Generation
                              • Independent Verification
```

*(Note: Roles and provider bindings are fully customizable to prevent vendor lock-in.)*

### 3. Git as the Ground Truth
AIFlow is non-invasive and non-destructive. It reads Git branches (`phase/*`, `feat/*`), uncommitted changes, commit messages, and repository files to automatically infer state without locking your index or rewriting history.

### 4. Zero Cloud Overhead & Instant Startup
Built in Rust, AIFlow executes in `<15ms`, stores zero proprietary code in the cloud, sends no telemetry, and requires zero external runtimes (no Python virtual environments or Node installations needed).

---

## Pragmatic Workflow & Presets

To strike the ideal balance between deep architectural specifications and developer speed, AIFlow provides **Workflow Presets**, featuring the **5-Phase Workflow (`Specify` → `Plan` → `Build` → `Verify` → `Ship`) as the primary default**:

```
Specify ──> Plan ──> Build ──> Verify ──> Ship
```

| Phase | Scope & Objective | Primary Role & AI Agent | Human Approval Gate |
| :--- | :--- | :--- | :---: |
| **1. Specify** | Requirements, system architecture, scope & tradeoffs | **Architect (Claude + Human)** | **Required** (Spec signoff) |
| **2. Plan** | Milestone planning & granular task checklist (`tasks.md`) | **Architect (Claude)** | Optional |
| **3. Build** | Feature implementation, refactoring, code authoring | **Implementer (Codex / Cursor)** | Continuous |
| **4. Verify** | Test suite execution, adversarial review, edge-case audit | **Reviewer & Tester (Gemini)** | **Required** (Verification signoff) |
| **5. Ship** | Documentation updates, README refresh, git tagging | **Release Lead (Human)** | **Required** (Final release) |

*(Need more or fewer phases? Switch anytime with `--preset lean` for 4 general phases, or `--preset ml` for the 4-phase Deep Learning & Model Development workflow).*

#### Machine Learning & Deep Learning Preset (`aiflow init --preset ml`)
Tailored for model development, neural network authoring, and tensor validation:
- **Step 1 (Spec):** Use `/grill-with-docs` to define input/output dimensions, evaluation metrics, and dataset boundaries.
- **Step 2 (Data Pipeline):** Use `/tdd` to write tests for data loading, transformations, and feature extraction modules.
- **Step 3 (Model Layers):** Write isolated tensor tests for custom layers or loss functions using `/tdd`.
- **Step 4 (Training Loop):** Implement the training loop, and if you hit convergence or gradient bugs, run `/diagnosing-bugs`.

---

## Planned CLI Experience

### Single-Project Status (`aiflow status`)
```text
Project:      Autonomous Navigation (python)
Location:     /Users/shrinkhals/Projects/autonomous-nav
Last Active:  12 minutes ago (commit 8f3c1b2)

Workflow Progress (Preset: Standard):
  ✓ 1. Specify        (Architecture & Scope approved by Human)
  ✓ 2. Plan           (Tasks defined in tasks.md)
  → 3. Build          [ACTIVE - Codex: TASK-04]
  ○ 4. Verify         (Tests & Code Review)
  ○ 5. Ship           (Docs & Release)


Git Status:
  Branch:       phase/02-object-detection
  Working Tree: 3 modified, 1 untracked
  Upstream:     ahead 1 commit

Test Suite (pytest):
  Status:       14 passed, 2 failed
  Last Run:     45 minutes ago

Current Task:
  TASK-04: Implement object-detection visualization

Documentation:
  Status:       Synchronized (README.md updated 2 days ago)

Next Logical Action:
  → Fix 2 failing tests in test_detector.py with Codex/Gemini
  → Run: aiflow test run
  → Assigned Role: Implementer (Codex)
```

### Next Action Advice (`aiflow next`)
```text
Project: Autonomous Navigation
Current Phase: Implementation

Next Action:
  Implement object-detection visualization in src/detector.py

Recommended Steps:
  1. Generate prompt scaffold:
     $ aiflow prompt --role implementer
  2. Execute unit tests:
     $ aiflow test run
  3. Mark task complete once passing:
     $ aiflow task done TASK-04
```

### Multi-Project Dashboard (`aiflow projects`)
```text
AIFlow Multi-Project Dashboard (4 tracked repositories)

Project                 Phase             Git Status         Tests     Health     Action Needed
------------------------------------------------------------------------------------------------------
Autonomous Navigation   Implementation   phase/02-detection 14/2 fail Attention  Fix 2 failed tests
OnyxBookingSystem       Testing          feat/stripe-v2     All pass  Active     Request code review
OnyxLink                Release          main (clean)       All pass  Done       Ready to tag release
AIFlow                  Architecture     main (clean)       N/A       Active     Approve architecture.md
```

### Repository Health Check (`aiflow doctor`)
```text
AIFlow Health Check: Autonomous Navigation

  [✓] Repository structure: Valid
  [✓] Required artifacts: .aiflow/project.yaml, workflow.yaml present
  [✓] Git working tree: Non-destructive inspection clean
  [!] Test suite: 2 failures detected in test_detector.py
  [✓] Documentation freshness: In sync with recent commits

Doctor Summary: 1 issue requiring attention before advancing to Phase 8 (Code Review).
```

---

## Project Artifacts (`.aiflow/`)

Tracked repositories contain a dedicated `.aiflow/` directory:

```text
.aiflow/
├── project.yaml          # Project identity, language, test runner, role bindings
├── workflow.yaml         # Project-specific phase state machine & transition rules
├── spec.md               # Requirements, system design, architectural decisions
├── tasks.md              # Checkable task list with phase tags
├── state.yaml            # Durable phase progress & human approvals
└── .cache/               # Local test results and volatile cache (Git-ignored)
```

---

## Engineering Specifications

Detailed design documents are maintained in the [`docs/`](docs/) directory:

- [**AIFlow Case Study & Architecture (PDF)**](AIFlow_Case_Study_and_Architecture.pdf): Complete whitepaper and architectural case study covering problem statement, implementation, AI stacks, and real-world workflows.
- [**docs/CASE_STUDY.md**](docs/CASE_STUDY.md): Markdown version of the AIFlow architecture and case study.
- [**docs/PRODUCT.md**](docs/PRODUCT.md): Comprehensive product vision, personas, scope boundaries, and core concepts.
- [**docs/ARCHITECTURE.md**](docs/ARCHITECTURE.md): Technology stack justification (Rust), layered modular design, data models, state heuristics, and test harness.
- [**docs/REQUIREMENTS.md**](docs/REQUIREMENTS.md): Functional and non-functional requirements, ambiguity resolutions, and technical risks.
- [**docs/PROJECT_PLAN.md**](docs/PROJECT_PLAN.md): Minimum Viable Product (MVP) scope, phased development roadmap, work breakdown structure (WBS), and quality gates.
- [**docs/RESEARCH.md**](docs/RESEARCH.md): In-depth competitive analysis of GitHub Spec Kit, OpenSpec, AI agent workflows, and project tracking CLIs.
- [**DEV_LOG.md**](DEV_LOG.md): Living progress log tracking every milestone, decision, and file manifest.

---

## Technology Stack

- **Core Implementation:** **Rust** (2024 edition)
- **CLI Framework:** `clap` (v4 with derive macros)
- **Git Integration:** Fast, safe, non-destructive subprocess execution
- **Terminal Styling & Tables:** `comfy-table`, `owo-colors`
- **Data Serialization:** `serde`, `serde_yaml`, `serde_json`
- **Target Distribution:** Single standalone static binary for macOS (`aarch64` Apple Silicon & `x86_64`) and Linux.

---

## Development Roadmap

- [x] **Phase 0: Architecture & RFC Review** — Specifications, competitive research, and state machine design.
- [x] **Phase 1: MVP Core (v0.1.0)** — Single-repo `init`, `status`, `next`, `phase`, `task`, `doctor`, and Git state detection. *(Completed)*
- [x] **Phase 2: Multi-Project Tracking (v0.2.0)** — `aiflow projects` multi-repo dashboard and workspace discovery. *(Completed)*
- [x] **Phase 3: AI Subscription Stacks & Test Runners (v0.3.0)** — 3 preset AI stacks (`aiflow stack`) and test runner integration (`aiflow test`). *(Completed)*
- [x] **Phase 4: Visual Control Plane & Terminal Branding (v0.4.0)** — App icon assets, iTerm2/Kitty image protocols, clean Fastfetch fallback, and adaptive first-time onboarding. *(Completed)*
- [ ] **Phase 5: Agent Prompt Scaffolding (v0.5.0)** — Context prompt generator (`aiflow prompt`) and doc staleness engine.
- [ ] **Phase 6: Public Distribution (v1.0.0)** — Homebrew tap, CI/CD, and binary releases.

---

## License

Dual-licensed under
- MIT license ([LICENSE](LICENSE))
