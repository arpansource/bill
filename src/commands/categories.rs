use crate::db::get_conn;
use chrono::Local;
use dialoguer::Input;

pub fn run() {
    let conn = get_conn();

    let name: String = Input::new()
        .with_prompt("New category")
        .interact_text()
        .unwrap();

    let now = Local::now().to_string();

    conn.execute(
        "INSERT INTO categories (name, created_at) VALUES (?1, ?2)",
        [&name, &now],
    )
    .unwrap();

    println!("✅ Category added");
}
