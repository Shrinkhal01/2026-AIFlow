use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cognitive AI roles supported in AIFlow
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Architect,
    Implementer,
    Reviewer,
    Tester,
    Approver,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Architect => write!(f, "Architect (Claude)"),
            Role::Implementer => write!(f, "Implementer (Codex)"),
            Role::Reviewer => write!(f, "Reviewer (Gemini)"),
            Role::Tester => write!(f, "Tester (Gemini)"),
            Role::Approver => write!(f, "Approver (Human)"),
        }
    }
}

/// A workflow phase in the state machine
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Phase {
    pub id: String,
    pub name: String,
    pub role: String,
    #[serde(default)]
    pub required_artifacts: Vec<String>,
    #[serde(default)]
    pub requires_human_approval: bool,
    pub next_phase: Option<String>,
}

/// The workflow definition (.aiflow/workflow.yaml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub version: String,
    pub name: String,
    pub preset: String,
    pub initial_phase: String,
    pub phases: Vec<Phase>,
}

impl WorkflowConfig {
    /// The default 5-phase workflow (Specify -> Plan -> Build -> Verify -> Ship)
    pub fn standard_5_phase() -> Self {
        Self {
            version: "1.0".to_string(),
            name: "Standard AI Workflow".to_string(),
            preset: "standard".to_string(),
            initial_phase: "specify".to_string(),
            phases: vec![
                Phase {
                    id: "specify".to_string(),
                    name: "Specify".to_string(),
                    role: "architect".to_string(),
                    required_artifacts: vec![".aiflow/spec.md".to_string()],
                    requires_human_approval: true,
                    next_phase: Some("plan".to_string()),
                },
                Phase {
                    id: "plan".to_string(),
                    name: "Plan".to_string(),
                    role: "architect".to_string(),
                    required_artifacts: vec![".aiflow/tasks.md".to_string()],
                    requires_human_approval: false,
                    next_phase: Some("build".to_string()),
                },
                Phase {
                    id: "build".to_string(),
                    name: "Build".to_string(),
                    role: "implementer".to_string(),
                    required_artifacts: vec![],
                    requires_human_approval: false,
                    next_phase: Some("verify".to_string()),
                },
                Phase {
                    id: "verify".to_string(),
                    name: "Verify".to_string(),
                    role: "tester".to_string(),
                    required_artifacts: vec![],
                    requires_human_approval: true,
                    next_phase: Some("ship".to_string()),
                },
                Phase {
                    id: "ship".to_string(),
                    name: "Ship".to_string(),
                    role: "approver".to_string(),
                    required_artifacts: vec![],
                    requires_human_approval: true,
                    next_phase: None,
                },
            ],
        }
    }

    /// The lean 4-phase workflow (Plan -> Build -> Verify -> Ship)
    pub fn lean_4_phase() -> Self {
        Self {
            version: "1.0".to_string(),
            name: "Lean AI Workflow".to_string(),
            preset: "lean".to_string(),
            initial_phase: "plan".to_string(),
            phases: vec![
                Phase {
                    id: "plan".to_string(),
                    name: "Plan".to_string(),
                    role: "architect".to_string(),
                    required_artifacts: vec![".aiflow/tasks.md".to_string()],
                    requires_human_approval: true,
                    next_phase: Some("build".to_string()),
                },
                Phase {
                    id: "build".to_string(),
                    name: "Build".to_string(),
                    role: "implementer".to_string(),
                    required_artifacts: vec![],
                    requires_human_approval: false,
                    next_phase: Some("verify".to_string()),
                },
                Phase {
                    id: "verify".to_string(),
                    name: "Verify".to_string(),
                    role: "tester".to_string(),
                    required_artifacts: vec![],
                    requires_human_approval: true,
                    next_phase: Some("ship".to_string()),
                },
                Phase {
                    id: "ship".to_string(),
                    name: "Ship".to_string(),
                    role: "approver".to_string(),
                    required_artifacts: vec![],
                    requires_human_approval: true,
                    next_phase: None,
                },
            ],
        }
    }

    /// The 4-phase Deep Learning / Machine Learning workflow:
    /// Step 1 (Spec): Use /grill-with-docs to define input/output dimensions, evaluation metrics, and dataset boundaries.
    /// Step 2 (Data Pipeline): Use /tdd to write tests for data loading, transformations, and feature extraction modules.
    /// Step 3 (Model Layers): Write isolated tensor tests for custom layers or loss functions using /tdd.
    /// Step 4 (Training Loop): Implement the training loop, and if you hit convergence or gradient bugs, run /diagnosing-bugs.
    pub fn ml_4_phase() -> Self {
        Self {
            version: "1.0".to_string(),
            name: "Machine Learning Model Workflow".to_string(),
            preset: "ml".to_string(),
            initial_phase: "spec".to_string(),
            phases: vec![
                Phase {
                    id: "spec".to_string(),
                    name: "Spec".to_string(),
                    role: "architect".to_string(),
                    required_artifacts: vec![".aiflow/spec.md".to_string()],
                    requires_human_approval: true,
                    next_phase: Some("data-pipeline".to_string()),
                },
                Phase {
                    id: "data-pipeline".to_string(),
                    name: "Data Pipeline".to_string(),
                    role: "implementer".to_string(),
                    required_artifacts: vec![],
                    requires_human_approval: false,
                    next_phase: Some("model-layers".to_string()),
                },
                Phase {
                    id: "model-layers".to_string(),
                    name: "Model Layers".to_string(),
                    role: "implementer".to_string(),
                    required_artifacts: vec![],
                    requires_human_approval: false,
                    next_phase: Some("training-loop".to_string()),
                },
                Phase {
                    id: "training-loop".to_string(),
                    name: "Training Loop".to_string(),
                    role: "implementer".to_string(),
                    required_artifacts: vec![],
                    requires_human_approval: true,
                    next_phase: None,
                },
            ],
        }
    }
}

fn default_ai_stack() -> String {
    "claude-architect".to_string()
}

/// Predefined AI Subscription Stacks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AIStackPreset {
    ClaudeArchitect,
    ClaudeCoder,
    AllClaude,
}

impl AIStackPreset {
    pub fn id(&self) -> &'static str {
        match self {
            AIStackPreset::ClaudeArchitect => "claude-architect",
            AIStackPreset::ClaudeCoder => "claude-coder",
            AIStackPreset::AllClaude => "all-claude",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AIStackPreset::ClaudeArchitect => "Claude Architect + Codex Coder",
            AIStackPreset::ClaudeCoder => "ChatGPT Planner + Claude Pro Coder",
            AIStackPreset::AllClaude => "All-Claude Stack (Claude Pro)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            AIStackPreset::ClaudeArchitect => "Claude 3.7 (Specs/Plan) → Codex/Cursor (Coding) → Gemini (Review)",
            AIStackPreset::ClaudeCoder => "ChatGPT o3/4o (Specs/Plan) → Claude Code Pro (Coding) → Gemini (Review)",
            AIStackPreset::AllClaude => "Claude 3.7 (Specs/Plan) → Claude Code Pro (Coding) → Gemini (Review)",
        }
    }

    pub fn roles(&self) -> HashMap<String, String> {
        let mut roles = HashMap::new();
        match self {
            AIStackPreset::ClaudeArchitect => {
                roles.insert("architect".to_string(), "claude-3-7-sonnet".to_string());
                roles.insert("implementer".to_string(), "codex-cursor".to_string());
                roles.insert("reviewer".to_string(), "gemini-2.5-pro".to_string());
                roles.insert("tester".to_string(), "gemini-2.5-pro".to_string());
                roles.insert("approver".to_string(), "human".to_string());
            }
            AIStackPreset::ClaudeCoder => {
                roles.insert("architect".to_string(), "chatgpt-o3".to_string());
                roles.insert("implementer".to_string(), "claude-code".to_string());
                roles.insert("reviewer".to_string(), "gemini-2.5-pro".to_string());
                roles.insert("tester".to_string(), "gemini-2.5-pro".to_string());
                roles.insert("approver".to_string(), "human".to_string());
            }
            AIStackPreset::AllClaude => {
                roles.insert("architect".to_string(), "claude-3-7-sonnet".to_string());
                roles.insert("implementer".to_string(), "claude-code".to_string());
                roles.insert("reviewer".to_string(), "gemini-2.5-pro".to_string());
                roles.insert("tester".to_string(), "gemini-2.5-pro".to_string());
                roles.insert("approver".to_string(), "human".to_string());
            }
        }
        roles
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id.to_lowercase().replace('_', "-").as_str() {
            "claude-architect" | "1" => Some(AIStackPreset::ClaudeArchitect),
            "claude-coder" | "claude-pro" | "2" => Some(AIStackPreset::ClaudeCoder),
            "all-claude" | "3" => Some(AIStackPreset::AllClaude),
            _ => None,
        }
    }

    pub fn all() -> &'static [AIStackPreset] {
        &[
            AIStackPreset::ClaudeArchitect,
            AIStackPreset::ClaudeCoder,
            AIStackPreset::AllClaude,
        ]
    }
}

/// Project identity metadata (.aiflow/project.yaml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub language: String,
    #[serde(default = "default_ai_stack")]
    pub ai_stack: String,
    #[serde(default)]
    pub roles: HashMap<String, String>,
}

impl ProjectConfig {
    pub fn new(name: &str, language: &str, stack_id: &str) -> Self {
        let preset = AIStackPreset::from_id(stack_id).unwrap_or(AIStackPreset::ClaudeArchitect);
        Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: format!("Project {} tracked by AIFlow", name),
            language: language.to_string(),
            ai_stack: preset.id().to_string(),
            roles: preset.roles(),
        }
    }

    #[allow(dead_code)]
    pub fn default_for(name: &str, language: &str) -> Self {
        Self::new(name, language, "claude-architect")
    }

    pub fn set_stack(&mut self, stack_id: &str) -> Result<AIStackPreset, String> {
        let preset = AIStackPreset::from_id(stack_id).ok_or_else(|| {
            format!(
                "Unknown stack '{}'. Available: 'claude-architect', 'claude-coder', 'all-claude'",
                stack_id
            )
        })?;
        self.ai_stack = preset.id().to_string();
        self.roles = preset.roles();
        Ok(preset)
    }
}

/// Record of an approved/completed phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedPhase {
    pub phase: String,
    pub completed_at: String,
    pub approved_by: String,
}

/// Durable project state (.aiflow/state.yaml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub version: String,
    pub current_phase: String,
    pub phase_started_at: String,
    #[serde(default)]
    pub completed_phases: Vec<CompletedPhase>,
    pub active_task_id: Option<String>,
}

impl ProjectState {
    pub fn initial(initial_phase: &str) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            version: "1.0".to_string(),
            current_phase: initial_phase.to_string(),
            phase_started_at: now,
            completed_phases: Vec::new(),
            active_task_id: None,
        }
    }

    pub fn is_phase_completed(&self, phase_id: &str) -> bool {
        self.completed_phases.iter().any(|c| c.phase == phase_id)
    }
}

/// A parsed task item from .aiflow/tasks.md
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskItem {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub is_active: bool,
}

/// Read-only Git inspection data
#[derive(Debug, Clone, Default)]
pub struct GitInfo {
    pub branch: String,
    pub modified_count: usize,
    pub untracked_count: usize,
    pub last_commit_message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_presets() {
        let presets = AIStackPreset::all();
        assert_eq!(presets.len(), 3);

        let p1 = AIStackPreset::from_id("claude-architect").unwrap();
        assert_eq!(p1, AIStackPreset::ClaudeArchitect);
        let roles1 = p1.roles();
        assert!(roles1.get("architect").unwrap().contains("claude"));
        assert!(roles1.get("implementer").unwrap().contains("codex"));

        let p2 = AIStackPreset::from_id("claude-coder").unwrap();
        assert_eq!(p2, AIStackPreset::ClaudeCoder);
        let roles2 = p2.roles();
        assert!(roles2.get("architect").unwrap().contains("chatgpt"));
        assert!(roles2.get("implementer").unwrap().contains("claude-code"));
    }

    #[test]
    fn test_project_config_stack_switch() {
        let mut cfg = ProjectConfig::new("test", "rust", "claude-architect");
        assert_eq!(cfg.ai_stack, "claude-architect");

        let res = cfg.set_stack("claude-coder");
        assert!(res.is_ok());
        assert_eq!(cfg.ai_stack, "claude-coder");
        assert!(cfg.roles.get("architect").unwrap().contains("chatgpt"));
    }

    #[test]
    fn test_ml_workflow_preset() {
        let wf = WorkflowConfig::ml_4_phase();
        assert_eq!(wf.preset, "ml");
        assert_eq!(wf.initial_phase, "spec");
        assert_eq!(wf.phases.len(), 4);
        assert_eq!(wf.phases[0].id, "spec");
        assert_eq!(wf.phases[1].id, "data-pipeline");
        assert_eq!(wf.phases[2].id, "model-layers");
        assert_eq!(wf.phases[3].id, "training-loop");
        assert!(wf.phases[0].requires_human_approval);
        assert!(wf.phases[3].requires_human_approval);
    }
}
