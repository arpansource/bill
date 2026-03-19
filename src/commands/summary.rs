use crate::db::get_conn;
use crate::utils::calculate_budget;
use chrono::{Datelike, Local};
use clap::Args;
use colored::*;

#[derive(Args)]
pub struct SummaryArgs {
    #[arg(long)]
    pub today: bool,
}

pub fn run(args: SummaryArgs) {
    let conn = get_conn();

    // Separate filters to avoid SQL ambiguity
    let (filter_plain, filter_join, label) = if args.today {
        (
            "DATE(created_at) = DATE('now', 'localtime')",
            "DATE(e.created_at) = DATE('now', 'localtime')",
            "Today's Summary",
        )
    } else {
        (
            "strftime('%Y-%m', created_at) = strftime('%Y-%m', 'now', 'localtime')",
            "strftime('%Y-%m', e.created_at) = strftime('%Y-%m', 'now', 'localtime')",
            "Monthly Summary",
        )
    };

    // ================= TOTAL =================
    let total_query = format!(
        "SELECT COALESCE(SUM(amount), 0)
         FROM expenses
         WHERE {} AND type != 'lent'",
        filter_plain
    );

    let total: f64 = conn.query_row(&total_query, [], |r| r.get(0)).unwrap();

    // ================= TREND =================
    let prev_query = if args.today {
        "SELECT COALESCE(SUM(amount), 0)
         FROM expenses
         WHERE DATE(created_at) = DATE('now', '-1 day', 'localtime')
         AND type != 'lent'"
    } else {
        "SELECT COALESCE(SUM(amount), 0)
         FROM expenses
         WHERE strftime('%Y-%m', created_at) = strftime('%Y-%m', 'now', '-1 month', 'localtime')
         AND type != 'lent'"
    };

    let prev: f64 = conn.query_row(prev_query, [], |r| r.get(0)).unwrap();

    let trend = if prev == 0.0 {
        "N/A".yellow()
    } else {
        let change = ((total - prev) / prev) * 100.0;
        if change >= 0.0 {
            format!("↑ {:.2}%", change).red()
        } else {
            format!("↓ {:.2}%", change.abs()).green()
        }
    };

    println!("\n{}", format!("========== {} ==========", label).bold());
    println!(
        "{} {}",
        "Total spent:".bold(),
        format!("₹{:.2}", total).cyan()
    );
    println!("{} {}\n", "Trend:".bold(), trend);

    // ================= TYPE BREAKDOWN =================
    println!("{}", "---- Type Breakdown ----".bold());

    let type_query = format!(
        "SELECT type, SUM(amount)
         FROM expenses
         WHERE {}
         GROUP BY type",
        filter_plain
    );

    let mut stmt = conn.prepare(&type_query).unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })
        .unwrap();

    for row in rows {
        let (t, sum) = row.unwrap();

        let colored_val = match t.as_str() {
            "need" => format!("₹{:.2}", sum).green(),
            "want" => format!("₹{:.2}", sum).red(),
            "lent" => format!("₹{:.2}", sum).yellow(),
            _ => format!("₹{:.2}", sum).normal(),
        };

        println!("{:<10} {}", t, colored_val);
    }

    // ================= CATEGORY BREAKDOWN =================
    println!("\n{}", "---- Category Breakdown ----".bold());

    let cat_query = format!(
        "SELECT c.name, SUM(e.amount)
         FROM expenses e
         JOIN categories c ON e.category_id = c.id
         WHERE {}
         GROUP BY c.name",
        filter_join
    );

    let mut stmt = conn.prepare(&cat_query).unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })
        .unwrap();

    for row in rows {
        let (cat, sum) = row.unwrap();
        println!("{:<10} {}", cat.blue(), format!("₹{:.2}", sum));
    }

    // ================= BUDGET =================
    let income: f64 = conn
        .query_row(
            "SELECT monthly_income FROM preferences WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0.0);

    if income > 0.0 {
        let budget = calculate_budget(income);

        if args.today {
            // ================= DAILY MODE =================
            let today = Local::now().day() as f64;
            let remaining_days = (budget.days_in_month as f64 - today).max(1.0);

            let need_spent: f64 = conn
                .query_row(
                    "SELECT COALESCE(SUM(amount), 0)
                 FROM expenses
                 WHERE type = 'need'
                 AND strftime('%Y-%m', created_at) = strftime('%Y-%m', 'now', 'localtime')",
                    [],
                    |r| r.get(0),
                )
                .unwrap();

            let want_spent: f64 = conn
                .query_row(
                    "SELECT COALESCE(SUM(amount), 0)
                 FROM expenses
                 WHERE type = 'want'
                 AND strftime('%Y-%m', created_at) = strftime('%Y-%m', 'now', 'localtime')",
                    [],
                    |r| r.get(0),
                )
                .unwrap();

            let remaining_need = (budget.need_monthly - need_spent).max(0.0);
            let remaining_want = (budget.want_monthly - want_spent).max(0.0);

            let safe_today = (remaining_need + remaining_want) / remaining_days;

            println!("\n{}", "---- Daily Budget ----".bold());

            println!(
                "Need: {} / {}",
                format!("₹{:.2}", need_spent).green(),
                format!("₹{:.2}", budget.need_daily).green()
            );

            println!(
                "Want: {} / {}",
                format!("₹{:.2}", want_spent).red(),
                format!("₹{:.2}", budget.want_daily).red()
            );

            println!(
                "\n{} {}",
                "Safe to spend today:".bold(),
                format!("₹{:.2}", safe_today).cyan().bold()
            );
        } else {
            // ================= MONTHLY MODE =================
            println!("\n{}", "---- Budget Overview (50-30-20) ----".bold());

            println!(
                "{} {}",
                "Days in month:".dimmed(),
                budget.days_in_month.to_string().dimmed()
            );

            println!(
                "Need  (Monthly): {} | Daily: {}",
                format!("₹{:.2}", budget.need_monthly).green(),
                format!("₹{:.2}", budget.need_daily).green()
            );

            println!(
                "Want  (Monthly): {} | Daily: {}",
                format!("₹{:.2}", budget.want_monthly).red(),
                format!("₹{:.2}", budget.want_daily).red()
            );
        }
    }

    println!();
}
