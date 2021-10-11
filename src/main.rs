use clap::{App, AppSettings, Arg};

mod db;
pub use db::Database;

fn main() {
    let matches = App::new("kvdb")
        .version("0.1.0")
        .author("Rodney K. <rodneykeeling@gmail.com>")
        .setting(AppSettings::ColoredHelp)
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
        .subcommand(
            App::new("print")
                .about("print a key/value if they exist OR print the entire kvdb if no key is specified")
                .arg(Arg::new("key").index(1)),
        )
        .get_matches();

    let mut database = Database::new().expect("Creating db failed");

    match matches.subcommand() {
        Some(("insert", sub_m)) => {
            let key = sub_m.value_of("key").unwrap().to_string();
            let value = sub_m.value_of("value").unwrap().to_string();

            database.insert(key.clone(), value.clone());
            println!(
                "Insert successful for key '{}' and value '{}'.",
                print_green(key),
                print_green(value)
            );
        }
        Some(("delete", sub_m)) => {
            let key = sub_m.value_of("key").unwrap().to_string();

            database.remove(key.clone());
            println!("Delete successful for key '{}'.", print_green(key));
        }
        Some(("print", sub_m)) => {
            let key = sub_m.value_of("key");
            // print whole kvdb if no key specified
            match key {
                Some(k) => {
                    let value = database.get(k.to_string());
                    match value {
                        None => println!("No value found for key '{}'", print_green(k.to_string())),
                        Some(_) => print!("{}\t{}", k, value.unwrap()),
                    }
                }
                None => database.print(),
            }
        }
        _ => {}
    }
}

fn print_green(str: String) -> String {
    format!("\x1b[0;32m{}\x1b[0m", str)
}
