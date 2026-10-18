use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "aiflow",
    author = "Shrinkhal",
    version = "0.4.0",
    about = "Local developer workflow and project-tracking CLI for AI-assisted development",
    long_about = "A local-first, Git-native developer control plane that tracks repository development phases, tasks, and next actions."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize AIFlow in the current repository
    Init(InitArgs),

    /// Display current project status, phase breadcrumb, and next action
    Status(StatusArgs),

    /// Display recommended next action and assigned AI role
    Next,

    /// Scan and display multi-repository fleet dashboard
    Projects(ProjectsArgs),

    /// Manage workflow phases
    Phase(PhaseArgs),

    /// Inspect and manage project tasks
    Task(TaskArgs),

    /// Run test suite and record cached evidence
    Test(TestArgs),

    /// View or switch active AI subscription stack (Claude Architect, Claude Coder, All-Claude)
    Stack(StackArgs),

    /// Run health check verifying repository structure and state
    Doctor,

    /// Display AIFlow brand banner and terminal rendering mode
    Banner(BannerArgs),
}

#[derive(Args, Debug)]
pub struct BannerArgs {
    /// Force ASCII Fastfetch mode even in modern terminals
    #[arg(long)]
    pub ascii: bool,

    /// Force terminal image mode
    #[arg(long)]
    pub image: bool,

    /// Display first-time setup welcome card
    #[arg(long)]
    pub welcome: bool,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Workflow preset to use: 'standard' (5 phases, default), 'lean' (4 phases), or 'ml' (4 ML phases)
    #[arg(short, long, default_value = "standard")]
    pub preset: String,

    /// Project name override (defaults to current directory name)
    #[arg(short, long)]
    pub name: Option<String>,

    /// Primary programming language (defaults to auto-detected)
    #[arg(short, long)]
    pub lang: Option<String>,

    /// AI subscription stack ('claude-architect', 'claude-coder', 'all-claude')
    #[arg(short, long)]
    pub stack: Option<String>,
}

#[derive(Args, Debug)]
pub struct StatusArgs {
    /// Output raw JSON instead of formatted terminal UI
    #[arg(long)]
    pub json: bool,

    /// Display first-time setup welcome card
    #[arg(long)]
    pub welcome: bool,
}

#[derive(Args, Debug)]
pub struct ProjectsArgs {
    /// Root directory to scan (defaults to configured roots or parent)
    #[arg(short, long)]
    pub root: Option<String>,

    /// Max directory recursion depth
    #[arg(short, long, default_value_t = 3)]
    pub depth: usize,

    /// Include repositories without .aiflow initialized
    #[arg(short, long)]
    pub all: bool,

    /// Filter projects by phase (e.g. 'plan', 'build', 'verify')
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Output JSON array of project summaries
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct PhaseArgs {
    #[command(subcommand)]
    pub action: PhaseSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum PhaseSubcommands {
    /// Show current phase details
    Status,
    /// Advance to the next phase in the workflow
    Next,
    /// Mark current phase as approved and advance
    Complete,
    /// Manually jump to a specific phase ID
    Set { phase_id: String },
}

#[derive(Args, Debug)]
pub struct TaskArgs {
    #[command(subcommand)]
    pub action: Option<TaskSubcommands>,
}

#[derive(Subcommand, Debug)]
pub enum TaskSubcommands {
    /// List all tasks from .aiflow/tasks.md
    List,
    /// Mark a task as completed by ID (e.g. TASK-01)
    Done { task_id: String },
    /// Add a new task to .aiflow/tasks.md
    Add { title: String },
}

#[derive(Args, Debug)]
pub struct TestArgs {
    #[command(subcommand)]
    pub action: Option<TestSubcommands>,
}

#[derive(Subcommand, Debug)]
pub enum TestSubcommands {
    /// Run project tests and update cache
    Run,
    /// View last cached test status
    Status,
}

#[derive(Args, Debug)]
pub struct StackArgs {
    #[command(subcommand)]
    pub action: Option<StackSubcommands>,
}

#[derive(Subcommand, Debug)]
pub enum StackSubcommands {
    /// List available AI subscription stacks
    List,
    /// Switch active AI stack ('claude-architect', 'claude-coder', 'all-claude')
    Set { stack_id: String },
}
