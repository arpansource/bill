use crate::config::load_config;
use rusqlite::Connection;

pub fn get_conn() -> Connection {
    let config = load_config().expect("Run `bill init` first");

    let conn = Connection::open(config.db_path).expect("Failed to open DB");

    init_schema(&conn);

    conn
}

fn init_schema(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS expenses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item TEXT NOT NULL,
            category_id INTEGER NOT NULL,
            amount REAL NOT NULL,
            type TEXT CHECK(type IN ('need', 'want', 'lent')) NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY(category_id) REFERENCES categories(id)
        );

        CREATE TABLE IF NOT EXISTS preferences (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            monthly_income REAL NOT NULL,
            monthly_spend_limit REAL,
            daily_spend_limit REAL
        );
        ",
    )
    .unwrap();
}
