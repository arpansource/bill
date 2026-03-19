mod commands;
mod config;
mod db;
mod utils;

use clap::{Parser, Subcommand};
use commands::summary::SummaryArgs;

#[derive(Parser)]
#[command(
    name = "bill",
    about = "💸 Personal finance CLI with smart budgeting",
    version,
    after_help = "Examples:\n  bill add\n  bill summary --today\n"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'v', long = "version", action = clap::ArgAction::SetTrue)]
    version: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize storage")]
    Init,

    #[command(about = "Add new expense")]
    Add,

    #[command(about = "Manage categories")]
    Categories,

    #[command(about = "Show summary (default monthly). Use --today flag to view today's summary")]
    Summary(SummaryArgs),

    #[command(about = "Setup preferences (income, etc.)")]
    Setup,

    #[command(about = "Remove all expenses")]
    Clean,
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        println!("bill {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    match cli.command {
        Some(Commands::Init) => commands::init::run(),

        Some(Commands::Add) => commands::add::run(),

        Some(Commands::Categories) => commands::categories::run(),

        Some(Commands::Summary(args)) => commands::summary::run(args),

        Some(Commands::Setup) => commands::setup::run(),

        Some(Commands::Clean) => commands::clean::run(),

        None => {
            Cli::parse_from(["bill", "--help"]);
        }
    }
}
