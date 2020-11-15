/*
    https://www.youtube.com/watch?v=WnWGO-tLtLA&feature=emb_logo
*/
use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was none");
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Database::new() crashed");
}

#[derive(Debug)]
struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Self, std::io::Error> {
        let mut map = HashMap::new();
        // read kv.db
        let contents = std::fs::read_to_string("kv.db")?; 
        // parse string and populate map
        for line in contents.lines() {
            let mut chunks = line.splitn(2,'\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map })
    }
}
