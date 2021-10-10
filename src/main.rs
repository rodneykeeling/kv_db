mod db;
pub use db::Database;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().unwrap();

    let mut database = Database::new().expect("Creating db failed");
    database.insert(key, value);
}
