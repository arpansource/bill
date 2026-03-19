use crate::db::get_conn;
use chrono::{Local, NaiveDate};
use dialoguer::{Confirm, Input, Select};
use rusqlite::params;

pub fn run() {
    let conn = get_conn();

    // CATEGORY SELECTION
    let mut stmt = conn.prepare("SELECT id, name FROM categories").unwrap();
    let categories: Vec<(i32, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    if categories.is_empty() {
        println!("⚠️ No categories found. Run `bill categories` first.");
        return;
    }

    let names: Vec<String> = categories.iter().map(|c| c.1.clone()).collect();

    let selected = Select::new()
        .with_prompt("Select category")
        .items(&names)
        .interact()
        .unwrap();

    let category_id = categories[selected].0;

    // INPUTS
    let item: String = Input::<String>::new()
        .with_prompt("Item")
        .interact_text()
        .unwrap();

    let amount: f64 = Input::<f64>::new()
        .with_prompt("Amount")
        .interact_text()
        .unwrap();

    let types = vec!["need", "want", "lent"];

    let t = Select::new()
        .with_prompt("Type")
        .items(&types)
        .interact()
        .unwrap();

    let expense_type = types[t];

    // DATE HANDLING
    let use_today = Confirm::new()
        .with_prompt("Use today's date?")
        .default(true)
        .interact()
        .unwrap();

    let datetime = if use_today {
        Local::now()
    } else {
        let input: String = Input::<String>::new()
            .with_prompt("Enter date (dd-mm-yyyy)")
            .validate_with(|input: &String| -> Result<(), &str> {
                NaiveDate::parse_from_str(input, "%d-%m-%Y")
                    .map(|_| ())
                    .map_err(|_| "Invalid format")
            })
            .interact_text()
            .unwrap();

        let date = NaiveDate::parse_from_str(&input, "%d-%m-%Y").unwrap();

        date.and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
    };

    let now = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    // INSERT
    conn.execute(
        "INSERT INTO expenses (item, category_id, amount, type, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![item, category_id, amount, expense_type, now, now],
    )
    .unwrap();

    println!("✅ Expense added");

    // ================= BUDGET WARNING =================
    let income: f64 = conn
        .query_row(
            "SELECT monthly_income FROM preferences WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0.0);

    if income > 0.0 {
        let need_daily = (income * 0.5) / 30.0;
        let want_daily = (income * 0.3) / 30.0;

        if expense_type == "want" && amount > want_daily {
            println!("⚠️ This exceeds your daily WANT budget!");
        }

        if expense_type == "need" && amount > need_daily {
            println!("⚠️ This exceeds your daily NEED budget!");
        }
    }
}
