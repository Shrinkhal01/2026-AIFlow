# AIFlow Development Log & Progress Tracker

This document tracks all design decisions, toolchain configurations, milestones completed, and next actions as AIFlow is constructed.

---

## Current Status Overview
- **Active Phase:** Milestone 4 (Visual Control Plane & Terminal Branding) — **COMPLETED (v0.4.0)**
- **Active Branch:** `main`
- **Milestone 3 Release Tag:** `v0.3.0`
- **Milestone 2 Release Tag:** `v0.2.0` (on `milestone-2`)
- **Milestone 1 Release Tag:** `v0.1.0` (on `coded-project`)
- **Language / Toolchain:** Rust 1.98.1 (Apple Silicon `aarch64-apple-darwin`), Cargo, Git 2.55.0
- **Workflow Preset:** 5-Phase Standard (`Specify` → `Plan` → `Build` → `Verify` → `Ship`)
- **Codebase Size:** ~2,100 lines of clean, synchronous, warning-free Rust.

---

## Chronological Progress Log

### Step 1: Research, Product Specification & Architecture (Oct 1, 2026)
- **Competitive Analysis:** Evaluated GitHub Spec Kit (`specify-cli`), OpenSpec, Aider, Claude Code, `git-bug`, and `Taskwarrior`. Documented in [`docs/RESEARCH.md`](docs/RESEARCH.md).
- **Product Strategy:** Defined problem statement, user personas, role specializations (Claude = Architect, Codex = Implementer, Gemini = Reviewer/Tester, Human = Approver), and scope boundaries in [`docs/PRODUCT.md`](docs/PRODUCT.md).
- **Technical Requirements:** Detailed 13 primary functional and non-functional requirements, resolved state ambiguities, and documented technical risk mitigations in [`docs/REQUIREMENTS.md`](docs/REQUIREMENTS.md).
- **Architecture & Schemas:** Designed layered architecture, data models for `.aiflow/` (`project.yaml`, `workflow.yaml`, `state.yaml`, `tasks.md`, `spec.md`), and testing strategy in [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).
- **Public Vision:** Authored public [`README.md`](README.md).

### Step 2: Pragmatic Workflow Optimization & Toolchain Decision
- **Decision:** Simplified workflow from 10 enterprise phases down to the **5-Phase Standard Workflow (`Specify` → `Plan` → `Build` → `Verify` → `Ship`)** as default. Preserves full architectural specification depth without excessive process overhead.
- **Language Selection:** Reconfirmed Rust for native Apple Silicon `<10ms` execution, zero runtime dependencies, and single static binary distribution.
- **Toolchain Installation:** Installed `rustup`, `rustc 1.98.1`, and `cargo` on the macOS host; configured `~/.zshrc` to auto-load `. "$HOME/.cargo/env"`.

### Step 3: Milestone 1 MVP Core Implementation
- **Dependencies (`Cargo.toml`):** Configured minimal, proven crates (`clap` v4 derive, `serde`, `serde_yaml`, `serde_json`, `owo-colors`, `thiserror`, `chrono`).
- **Domain Models (`src/domain.rs`):** Implemented `ProjectConfig`, `WorkflowConfig`, `Phase`, `ProjectState`, `TaskItem`, `Role`, and `GitInfo`.
- **Git Inspector (`src/git.rs`):** Implemented safe, non-destructive Git status, branch detection, and commit log reader.
- **Artifact Engine (`src/storage.rs`):** Implemented `.aiflow/` template generator, YAML state persistence, and Markdown task checklist parser.
- **State Machine & Next Action Engine (`src/fsm.rs`):** Implemented dynamic next-action recommendation algorithm based on current phase and completed tasks.
- **CLI Subcommands (`src/cli.rs` & `src/main.rs`):**
  - `aiflow init`: Scaffolds `.aiflow/` with automatic language and project detection.
  - `aiflow status`: Renders rich colorized breadcrumb progress, git working tree status, task progress, and recommended next action.
  - `aiflow next`: Returns immediate next action, assigned AI agent, and recommended command.
  - `aiflow phase [next | complete | set]`: Manages phase transitions with approval records.
  - `aiflow task [list | done | add]`: Manages Markdown checklists in `.aiflow/tasks.md`.
  - `aiflow doctor`: Audits repository consistency, missing files, and git status.
- **Compilation & Verification:** Built in `1.72s` with **zero compiler warnings**. All subcommands verified against live repository.
- **Global Installation:** Installed release binary to `~/.cargo/bin/aiflow`. Configured `~/.zshrc` so `aiflow` can be executed globally from any terminal.

### Step 4: Milestone 2 Implementation (Fleet Dashboard & Testing Integration)
- **Branch:** `milestone-2` (tracked on `origin/milestone-2`).
- **Release v0.1.0 Tagged:** Officially tagged `v0.1.0` on `coded-project` branch representing Milestone 1 completion.
- **Global Configuration (`src/config.rs`):** Auto-creates and loads `~/.config/aiflow/config.yaml` with configurable `workspace_roots`, default presets, and role mappings.
- **Multi-Repository Discovery & Fleet Dashboard (`src/discovery.rs`):**
  - High-performance recursive scanner powered by `walkdir` (prunes `.git`, `node_modules`, `target`, `.cache`, `venv`).
  - Terminal table rendered via `comfy-table` with rounded UTF-8 borders, colored phase badges, git branch cleanliness, task completion counters, and next actions.
  - Subcommand `aiflow projects [--root <DIR>] [--all] [--filter <PHASE>] [--json]`.
- **Test Runner Detection & Evidence Inspection (`src/tester.rs`):**
  - Auto-detects test runners (`cargo test`, `pytest`, `npm test`, `go test`).
  - Subcommand `aiflow test [run | status]`: executes tests, records exit code, timestamp, and message into `.aiflow/.cache/test_results.json`.
  - Passively surfaced in `aiflow status` under `Test Suite Evidence:`.
- **AI Subscription Stacks (`src/domain.rs`, `src/fsm.rs`, `src/main.rs`):**
  - Implemented 3 preset subscription stacks: `claude-architect` (Claude + Codex), `claude-coder` (ChatGPT + Claude Code Pro), and `all-claude`.
  - Added interactive questionnaire during `aiflow init` to let developers choose their active subscription setup.
  - Subcommand `aiflow stack [list | set <name>]` allowing instant switching across stacks.
  - Dynamically guides the developer in `aiflow status` and `aiflow next` with role-specific commands (e.g. recommending `claude` in terminal vs IDE authoring).
- **Test Harness:** 8 automated unit tests validating discovery pruning, config serialization, runner detection, AI stacks, and current project identification.
- **Installation:** Replaced global release binary in `~/.cargo/bin/aiflow` with `v0.3.0`.

### Step 5: Milestone 4 Visual Control Plane & Terminal Branding (Oct 2026, v0.4.0)
- **App Icon & Visual Assets:**
  - Prepared optimized terminal icon asset [assets/aiflow_terminal.jpg](assets/aiflow_terminal.jpg) (256x256, 18 KB, dark slate squircle badge) embedded via `include_bytes!`.
  - Added clean transparent squircle badge [assets/aiflow_icon.png](assets/aiflow_icon.png) for web/markdown presentation.
- **Terminal Graphic Protocol Auto-Detection (`src/banner.rs`):**
  - Implemented automatic detection for iTerm2, Ghostty, Kitty, VS Code Terminal, and WezTerm.
  - Emits iTerm2 inline image protocol (`\x1b]1337;File=inline=1...`) or Kitty chunked graphics protocol.
- **High-Precision Fastfetch Fallback:**
  - Recreated the icon's exact visual geometry (terminal prompt `>_`, git pipeline branch graph, electric cyan "A" arrow, and warm orange leg) in 24-bit TrueColor Unicode art.
  - Side-by-side Neofetch-style layout with exact 25-column visual character alignment.
- **Adaptive Project Lifecycle State:**
  - **First-Time Opened:** Displays app version, overview, workspace name, detected language, AI stacks list, and getting-started guide.
  - **Working Stage:** Automatically displays real-time project working stage, active phase role, AI stack, task progress, and Git branch cleanliness.
- **CLI Subcommand & Overrides (`src/cli.rs` & `src/main.rs`):**
  - Added `aiflow banner [--ascii | --image | --welcome]` and `aiflow status --welcome`.
  - Preserves clean JSON mode (`aiflow status --json`) without ANSI interference.

### Step 6: Machine Learning & Deep Learning Preset (Oct 2026, v0.4.1)
- **Domain Modeling (`src/domain.rs`):** Added `WorkflowConfig::ml_4_phase()` defining the 4-phase model development lifecycle:
  1. `spec` (Spec): input/output tensor dimensions, evaluation metrics, dataset boundaries.
  2. `data-pipeline` (Data Pipeline): test data loading, transforms, and feature extraction.
  3. `model-layers` (Model Layers): isolated tensor tests for custom layers and loss functions.
  4. `training-loop` (Training Loop): training loop implementation and convergence/gradient bug diagnosis.
- **FSM Recommendation Engine (`src/fsm.rs`):**
  - Configured `compute_next_action` to directly surface the required AI prompt slash commands:
    - Step 1: `/grill-with-docs "Define input/output dimensions, evaluation metrics, and dataset boundaries"`
    - Step 2: `/tdd "Write tests for <task>"` (data loading & feature extraction)
    - Step 3: `/tdd "Write isolated tensor tests for <task>"` (custom layers & loss functions)
    - Step 4: `/diagnosing-bugs "Diagnose convergence or gradient issues in training loop"`
- **Artifact Scaffolding & Slash Commands (`src/storage.rs`):**
  - Generated ML-tailored `.aiflow/spec.md` with explicit sections for Tensor Shapes, Dtypes, Evaluation Metrics, and Preprocessing Bounds.
  - Generated ML-tailored `.aiflow/tasks.md` with phased TDD checklists.
  - Scaffolded `.aiflow/commands/` containing prompt instructions for `/grill-with-docs.md`, `/tdd.md`, and `/diagnosing-bugs.md`.
- **CLI & Test Suite:**
  - Added `--preset ml` to `aiflow init`.
  - Added 3 comprehensive unit tests across domain, fsm, and storage modules (11/11 tests passing).
  - Built optimized release binary and updated `~/.cargo/bin/aiflow`.

---

## File Manifest

| File | Purpose | Lines of Code |
| :--- | :--- | :---: |
| [`Cargo.toml`](Cargo.toml) | Cargo workspace manifest & dependencies (`v0.4.0`) | ~25 |
| [`src/main.rs`](src/main.rs) | CLI entry point, command dispatch, and terminal formatting | ~430 |
| [`src/banner.rs`](src/banner.rs) | Terminal graphics protocols, fastfetch art, and adaptive cards | ~340 |
| [`src/cli.rs`](src/cli.rs) | Clap command-line parser & subcommands (`projects`, `test`, `banner`) | ~170 |
| [`src/domain.rs`](src/domain.rs) | Core domain models, state structs, and role definitions | ~170 |
| [`src/discovery.rs`](src/discovery.rs) | Multi-repo scanner and `comfy-table` fleet dashboard | ~260 |
| [`src/config.rs`](src/config.rs) | Global configuration loader (`~/.config/aiflow/config.yaml`) | ~115 |
| [`src/tester.rs`](src/tester.rs) | Test runner detector, execution harness, and passive cache | ~160 |
| [`src/git.rs`](src/git.rs) | Read-only Git inspector (branch, dirty tree, commits) | ~80 |
| [`src/storage.rs`](src/storage.rs) | `.aiflow/` filesystem manager, first-open tracker & parser | ~185 |
| [`src/fsm.rs`](src/fsm.rs) | Workflow state machine & Next Action recommendation engine | ~140 |
| [`src/doctor.rs`](src/doctor.rs) | Health audit and consistency linter | ~100 |

---

## Recommended Next Steps (Milestone 5)
1. **AI Context & Prompt Scaffolding (`aiflow prompt --role <role>`):**
   - Generate structured Markdown prompt context (spec excerpt + active task + git diff) tailored for Claude Code, Codex, or Gemini.
2. **Documentation Staleness Analyzer (`aiflow docs check`):**
   - Compare commit timestamps and line changes between `src/` and `README.md`/`docs/` to warn if docs are falling behind implementation.
3. **Interactive Terminal TUI Dashboard (`ratatui`):**
   - Provide an optional real-time curses-style dashboard for flight monitoring across all projects.
