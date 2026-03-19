use crate::config::{load_config, save_config, Config};
use dialoguer::{Confirm, Input, Select};
use dirs;
use std::fs;
use std::path::PathBuf;

pub fn run() {
    println!("👋 Welcome to bill setup\n");

    // 🔍 Check existing config
    if let Some(existing) = load_config() {
        println!("⚠️ Storage already configured at: {}\n", existing.db_path);

        let proceed = Confirm::new()
            .with_prompt("Re-initializing may overwrite your data. Continue?")
            .default(false)
            .interact()
            .unwrap();

        if !proceed {
            println!("❌ Setup cancelled");
            return;
        }
    }

    // 📦 Storage selection
    let options = vec!["default", "manual"];

    let choice = Select::new()
        .with_prompt("Choose storage type")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let db_path = if options[choice] == "default" {
        let mut path: PathBuf = dirs::home_dir().expect("Could not find home directory");

        path.push(".bill");

        // create directory if not exists
        fs::create_dir_all(&path).expect("Failed to create .bill directory");

        path.push("bills.db");

        path.to_string_lossy().to_string()
    } else {
        Input::<String>::new()
            .with_prompt("Enter SQLite DB path")
            .interact_text()
            .unwrap()
    };

    let config = Config { db_path };

    save_config(&config);

    println!("\n✅ Setup complete!");
    println!("📁 Database location: {}", config.db_path);
    println!("👉 You can now use `bill add`\n");
}
