use crate::db::get_conn;
use dialoguer::Input;

pub fn run() {
    let conn = get_conn();

    let income: f64 = Input::<f64>::new()
        .with_prompt("Enter monthly income")
        .interact_text()
        .unwrap();

    conn.execute(
        "INSERT OR REPLACE INTO preferences (id, monthly_income)
         VALUES (1, ?1)",
        [income],
    )
    .unwrap();

    println!("✅ Preferences saved");
}
