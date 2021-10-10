use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);

    let mut database = Database::new().expect("Creating db failed");
    database.insert(key, value);
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let mut contents = String::new();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("kv.db")
            .expect("Unable to open file");
        file.read_to_string(&mut contents)?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
}

// executed after a Database instance goes out of scope but before it's removed
// from memory
impl Drop for Database {
    fn drop(&mut self) {
        // flush data to file
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        let _ = std::fs::write("kv.db", contents);
    }
}
