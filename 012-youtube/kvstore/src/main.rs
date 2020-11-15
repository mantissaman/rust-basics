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

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Self, std::io::Error> {
        // read kv.db
        // let c = match std::fs::read_to_string("kv.db"){
        //     Ok(contents) => contents,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        // Equivalent code below - bubble error
        let contents = std::fs::read_to_string("kv.db")?;
        

        // parse string

        // populate map
        Ok(Database {
            map: HashMap::new(),
        })
    }
}
