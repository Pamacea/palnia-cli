mod client;
mod commands;
mod config;
mod types;

use clap::{Parser, Subcommand};
use commands::{auth, calendar, events, habits, images, init, tasks};

#[derive(Parser)]
#[command(name = "palnia", version, about = "CLI for Palnia productivity app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with a Palnia API token
    Login {
        /// API URL override
        #[arg(long)]
        url: Option<String>,
    },
    /// Remove stored credentials
    Logout,
    /// Show current authenticated user
    Whoami,
    /// Manage tasks
    Tasks {
        #[command(subcommand)]
        action: Option<tasks::TaskAction>,
    },
    /// Manage events
    Events {
        #[command(subcommand)]
        action: Option<events::EventAction>,
    },
    /// Manage habits
    Habits {
        #[command(subcommand)]
        action: Option<habits::HabitAction>,
    },
    /// Show today's agenda (events + tasks)
    Agenda {
        #[command(subcommand)]
        action: Option<calendar::AgendaAction>,
    },
    /// Manage images
    Images {
        #[command(subcommand)]
        action: Option<images::ImageAction>,
    },
    /// Initialize Palnia integrations
    Init {
        /// Generate .claude/PALNIA.md and add @PALNIA.md to CLAUDE.md
        #[arg(long = "claude-code")]
        claude_code: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Login { url } => auth::login(url).await,
        Commands::Logout => auth::logout(),
        Commands::Whoami => auth::whoami().await,
        Commands::Tasks { action } => tasks::run(action).await,
        Commands::Events { action } => events::run(action).await,
        Commands::Habits { action } => habits::run(action).await,
        Commands::Agenda { action } => calendar::run(action).await,
        Commands::Images { action } => images::run(action).await,
        Commands::Init { claude_code } => {
            if claude_code {
                init::claude_code()
            } else {
                Err(anyhow::anyhow!("Spécifiez une option : --claude-code"))
            }
        }
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
