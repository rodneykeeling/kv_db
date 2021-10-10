use clap::{Arg, App};

mod db;
pub use db::Database;

fn main() {
    let matches = App::new("kvdb")
        .version("0.1.0")
        .author("Rodney K. <rodneykeeling@gmail.com>")
        .subcommand(App::new("insert")
            .about("Inserts a new key/value")
            .arg(Arg::new("key")
                .short('k')
                .long("key")
                .takes_value(true))
            .arg(Arg::new("value")
                .short('v')
                .long("value")
                .takes_value(true)))
        .get_matches();

    let mut database = Database::new().expect("Creating db failed");

    if let Some(matches) = matches.subcommand_matches("insert") {
        let key = matches.value_of("key").unwrap().to_string();
        let value = matches.value_of("value").unwrap().to_string();

        database.insert(key, value);
    }
}
