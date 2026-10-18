use crate::domain::{GitInfo, Phase, ProjectConfig, ProjectState, Role, TaskItem, WorkflowConfig};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct NextAction {
    pub summary: String,
    pub assigned_role: Role,
    pub recommended_command: String,
    pub explanation: String,
}

/// Look up a phase by ID in the workflow configuration
pub fn find_phase<'a>(workflow: &'a WorkflowConfig, phase_id: &str) -> Option<&'a Phase> {
    workflow.phases.iter().find(|p| p.id == phase_id)
}

/// Compute the next logical action based on current state, git signals, tasks, and active AI stack
pub fn compute_next_action(
    project: &ProjectConfig,
    _workflow: &WorkflowConfig,
    state: &ProjectState,
    git: &GitInfo,
    tasks: &[TaskItem],
    repo_root: &Path,
) -> NextAction {
    let current_phase = state.current_phase.as_str();

    let architect_agent = project
        .roles
        .get("architect")
        .map(|s| s.as_str())
        .unwrap_or("claude-3-7-sonnet");

    let implementer_agent = project
        .roles
        .get("implementer")
        .map(|s| s.as_str())
        .unwrap_or("codex-cursor");

    let is_chatgpt_architect = architect_agent.contains("chatgpt");
    let is_claude_coder = implementer_agent.contains("claude-code");

    match current_phase {
        // --- ML Model Workflow (4 Phases) ---
        "spec" => {
            let spec_path = repo_root.join(".aiflow/spec.md");
            if !spec_path.exists() {
                NextAction {
                    summary: "Define input/output dimensions, evaluation metrics, and dataset boundaries".to_string(),
                    assigned_role: Role::Architect,
                    recommended_command: "/grill-with-docs \"Define input/output dimensions, evaluation metrics, and dataset boundaries\"".to_string(),
                    explanation: "Use /grill-with-docs to define your input/output dimensions, evaluation metrics, and dataset boundaries in .aiflow/spec.md.".to_string(),
                }
            } else {
                NextAction {
                    summary: "Review specification drafted via /grill-with-docs".to_string(),
                    assigned_role: Role::Approver,
                    recommended_command: "aiflow phase complete".to_string(),
                    explanation: "Verify tensor dimensions, evaluation metrics, and dataset boundaries in .aiflow/spec.md. Once approved, advance to Data Pipeline.".to_string(),
                }
            }
        }
        "data-pipeline" | "data_pipeline" => {
            let pending_tasks: Vec<&TaskItem> = tasks.iter().filter(|t| !t.completed).collect();
            if let Some(first_pending) = pending_tasks.first() {
                NextAction {
                    summary: format!("Author data pipeline tests for {}: {} using /tdd", first_pending.id, first_pending.title),
                    assigned_role: Role::Implementer,
                    recommended_command: format!("/tdd \"Write tests for {}\"", first_pending.title),
                    explanation: "Use /tdd to write tests for data loading, transformations, and feature extraction modules.".to_string(),
                }
            } else if tasks.is_empty() {
                NextAction {
                    summary: "Write data pipeline tests using /tdd".to_string(),
                    assigned_role: Role::Implementer,
                    recommended_command: "/tdd \"Write tests for data loading, transformations, and feature extraction modules\"".to_string(),
                    explanation: "Use /tdd to write tests for data loading, transformations, and feature extraction modules.".to_string(),
                }
            } else {
                NextAction {
                    summary: "Data pipeline tests passing → advance to Model Layers".to_string(),
                    assigned_role: Role::Implementer,
                    recommended_command: "aiflow phase next".to_string(),
                    explanation: "Data loading, transformations, and feature extraction modules tested and verified. Ready for model layers.".to_string(),
                }
            }
        }
        "model-layers" | "model_layers" => {
            let pending_tasks: Vec<&TaskItem> = tasks.iter().filter(|t| !t.completed).collect();
            if let Some(first_pending) = pending_tasks.first() {
                NextAction {
                    summary: format!("Author isolated tensor tests for {}: {} using /tdd", first_pending.id, first_pending.title),
                    assigned_role: Role::Implementer,
                    recommended_command: format!("/tdd \"Write isolated tensor tests for {}\"", first_pending.title),
                    explanation: "Write isolated tensor tests for custom layers or loss functions using /tdd.".to_string(),
                }
            } else if tasks.is_empty() {
                NextAction {
                    summary: "Write isolated tensor tests using /tdd".to_string(),
                    assigned_role: Role::Implementer,
                    recommended_command: "/tdd \"Write isolated tensor tests for custom layers or loss functions\"".to_string(),
                    explanation: "Write isolated tensor tests for custom layers or loss functions using /tdd.".to_string(),
                }
            } else {
                NextAction {
                    summary: "Model layer tests passing → advance to Training Loop".to_string(),
                    assigned_role: Role::Implementer,
                    recommended_command: "aiflow phase next".to_string(),
                    explanation: "Custom layers and loss functions validated with isolated tensor tests. Ready to implement training loop.".to_string(),
                }
            }
        }
        "training-loop" | "training_loop" => {
            if let Some(test_res) = crate::tester::load_cached_test_result(repo_root) {
                if !test_res.passed {
                    return NextAction {
                        summary: "Diagnose gradient/convergence bug or failing test".to_string(),
                        assigned_role: Role::Implementer,
                        recommended_command: "/diagnosing-bugs \"Diagnose gradient/convergence issues in training loop\"".to_string(),
                        explanation: format!("Test run failed with exit code {}. Run /diagnosing-bugs to inspect convergence, gradient exploding/vanishing, or NaNs.", test_res.exit_code),
                    };
                }
            }
            NextAction {
                summary: "Implement training loop & diagnose convergence".to_string(),
                assigned_role: Role::Implementer,
                recommended_command: "/diagnosing-bugs \"Diagnose convergence or gradient issues in training loop\"".to_string(),
                explanation: "Implement the training loop, and if you hit convergence or gradient bugs, run /diagnosing-bugs.".to_string(),
            }
        }

        // --- Standard / Lean Workflow Phases ---
        "specify" => {
            let spec_path = repo_root.join(".aiflow/spec.md");
            if !spec_path.exists() {
                NextAction {
                    summary: "Create .aiflow/spec.md".to_string(),
                    assigned_role: Role::Architect,
                    recommended_command: "aiflow init".to_string(),
                    explanation: "Project specification document is missing.".to_string(),
                }
            } else if is_chatgpt_architect {
                NextAction {
                    summary: "Review specification drafted with ChatGPT".to_string(),
                    assigned_role: Role::Approver,
                    recommended_command: "aiflow phase complete".to_string(),
                    explanation: "Draft architecture & scope in .aiflow/spec.md with ChatGPT (o3/4o), then approve to advance.".to_string(),
                }
            } else {
                NextAction {
                    summary: "Review specification drafted with Claude".to_string(),
                    assigned_role: Role::Approver,
                    recommended_command: "aiflow phase complete".to_string(),
                    explanation: "Draft architecture & scope in .aiflow/spec.md with Claude 3.7, then approve to advance.".to_string(),
                }
            }
        }
        "plan" => {
            if tasks.is_empty() {
                if is_chatgpt_architect {
                    NextAction {
                        summary: "Generate task list in .aiflow/tasks.md using ChatGPT".to_string(),
                        assigned_role: Role::Architect,
                        recommended_command: "aiflow task add \"TASK-01: Feature scaffold\"".to_string(),
                        explanation: "Deconstruct spec.md into modular task items using ChatGPT (o3 / GPT-4o).".to_string(),
                    }
                } else {
                    NextAction {
                        summary: "Generate task list in .aiflow/tasks.md using Claude".to_string(),
                        assigned_role: Role::Architect,
                        recommended_command: "aiflow task add \"TASK-01: Feature scaffold\"".to_string(),
                        explanation: "Deconstruct spec.md into modular task items using Claude 3.7 Sonnet.".to_string(),
                    }
                }
            } else if is_claude_coder {
                NextAction {
                    summary: format!("Begin Build phase with Claude Code ({} tasks defined)", tasks.len()),
                    assigned_role: Role::Implementer,
                    recommended_command: "aiflow phase next".to_string(),
                    explanation: "Tasks are planned and ready for implementation by Claude Code in your terminal.".to_string(),
                }
            } else {
                NextAction {
                    summary: format!("Begin Build phase with Codex/Cursor ({} tasks defined)", tasks.len()),
                    assigned_role: Role::Implementer,
                    recommended_command: "aiflow phase next".to_string(),
                    explanation: "Tasks are planned and ready for implementation by Codex / Cursor IDE.".to_string(),
                }
            }
        }
        "build" => {
            let pending_tasks: Vec<&TaskItem> = tasks.iter().filter(|t| !t.completed).collect();
            if let Some(first_pending) = pending_tasks.first() {
                if is_claude_coder {
                    NextAction {
                        summary: format!("Implement {}: {} with Claude Code", first_pending.id, first_pending.title),
                        assigned_role: Role::Implementer,
                        recommended_command: format!("claude \"Implement {}\"", first_pending.title),
                        explanation: format!("Run 'claude' in terminal to author {}. Mark done with 'aiflow task done {}'.", first_pending.id, first_pending.id),
                    }
                } else {
                    NextAction {
                        summary: format!("Implement {}: {} in Cursor / Codex", first_pending.id, first_pending.title),
                        assigned_role: Role::Implementer,
                        recommended_command: format!("aiflow task done {}", first_pending.id),
                        explanation: format!("Author code in your IDE for {}. Test and mark done once verified.", first_pending.id),
                    }
                }
            } else if tasks.is_empty() {
                NextAction {
                    summary: "Add implementation tasks".to_string(),
                    assigned_role: Role::Implementer,
                    recommended_command: "aiflow task add \"Implement core feature\"".to_string(),
                    explanation: "No tasks found in .aiflow/tasks.md.".to_string(),
                }
            } else {
                NextAction {
                    summary: "All implementation tasks completed → advance to Verify".to_string(),
                    assigned_role: Role::Tester,
                    recommended_command: "aiflow phase next".to_string(),
                    explanation: "Build phase complete. Ready to run test suites and adversarial review with Gemini.".to_string(),
                }
            }
        }
        "verify" => {
            if let Some(test_res) = crate::tester::load_cached_test_result(repo_root) {
                if !test_res.passed {
                    return NextAction {
                        summary: format!("Fix failing tests ({})", test_res.command),
                        assigned_role: Role::Implementer,
                        recommended_command: "aiflow test run".to_string(),
                        explanation: format!("Test run failed with exit code {}. Resolve errors before proceeding.", test_res.exit_code),
                    };
                }
            } else if crate::tester::detect_test_runner(repo_root).is_some() {
                return NextAction {
                    summary: "Run project test suite".to_string(),
                    assigned_role: Role::Tester,
                    recommended_command: "aiflow test run".to_string(),
                    explanation: "Test runner detected. Run tests to record verification evidence.".to_string(),
                };
            }

            if git.modified_count > 0 {
                NextAction {
                    summary: "Commit or stash modified files before final review".to_string(),
                    assigned_role: Role::Reviewer,
                    recommended_command: "git status && git commit -m \"verify: pass test suite\"".to_string(),
                    explanation: format!("{} modified files detected in working tree.", git.modified_count),
                }
            } else {
                NextAction {
                    summary: "Perform adversarial review with Gemini".to_string(),
                    assigned_role: Role::Reviewer,
                    recommended_command: "aiflow phase complete".to_string(),
                    explanation: "Prompt Gemini 2.5 Pro with code diff for edge-case and security audit. Once approved, advance to Ship.".to_string(),
                }
            }
        }
        "ship" => {
            NextAction {
                summary: "Update documentation and tag release".to_string(),
                assigned_role: Role::Approver,
                recommended_command: "git tag -a v0.1.0 -m \"Release v0.1.0\"".to_string(),
                explanation: "Final human signoff: verify README, changelog, and push git tags.".to_string(),
            }
        }
        _ => NextAction {
            summary: "Check project health".to_string(),
            assigned_role: Role::Approver,
            recommended_command: "aiflow doctor".to_string(),
            explanation: format!("Unknown or custom phase: {}", current_phase),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_ml_workflow_fsm_actions() {
        let project = ProjectConfig::new("ml-model", "python", "claude-architect");
        let workflow = WorkflowConfig::ml_4_phase();
        let git = GitInfo::default();
        let empty_tasks: Vec<TaskItem> = vec![];
        let temp_dir = PathBuf::from("/tmp/nonexistent-aiflow-test-path");

        // 1. Spec phase (no spec.md)
        let state_spec = ProjectState::initial("spec");
        let action_spec = compute_next_action(&project, &workflow, &state_spec, &git, &empty_tasks, &temp_dir);
        assert!(action_spec.recommended_command.contains("/grill-with-docs"));
        assert!(action_spec.summary.contains("input/output dimensions"));

        // 2. Data pipeline phase
        let state_dp = ProjectState::initial("data-pipeline");
        let action_dp = compute_next_action(&project, &workflow, &state_dp, &git, &empty_tasks, &temp_dir);
        assert!(action_dp.recommended_command.contains("/tdd"));
        assert!(action_dp.explanation.contains("data loading"));

        // 3. Model layers phase
        let state_ml = ProjectState::initial("model-layers");
        let action_ml = compute_next_action(&project, &workflow, &state_ml, &git, &empty_tasks, &temp_dir);
        assert!(action_ml.recommended_command.contains("/tdd"));
        assert!(action_ml.explanation.contains("isolated tensor tests"));

        // 4. Training loop phase
        let state_tl = ProjectState::initial("training-loop");
        let action_tl = compute_next_action(&project, &workflow, &state_tl, &git, &empty_tasks, &temp_dir);
        assert!(action_tl.recommended_command.contains("/diagnosing-bugs"));
    }
}
