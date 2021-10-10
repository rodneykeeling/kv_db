use clap::{App, Arg};

mod db;
pub use db::Database;

fn main() {
    let matches = App::new("kvdb")
        .version("0.1.0")
        .author("Rodney K. <rodneykeeling@gmail.com>")
        .subcommand(
            App::new("insert")
                .about("Inserts a new key/value")
                .arg(Arg::new("key").index(1).required(true))
                .arg(Arg::new("value").index(2).required(true)),
        )
        .subcommand(
            App::new("delete")
                .about("Deletes a key/value if they exist")
                .arg(Arg::new("key").index(1).required(true)),
        )
        .get_matches();

    let mut database = Database::new().expect("Creating db failed");

    match matches.subcommand() {
        Some(("insert", sub_m)) => {
            let key = sub_m.value_of("key").unwrap().to_string();
            let value = sub_m.value_of("value").unwrap().to_string();

            database.insert(key, value);
        }
        Some(("delete", sub_m)) => {
            let key = sub_m.value_of("key").unwrap().to_string();

            database.remove(key);
        }
        _ => {}
    }
}
