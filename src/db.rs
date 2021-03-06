use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct Database {
    map: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Result<Database, std::io::Error> {
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

    pub fn print(&self) {
        for (key, value) in &self.map {
            println!("{}\t{}", key, value);
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        let result = self.map.get(&key);
        result.map(|result| result.to_string())
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key.to_lowercase(), value);
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key.to_lowercase());
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
