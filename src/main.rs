mod banner;
mod cli;
mod config;
mod discovery;
mod doctor;
mod domain;
mod fsm;
mod git;
mod storage;
mod tester;

use clap::Parser;
use cli::{Cli, Commands, PhaseSubcommands, StackSubcommands, TaskSubcommands, TestSubcommands};
use domain::{AIStackPreset, CompletedPhase};
use fsm::{compute_next_action, find_phase};
use git::inspect_git;
use owo_colors::OwoColorize;
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use storage::{
    add_task, init_project, is_initialized, load_project_bundle, load_tasks, mark_task_done,
    save_project_config, save_state,
};

fn main() {
    let args = Cli::parse();
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    match args.command.unwrap_or(Commands::Status(cli::StatusArgs { json: false, welcome: false })) {
        Commands::Init(init_args) => handle_init(&current_dir, &init_args),
        Commands::Status(status_args) => handle_status(&current_dir, status_args.json, status_args.welcome),
        Commands::Next => handle_next(&current_dir),
        Commands::Projects(projects_args) => handle_projects(&current_dir, &projects_args),
        Commands::Phase(phase_args) => handle_phase(&current_dir, &phase_args.action),
        Commands::Task(task_args) => handle_task(&current_dir, task_args.action),
        Commands::Test(test_args) => handle_test(&current_dir, test_args.action),
        Commands::Stack(stack_args) => handle_stack(&current_dir, stack_args.action),
        Commands::Doctor => {
            doctor::run_doctor(&current_dir);
        }
        Commands::Banner(banner_args) => handle_banner(&current_dir, &banner_args),
    }
}

/// Auto-detect project programming language from repository files
pub fn detect_language(root: &Path) -> &'static str {
    if root.join("Cargo.toml").exists() {
        "rust"
    } else if root.join("go.mod").exists() {
        "go"
    } else if root.join("package.json").exists() {
        "typescript"
    } else if root.join("pyproject.toml").exists() || root.join("requirements.txt").exists() {
        "python"
    } else {
        "generic"
    }
}

fn prompt_select_stack() -> String {
    println!("\n{}", "Select your active AI Subscription Setup:".bold());
    let presets = AIStackPreset::all();
    for (i, p) in presets.iter().enumerate() {
        let default_tag = if i == 0 { " (Default)" } else { "" };
        println!(
            "  [{}] {}{}",
            (i + 1).to_string().cyan().bold(),
            p.display_name().bold(),
            default_tag.dimmed()
        );
        println!("      Roles: {}", p.description().dimmed());
    }
    print!("\nEnter selection [1-3] (press Enter for default): ");
    let _ = std::io::stdout().flush();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        if let Some(preset) = AIStackPreset::from_id(trimmed) {
            return preset.id().to_string();
        }
    }
    "claude-architect".to_string()
}

fn handle_init(root: &Path, args: &cli::InitArgs) {
    if is_initialized(root) {
        println!("{}", "AIFlow is already initialized in this repository.".yellow());
        println!("Run '{}' to inspect project status.", "aiflow status".cyan());
        return;
    }

    let default_name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project");
    let name = args.name.as_deref().unwrap_or(default_name);
    let detected_lang = detect_language(root);
    let lang = args.lang.as_deref().unwrap_or(detected_lang);

    let stack = if let Some(ref s) = args.stack {
        s.clone()
    } else {
        prompt_select_stack()
    };

    match init_project(root, &args.preset, name, lang, &stack) {
        Ok(_) => {
            let stack_display = AIStackPreset::from_id(&stack)
                .map(|p| p.display_name().to_string())
                .unwrap_or_else(|| stack.clone());

            banner::render_banner(&banner::BannerMode::FirstTime(banner::FirstTimeInfo {
                workspace_name: name,
                language: lang,
                is_initialized: true,
                ai_stack: Some(&stack_display),
            }));

            println!("{}", "🎉 Initialized AIFlow project successfully!".green().bold());
            println!("  Preset:        {}", args.preset.magenta());
            if matches!(args.preset.as_str(), "ml" | "deep-learning" | "deeplearning") {
                println!("  Artifacts:     .aiflow/project.yaml, workflow.yaml, state.yaml, spec.md, tasks.md, commands/\n");
            } else {
                println!("  Artifacts:     .aiflow/project.yaml, workflow.yaml, state.yaml, spec.md, tasks.md\n");
            }
            println!("Next Steps:");
            println!("  1. View project status:    {}", "aiflow status".bold().cyan());
            println!("  2. See next action:        {}", "aiflow next".bold().cyan());
            println!("  3. Switch AI stack:        {}\n", "aiflow stack list".bold().cyan());
        }
        Err(e) => {
            eprintln!("{} Failed to initialize AIFlow: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}

fn handle_status(root: &Path, json: bool, welcome: bool) {
    if !is_initialized(root) {
        if json {
            eprintln!("{}", serde_json::json!({ "error": "AIFlow not initialized. Run 'aiflow init' first." }));
            std::process::exit(1);
        }

        let workspace_name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project");
        let lang = detect_language(root);

        banner::render_banner(&banner::BannerMode::FirstTime(banner::FirstTimeInfo {
            workspace_name,
            language: lang,
            is_initialized: false,
            ai_stack: None,
        }));

        println!("{}", "Getting Started:".bold());
        println!("  1. {}   Initialize .aiflow standard 5-phase preset", "aiflow init".cyan());
        println!("  2. {}   Use 4-phase lean preset (Plan->Build->Verify->Ship)", "aiflow init --preset lean".cyan());
        println!("  3. {}   Use 4-phase ML preset (Spec->Data Pipeline->Model Layers->Training Loop)", "aiflow init --preset ml".cyan());
        println!("  4. {}   Select active AI subscription stack", "aiflow stack switch".cyan());
        println!("  5. {}   Inspect fleet across workspace repositories\n", "aiflow projects".cyan());
        return;
    }

    let (project, workflow, state) = match load_project_bundle(root) {
        Ok(bundle) => bundle,
        Err(e) => {
            eprintln!("{} Failed loading project: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let git = inspect_git(root).unwrap_or_default();
    let tasks = load_tasks(root).unwrap_or_default();
    let next_action = compute_next_action(&project, &workflow, &state, &git, &tasks, root);
    let test_cached = tester::load_cached_test_result(root);
    let detected_runner = tester::detect_test_runner(root);

    if json {
        let json_output = serde_json::json!({
            "project": project,
            "workflow_preset": workflow.preset,
            "current_phase": state.current_phase,
            "git": {
                "branch": git.branch,
                "modified_files": git.modified_count,
                "untracked_files": git.untracked_count,
                "last_commit": git.last_commit_message
            },
            "tasks_count": tasks.len(),
            "tasks_completed": tasks.iter().filter(|t| t.completed).count(),
            "test_result": test_cached,
            "next_action": {
                "summary": next_action.summary,
                "role": next_action.assigned_role.to_string(),
                "command": next_action.recommended_command
            }
        });
        println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
        return;
    }

    // Determine if this is the first time the initialized project is opened
    let is_first_view = welcome || storage::is_first_open(root);

    if is_first_view {
        let stack_display = AIStackPreset::from_id(&project.ai_stack)
            .map(|p| p.display_name().to_string())
            .unwrap_or_else(|| project.ai_stack.clone());

        banner::render_banner(&banner::BannerMode::FirstTime(banner::FirstTimeInfo {
            workspace_name: &project.name,
            language: &project.language,
            is_initialized: true,
            ai_stack: Some(&stack_display),
        }));

        storage::mark_as_opened(root);
    } else {
        let active_phase = workflow.phases.iter().find(|p| p.id == state.current_phase);
        let phase_name = active_phase.map(|p| p.name.as_str()).unwrap_or(&state.current_phase);
        let phase_role = active_phase.map(|p| p.role.as_str()).unwrap_or("architect");
        let stack_display = AIStackPreset::from_id(&project.ai_stack)
            .map(|p| p.display_name().to_string())
            .unwrap_or_else(|| project.ai_stack.clone());
        let completed_count = tasks.iter().filter(|t| t.completed).count();

        banner::render_banner(&banner::BannerMode::WorkingStage(banner::WorkingStageInfo {
            project_name: &project.name,
            language: &project.language,
            phase_name,
            phase_role,
            ai_stack: &stack_display,
            tasks_completed: completed_count,
            tasks_total: tasks.len(),
            branch: &git.branch,
            modified_count: git.modified_count,
            untracked_count: git.untracked_count,
        }));
    }

    if let Some(ref commit) = git.last_commit_message {
        println!("  {} {}", "Last Commit:".dimmed(), commit.dimmed());
    }

    // Workflow Breadcrumb
    println!("\n{}", "Workflow Progress:".bold());
    for (idx, phase) in workflow.phases.iter().enumerate() {
        let num = idx + 1;
        if state.is_phase_completed(&phase.id) {
            println!("  {} {}. {:<12} (Completed)", "✓".green(), num, phase.name);
        } else if phase.id == state.current_phase {
            println!(
                "  {} {}. {:<12} [ACTIVE - {}]",
                "→".cyan().bold(),
                num,
                phase.name.bold().cyan(),
                phase.role.yellow()
            );
        } else {
            println!("  {} {}. {:<12}", "○".dimmed(), num, phase.name.dimmed());
        }
    }

    // Git Status
    println!("\n{}", "Git Status:".bold());
    println!("  Branch:       {}", git.branch.cyan());
    println!(
        "  Working Tree: {} modified, {} untracked",
        git.modified_count, git.untracked_count
    );

    // Tasks Status
    let completed_count = tasks.iter().filter(|t| t.completed).count();
    println!("\n{}", "Tasks:".bold());
    println!("  Progress:     {}/{} completed", completed_count, tasks.len());
    if let Some(active) = tasks.iter().find(|t| !t.completed) {
        println!("  Active Task:  {}: {}", active.id.cyan(), active.title);
    }

    // Test Suite Evidence
    if test_cached.is_some() || detected_runner.is_some() {
        println!("\n{}", "Test Suite Evidence:".bold());
        if let Some(res) = test_cached {
            let status_badge = if res.passed {
                "PASSED".green().bold().to_string()
            } else {
                format!("FAILED (code {})", res.exit_code).red().bold().to_string()
            };
            println!("  Runner:       {}", res.command.cyan());
            println!("  Status:       {}", status_badge);
            println!("  Executed:     {}", res.executed_at.dimmed());
        } else if let Some(runner) = detected_runner {
            println!("  Runner:       {} (detected)", runner.display_name().cyan());
            println!("  Status:       {}", "No test run recorded yet".yellow());
            println!("  Command:      {}", "aiflow test run".cyan());
        }
    }

    // Next Action
    println!("\n{}", "Next Logical Action:".bold().yellow());
    println!("  → {}", next_action.summary.bold());
    println!("  → Recommended Command: {}", next_action.recommended_command.cyan());
    println!("  → Assigned Role:       {}", next_action.assigned_role.to_string().magenta());
    println!("  → Context:             {}\n", next_action.explanation.dimmed());
}

fn handle_next(root: &Path) {
    if !is_initialized(root) {
        eprintln!("{}", "Error: AIFlow is not initialized. Run 'aiflow init'.".red());
        std::process::exit(1);
    }

    let (project, workflow, state) = match load_project_bundle(root) {
        Ok(bundle) => bundle,
        Err(e) => {
            eprintln!("{} Failed loading project: {}", "Error:".red(), e);
            std::process::exit(1);
        }
    };

    let git = inspect_git(root).unwrap_or_default();
    let tasks = load_tasks(root).unwrap_or_default();
    let next_action = compute_next_action(&project, &workflow, &state, &git, &tasks, root);

    println!("\n{}", format!("Next Action for {}:", project.name).bold());
    println!("  Phase:         {}", state.current_phase.yellow());
    println!("  Action:        {}", next_action.summary.bold());
    println!("  Command:       {}", next_action.recommended_command.cyan().bold());
    println!("  Assigned Role: {}", next_action.assigned_role.to_string().magenta());
    println!("  Why:           {}\n", next_action.explanation.dimmed());
}

fn handle_stack(root: &Path, action: Option<StackSubcommands>) {
    if !is_initialized(root) {
        eprintln!("{}", "Error: AIFlow is not initialized in this repository.".red().bold());
        eprintln!("Run '{}' to initialize.", "aiflow init".cyan());
        std::process::exit(1);
    }

    let (mut project, _, _) = match load_project_bundle(root) {
        Ok(bundle) => bundle,
        Err(e) => {
            eprintln!("{} Failed loading project: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let action = action.unwrap_or(StackSubcommands::List);

    match action {
        StackSubcommands::List => {
            println!("\n{}", "Available AI Subscription Stacks:".bold());
            let presets = AIStackPreset::all();
            for (i, p) in presets.iter().enumerate() {
                let is_active = project.ai_stack == p.id();
                let active_marker = if is_active {
                    " [ACTIVE]".green().bold().to_string()
                } else {
                    String::new()
                };
                println!(
                    "\n  [{}] {} ({}){}",
                    (i + 1).to_string().cyan().bold(),
                    p.display_name().bold(),
                    p.id().yellow(),
                    active_marker
                );
                println!("      {}", p.description().dimmed());
            }

            println!("\nCurrent Active Stack: {}", project.ai_stack.cyan().bold());
            println!("Switch stack with:    {}\n", "aiflow stack set <id>".bold().cyan());
        }
        StackSubcommands::Set { stack_id } => {
            match project.set_stack(&stack_id) {
                Ok(preset) => {
                    if let Err(e) = save_project_config(root, &project) {
                        eprintln!("Failed saving project.yaml: {}", e);
                    } else {
                        println!("\n{} Active AI stack set to: {}", "✓".green(), preset.display_name().cyan().bold());
                        println!("  Stack ID:    {}", preset.id().yellow());
                        println!("  Workflow:    {}\n", preset.description().dimmed());
                        println!("Run '{}' to see your new role-specific next actions.\n", "aiflow next".cyan());
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn handle_projects(current_dir: &Path, args: &cli::ProjectsArgs) {
    let mut roots: Vec<PathBuf> = Vec::new();

    if let Some(ref custom_root) = args.root {
        roots.push(PathBuf::from(custom_root));
    } else {
        let cfg = config::load_or_init_global_config();
        for r in cfg.workspace_roots {
            let pb = PathBuf::from(r);
            if pb.exists() {
                roots.push(pb);
            }
        }
        // Also ensure current directory or parent directory is covered
        if let Some(parent) = current_dir.parent() {
            if !roots.iter().any(|r| parent.starts_with(r)) {
                roots.push(parent.to_path_buf());
            }
        } else if !roots.iter().any(|r| current_dir.starts_with(r)) {
            roots.push(current_dir.to_path_buf());
        }
    }

    let summaries = discovery::discover_projects(&roots, args.depth);

    if args.json {
        let filtered: Vec<&discovery::ProjectSummary> = summaries
            .iter()
            .filter(|p| {
                if !args.all && !p.is_aiflow_initialized {
                    return false;
                }
                if let Some(ref filter) = args.filter {
                    if let Some(ref phase) = p.phase {
                        return phase.eq_ignore_ascii_case(filter);
                    } else {
                        return false;
                    }
                }
                true
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&filtered).unwrap());
        return;
    }

    discovery::render_projects_table(&summaries, args.all, args.filter.as_deref());
}

fn handle_phase(root: &Path, action: &PhaseSubcommands) {
    let (_, workflow, mut state) = match load_project_bundle(root) {
        Ok(bundle) => bundle,
        Err(e) => {
            eprintln!("{} Failed loading project: {}", "Error:".red(), e);
            std::process::exit(1);
        }
    };

    match action {
        PhaseSubcommands::Status => {
            println!("Current Phase: {}", state.current_phase.bold().yellow());
            if let Some(phase) = find_phase(&workflow, &state.current_phase) {
                println!("Role:          {}", phase.role);
                println!("Requires Lead: {}", phase.requires_human_approval);
                if let Some(ref next) = phase.next_phase {
                    println!("Next Phase:    {}", next);
                }
            }
        }
        PhaseSubcommands::Next | PhaseSubcommands::Complete => {
            let current = state.current_phase.clone();
            if let Some(phase) = find_phase(&workflow, &current) {
                if let Some(ref next_id) = phase.next_phase {
                    // Record completion
                    state.completed_phases.push(CompletedPhase {
                        phase: current.clone(),
                        completed_at: chrono::Utc::now().to_rfc3339(),
                        approved_by: "human".to_string(),
                    });
                    state.current_phase = next_id.clone();
                    state.phase_started_at = chrono::Utc::now().to_rfc3339();

                    if let Err(e) = save_state(root, &state) {
                        eprintln!("Failed saving state: {}", e);
                    } else {
                        println!(
                            "{} Phase '{}' completed. Advanced to '{}'.",
                            "✓".green(),
                            current.yellow(),
                            next_id.cyan().bold()
                        );
                    }
                } else {
                    println!("{} Project has reached the final release phase!", "✓".green());
                }
            }
        }
        PhaseSubcommands::Set { phase_id } => {
            if find_phase(&workflow, phase_id).is_some() {
                state.current_phase = phase_id.clone();
                if let Err(e) = save_state(root, &state) {
                    eprintln!("Failed saving state: {}", e);
                } else {
                    println!("{} Phase set to '{}'.", "✓".green(), phase_id.cyan());
                }
            } else {
                eprintln!("{} Phase '{}' not found in workflow.", "Error:".red(), phase_id);
            }
        }
    }
}

fn handle_task(root: &Path, action: Option<TaskSubcommands>) {
    let action = action.unwrap_or(TaskSubcommands::List);

    match action {
        TaskSubcommands::List => match load_tasks(root) {
            Ok(tasks) => {
                if tasks.is_empty() {
                    println!("No tasks defined in .aiflow/tasks.md.");
                    println!("Add a task with '{}'.", "aiflow task add \"Task title\"".cyan());
                } else {
                    println!("\n{}", "Project Tasks:".bold());
                    for t in tasks {
                        let marker = if t.completed {
                            "[x]".green().to_string()
                        } else {
                            "[ ]".yellow().to_string()
                        };
                        println!("  {} {}: {}", marker, t.id.cyan(), t.title);
                    }
                    println!();
                }
            }
            Err(e) => eprintln!("Error loading tasks: {}", e),
        },
        TaskSubcommands::Done { task_id } => match mark_task_done(root, &task_id) {
            Ok(true) => println!("{} Task {} marked as completed.", "✓".green(), task_id.cyan()),
            Ok(false) => println!("{} Task {} not found or already completed.", "!".yellow(), task_id),
            Err(e) => eprintln!("Error updating task: {}", e),
        },
        TaskSubcommands::Add { title } => match add_task(root, &title) {
            Ok(id) => println!("{} Added task {} in .aiflow/tasks.md", "✓".green(), id.cyan()),
            Err(e) => eprintln!("Error adding task: {}", e),
        },
    }
}

fn handle_test(root: &Path, action: Option<TestSubcommands>) {
    let action = action.unwrap_or(TestSubcommands::Run);

    match action {
        TestSubcommands::Run => {
            match tester::run_tests(root) {
                Ok(result) => {
                    println!("\n{}", "Test Execution Summary:".bold());
                    println!("  Runner:       {}", result.command.cyan());
                    let status_badge = if result.passed {
                        "PASSED".green().bold().to_string()
                    } else {
                        format!("FAILED (code {})", result.exit_code).red().bold().to_string()
                    };
                    println!("  Result:       {}", status_badge);
                    println!("  Recorded At:  {}", result.executed_at.dimmed());
                    println!("  Cache:        .aiflow/.cache/test_results.json\n");
                    if !result.passed {
                        std::process::exit(result.exit_code);
                    }
                }
                Err(e) => {
                    eprintln!("{} Failed to run tests: {}", "Error:".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
        TestSubcommands::Status => {
            if let Some(res) = tester::load_cached_test_result(root) {
                println!("\n{}", "Last Recorded Test Results:".bold());
                println!("  Runner:       {}", res.command.cyan());
                let status_badge = if res.passed {
                    "PASSED".green().bold().to_string()
                } else {
                    format!("FAILED (code {})", res.exit_code).red().bold().to_string()
                };
                println!("  Result:       {}", status_badge);
                println!("  Recorded At:  {}", res.executed_at.dimmed());
                println!("  Summary:      {}\n", res.message);
            } else if let Some(runner) = tester::detect_test_runner(root) {
                println!("\nDetected test runner: {}", runner.display_name().cyan());
                println!("No cached test run found. Run '{}' to execute.", "aiflow test run".bold().cyan());
            } else {
                println!("\nNo recognized test runner found in this project.");
            }
        }
    }
}

fn handle_banner(root: &Path, args: &cli::BannerArgs) {
    if args.ascii {
        std::env::set_var("AIFLOW_BANNER", "ascii");
    } else if args.image {
        std::env::set_var("AIFLOW_BANNER", "image");
    }

    if args.welcome || !is_initialized(root) {
        let workspace_name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project");
        let lang = detect_language(root);
        let (name, is_init, stack) = if is_initialized(root) {
            let bundle = load_project_bundle(root).ok();
            let p_name = bundle
                .as_ref()
                .map(|b| b.0.name.clone())
                .unwrap_or_else(|| workspace_name.to_string());
            let p_stack = bundle.as_ref().and_then(|b| {
                AIStackPreset::from_id(&b.0.ai_stack).map(|p| p.display_name().to_string())
            });
            (p_name, true, p_stack)
        } else {
            (workspace_name.to_string(), false, None)
        };

        banner::render_banner(&banner::BannerMode::FirstTime(banner::FirstTimeInfo {
            workspace_name: &name,
            language: lang,
            is_initialized: is_init,
            ai_stack: stack.as_deref(),
        }));
    } else if let Ok((project, workflow, state)) = load_project_bundle(root) {
        let git = inspect_git(root).unwrap_or_default();
        let tasks = load_tasks(root).unwrap_or_default();
        let active_phase = workflow.phases.iter().find(|p| p.id == state.current_phase);
        let phase_name = active_phase.map(|p| p.name.as_str()).unwrap_or(&state.current_phase);
        let phase_role = active_phase.map(|p| p.role.as_str()).unwrap_or("architect");
        let stack_display = AIStackPreset::from_id(&project.ai_stack)
            .map(|p| p.display_name().to_string())
            .unwrap_or_else(|| project.ai_stack.clone());
        let completed_count = tasks.iter().filter(|t| t.completed).count();

        banner::render_banner(&banner::BannerMode::WorkingStage(banner::WorkingStageInfo {
            project_name: &project.name,
            language: &project.language,
            phase_name,
            phase_role,
            ai_stack: &stack_display,
            tasks_completed: completed_count,
            tasks_total: tasks.len(),
            branch: &git.branch,
            modified_count: git.modified_count,
            untracked_count: git.untracked_count,
        }));
    }
}

