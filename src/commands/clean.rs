use crate::db::get_conn;

pub fn run() {
    let conn = get_conn();

    conn.execute("DELETE FROM expenses", []).unwrap();

    println!("🧹 All expenses cleared (categories & limits preserved)");
}
