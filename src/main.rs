mod commands;
mod config;
mod db;
mod utils;

use clap::{Parser, Subcommand};
use commands::summary::SummaryArgs;

#[derive(Parser)]
#[command(name = "bill")]
#[command(about = "💸 Your personal finance CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add,
    Categories,
    Summary(SummaryArgs),
    Setup,
    Clean,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => commands::init::run(),

        Some(Commands::Add) => commands::add::run(),

        Some(Commands::Categories) => commands::categories::run(),

        Some(Commands::Summary(args)) => commands::summary::run(args),

        Some(Commands::Setup) => commands::setup::run(),

        Some(Commands::Clean) => commands::clean::run(),

        None => {
            println!("💸 bill - Personal Finance CLI\n");
            println!("Run `bill init` to get started.\n");

            println!("Available commands:");
            println!("  init            Initialize storage");
            println!("  add             Add new expense");
            println!("  categories      Manage categories");
            println!("  summary         Monthly summary (default)");
            println!("  summary --today Today's summary");
            println!("  set-warning     Set daily/monthly limits");
            println!("  clean           Remove all expenses");
        }
    }
}
