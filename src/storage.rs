use crate::domain::{ProjectConfig, ProjectState, TaskItem, WorkflowConfig};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parse/serialize error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("AIFlow not initialized. Run 'aiflow init' first.")]
    NotInitialized,
}

pub fn aiflow_dir(root: &Path) -> PathBuf {
    root.join(".aiflow")
}

pub fn is_initialized(root: &Path) -> bool {
    aiflow_dir(root).join("project.yaml").exists()
}

pub fn is_first_open(root: &Path) -> bool {
    !aiflow_dir(root).join(".cache").join(".opened").exists()
}

pub fn mark_as_opened(root: &Path) {
    let dir = aiflow_dir(root).join(".cache");
    let _ = fs::create_dir_all(&dir);
    let _ = fs::write(dir.join(".opened"), "1");
}

/// Initialize the .aiflow directory with standard templates
pub fn init_project(
    root: &Path,
    preset: &str,
    project_name: &str,
    language: &str,
    stack: &str,
) -> Result<(), StorageError> {
    let dir = aiflow_dir(root);
    fs::create_dir_all(&dir)?;
    fs::create_dir_all(dir.join(".cache"))?;

    // 1. project.yaml
    let project_cfg = ProjectConfig::new(project_name, language, stack);
    let project_yaml = serde_yaml::to_string(&project_cfg)?;
    fs::write(dir.join("project.yaml"), project_yaml)?;

    // 2. workflow.yaml
    let workflow_cfg = match preset {
        "lean" => WorkflowConfig::lean_4_phase(),
        "ml" | "deep-learning" | "deeplearning" => WorkflowConfig::ml_4_phase(),
        _ => WorkflowConfig::standard_5_phase(),
    };
    let workflow_yaml = serde_yaml::to_string(&workflow_cfg)?;
    fs::write(dir.join("workflow.yaml"), workflow_yaml)?;

    // 3. state.yaml
    let state = ProjectState::initial(&workflow_cfg.initial_phase);
    let state_yaml = serde_yaml::to_string(&state)?;
    fs::write(dir.join("state.yaml"), state_yaml)?;

    let is_ml = matches!(preset, "ml" | "deep-learning" | "deeplearning");

    // 4. spec.md (Architecture & Requirements template)
    let spec_path = dir.join("spec.md");
    if !spec_path.exists() {
        let spec_content = if is_ml {
            format!(
                "# ML Model Specification: {}\n\n\
                ## 1. Problem Statement & Scope\n\
                <!-- Defined by Human Developer -->\n\
                Describe the model objective, baseline performance targets, and deployment constraints.\n\n\
                ## 2. Input / Output Tensor Dimensions & Data Types\n\
                <!-- Drafted using /grill-with-docs -->\n\
                - Input Tensor Shape: e.g. [batch_size, sequence_length, embedding_dim] or [batch_size, channels, height, width]\n\
                - Target / Output Shape: e.g. [batch_size, num_classes]\n\
                - Data Type & Precision: float32 / bfloat16 / float16\n\n\
                ## 3. Evaluation Metrics & Success Criteria\n\
                - Primary Metric (e.g. Validation Loss, Accuracy, F1-Score, Perplexity):\n\
                - Target Threshold to pass:\n\n\
                ## 4. Dataset Boundaries & Preprocessing Limits\n\
                - Dataset Sources & Splits: Train / Validation / Test\n\
                - Preprocessing, Transformations & Augmentations:\n\
                - Out-of-Distribution Handling & Clipping Limits:\n\
                - Boundary Conditions (NaN/Inf handling, zero-division guards):\n\n\
                ## 5. Architecture & Loss Formulation\n\
                - Custom Layers & Components:\n\
                - Loss Formulation & Regularization:\n\n\
                ## 6. Human Approval\n\
                - [ ] Specification, tensor dimensions, and metrics approved by lead developer\n",
                project_name
            )
        } else {
            format!(
                "# Specification: {}\n\n\
                ## 1. Problem Statement & Scope\n\
                <!-- Defined by Human Developer -->\n\
                Describe what problem this project solves and the core requirements.\n\n\
                ## 2. Technical Architecture & Decisions\n\
                <!-- Authored by Claude (Architect) -->\n\
                Describe the component layout, technology choices, and data flow.\n\n\
                ## 3. Human Approval\n\
                - [ ] Architecture approved by lead developer\n",
                project_name
            )
        };
        fs::write(spec_path, spec_content)?;
    }

    // 5. tasks.md template
    let tasks_path = dir.join("tasks.md");
    if !tasks_path.exists() {
        let tasks_content = if is_ml {
            format!(
                "# Project Tasks: {}\n\n\
                ## Phase: Data Pipeline\n\
                - [ ] TASK-01: /tdd for dataset loading, batch collation, and transforms\n\
                - [ ] TASK-02: /tdd for input transformations, normalization, and feature extraction modules\n\n\
                ## Phase: Model Layers\n\
                - [ ] TASK-03: /tdd isolated tensor tests for custom layers (forward pass & shape assertions)\n\
                - [ ] TASK-04: /tdd unit tests for custom loss functions and gradient propagation\n\n\
                ## Phase: Training Loop\n\
                - [ ] TASK-05: Implement training loop with step logging, validation loop, and checkpointing\n\
                - [ ] TASK-06: Verify convergence and run /diagnosing-bugs if gradients explode/vanish\n",
                project_name
            )
        } else {
            format!(
                "# Project Tasks: {}\n\n\
                ## Phase: Build\n\
                - [ ] TASK-01: Set up core project structure and entry point\n\
                - [ ] TASK-02: Implement primary domain logic and tests\n",
                project_name
            )
        };
        fs::write(tasks_path, tasks_content)?;
    }

    // 6. Scaffold AI agent command templates (.aiflow/commands/) for ML preset
    if is_ml {
        let cmd_dir = dir.join("commands");
        let _ = fs::create_dir_all(&cmd_dir);

        let grill_cmd = "# /grill-with-docs Command\n\n\
            **Purpose:** Interrogate ML model specifications, dimensions, metrics, and dataset boundaries.\n\n\
            **Prompt for AI Agent:**\n\
            ```markdown\n\
            Act as an adversarial Senior ML Systems Architect. Before writing any code:\n\
            1. Cross-reference the framework documentation (PyTorch, JAX, Hugging Face, etc.).\n\
            2. Interrogate the user to explicitly define input/output tensor shapes across batch dimensions.\n\
            3. Pin down exact evaluation metrics (validation loss, F1, accuracy, throughput).\n\
            4. Establish dataset boundaries: missing values, normalization bounds, data augmentation ranges, and NaN/Inf guards.\n\
            5. Document the verified specifications into .aiflow/spec.md.\n\
            ```\n";
        let _ = fs::write(cmd_dir.join("grill-with-docs.md"), grill_cmd);

        let tdd_cmd = "# /tdd Command\n\n\
            **Purpose:** Test-Driven Development for data pipelines, transforms, and tensor operations.\n\n\
            **Prompt for AI Agent:**\n\
            ```markdown\n\
            Enforce strict Test-Driven Development (Red -> Green -> Refactor):\n\
            1. Write isolated unit tests first (e.g. pytest tests/test_dataloader.py or test_layers.py).\n\
            2. For data pipeline: test data loading, shape preservation, normalization bounds, and batch collation.\n\
            3. For model layers: test isolated forward pass, batch invariance, output tensor shapes, and loss gradient propagation.\n\
            4. Run the test suite via `aiflow test run` to observe failure, then implement minimal code to pass.\n\
            ```\n";
        let _ = fs::write(cmd_dir.join("tdd.md"), tdd_cmd);

        let bug_cmd = "# /diagnosing-bugs Command\n\n\
            **Purpose:** Diagnose training loop convergence, gradient anomalies, and tensor bugs.\n\n\
            **Prompt for AI Agent:**\n\
            ```markdown\n\
            Diagnose convergence and numerical stability issues in the training loop:\n\
            1. Check gradient norms: identify exploding (NaN/Inf) or vanishing gradients across layers.\n\
            2. Check activations: identify dead ReLUs or saturated sigmoids/tanh.\n\
            3. Check loss curve: diagnose divergence, loss plateaus, or learning rate schedule mismatch.\n\
            4. Inspect data pipeline: check for data leakage between train/val, silent broadcasting errors, or unnormalized inputs.\n\
            5. Verify model train/eval mode toggles (e.g. model.train() vs model.eval() with BatchNorm/Dropout).\n\
            ```\n";
        let _ = fs::write(cmd_dir.join("diagnosing-bugs.md"), bug_cmd);
    }

    // 6. Ensure .aiflow/.cache/ is ignored by git
    let gitignore_path = root.join(".gitignore");
    let mut gitignore_content = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };

    if !gitignore_content.contains(".aiflow/.cache/") {
        if !gitignore_content.is_empty() && !gitignore_content.ends_with('\n') {
            gitignore_content.push('\n');
        }
        gitignore_content.push_str("\n# AIFlow volatile cache\n.aiflow/.cache/\n");
        fs::write(&gitignore_path, gitignore_content)?;
    }

    Ok(())
}

/// Load project, workflow, and state files
pub fn load_project_bundle(
    root: &Path,
) -> Result<(ProjectConfig, WorkflowConfig, ProjectState), StorageError> {
    let dir = aiflow_dir(root);
    if !dir.join("project.yaml").exists() {
        return Err(StorageError::NotInitialized);
    }

    let project_yaml = fs::read_to_string(dir.join("project.yaml"))?;
    let project_cfg: ProjectConfig = serde_yaml::from_str(&project_yaml)?;

    let workflow_yaml = fs::read_to_string(dir.join("workflow.yaml"))?;
    let workflow_cfg: WorkflowConfig = serde_yaml::from_str(&workflow_yaml)?;

    let state_yaml = fs::read_to_string(dir.join("state.yaml"))?;
    let state: ProjectState = serde_yaml::from_str(&state_yaml)?;

    Ok((project_cfg, workflow_cfg, state))
}

/// Save state.yaml
pub fn save_state(root: &Path, state: &ProjectState) -> Result<(), StorageError> {
    let dir = aiflow_dir(root);
    let state_yaml = serde_yaml::to_string(state)?;
    fs::write(dir.join("state.yaml"), state_yaml)?;
    Ok(())
}

/// Save project.yaml
pub fn save_project_config(root: &Path, config: &ProjectConfig) -> Result<(), StorageError> {
    let dir = aiflow_dir(root);
    let project_yaml = serde_yaml::to_string(config)?;
    fs::write(dir.join("project.yaml"), project_yaml)?;
    Ok(())
}

/// Parse task items from .aiflow/tasks.md
pub fn load_tasks(root: &Path) -> Result<Vec<TaskItem>, StorageError> {
    let tasks_path = aiflow_dir(root).join("tasks.md");
    if !tasks_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(tasks_path)?;
    let mut tasks = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [") {
            let completed = trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]");
            let remainder = trimmed.trim_start_matches("- [ ] ").trim_start_matches("- [x] ").trim_start_matches("- [X] ");
            let is_active = remainder.to_lowercase().contains("(active)");

            // Try to split on ":" to get TASK-ID
            let parts: Vec<&str> = remainder.splitn(2, ':').collect();
            let (id, title) = if parts.len() == 2 {
                (parts[0].trim().to_string(), parts[1].trim().to_string())
            } else {
                (format!("TASK-{:02}", tasks.len() + 1), remainder.to_string())
            };

            tasks.push(TaskItem {
                id,
                title,
                completed,
                is_active,
            });
        }
    }

    Ok(tasks)
}

/// Mark a specific task as done in .aiflow/tasks.md
pub fn mark_task_done(root: &Path, task_id: &str) -> Result<bool, StorageError> {
    let tasks_path = aiflow_dir(root).join("tasks.md");
    if !tasks_path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(&tasks_path)?;
    let mut updated_lines = Vec::new();
    let mut found = false;

    for line in content.lines() {
        if line.contains(task_id) && line.trim().starts_with("- [ ]") {
            let replaced = line.replace("- [ ]", "- [x]");
            // Remove (Active) tag if present
            let cleaned = replaced.replace("(Active)", "").replace("(active)", "");
            updated_lines.push(cleaned.trim_end().to_string());
            found = true;
        } else {
            updated_lines.push(line.to_string());
        }
    }

    if found {
        fs::write(tasks_path, updated_lines.join("\n") + "\n")?;
    }

    Ok(found)
}

/// Add a new task to .aiflow/tasks.md
pub fn add_task(root: &Path, title: &str) -> Result<String, StorageError> {
    let tasks_path = aiflow_dir(root).join("tasks.md");
    let current_tasks = load_tasks(root)?;
    let next_id = format!("TASK-{:02}", current_tasks.len() + 1);

    let line = format!("- [ ] {}: {}\n", next_id, title);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&tasks_path)?;
    file.write_all(line.as_bytes())?;

    Ok(next_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_ml_preset() {
        let temp_dir = std::env::temp_dir().join(format!(
            "aiflow_test_ml_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&temp_dir);

        let res = init_project(&temp_dir, "ml", "ml-model", "python", "claude-architect");
        assert!(res.is_ok());

        let aiflow = temp_dir.join(".aiflow");
        assert!(aiflow.join("workflow.yaml").exists());
        assert!(aiflow.join("spec.md").exists());
        assert!(aiflow.join("tasks.md").exists());
        assert!(aiflow.join("commands/grill-with-docs.md").exists());
        assert!(aiflow.join("commands/tdd.md").exists());
        assert!(aiflow.join("commands/diagnosing-bugs.md").exists());

        let spec = fs::read_to_string(aiflow.join("spec.md")).unwrap();
        assert!(spec.contains("Tensor Dimensions"));
        assert!(spec.contains("Evaluation Metrics"));
        assert!(spec.contains("Dataset Boundaries"));

        let tasks = fs::read_to_string(aiflow.join("tasks.md")).unwrap();
        assert!(tasks.contains("Data Pipeline"));
        assert!(tasks.contains("Model Layers"));
        assert!(tasks.contains("Training Loop"));

        let (proj, wf, state) = load_project_bundle(&temp_dir).unwrap();
        assert_eq!(proj.name, "ml-model");
        assert_eq!(wf.preset, "ml");
        assert_eq!(state.current_phase, "spec");
        assert_eq!(wf.phases.len(), 4);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
