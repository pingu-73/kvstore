use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key wasn't there");
    let value = arguments.next().unwrap();
    print!("{} {}", key, value);
    // let contents = format!("{}\t{}\n", key, value); 
    // std::fs::write("kv.db", contents).unwrap();

    let mut database = Database::new().expect("creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new () -> Result<Database, std::io::Error> {
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            // let mut chunks = line.splitn(2, '\t');
            // let key = chunks.next().expect("No Key");
            // let value = chunks.next().expect("No value");
            let (key, value) = line.split_once('\t').expect("Curropt Database");
            map.insert(key.to_owned(), value.to_owned());
        }
        // read the kv.db file 
        // parse the string
        // populate our map
        Ok(Database { map: map })
    }

    fn insert(& mut self, key: String ,value: String) {
        self.map.insert(key, value);
    }

    #[allow(dead_code)]
    fn flush(self) -> std::io::Result<()> {  // this is a type alias for Result<(), std::io::Error>
        let mut contents = String::new();
        for pairs in self.map {
            let kvpair = format!("{}\t{}\n", pairs.0, pairs.1);
            contents.push_str(&kvpair);
        }
        std::fs::write("kv.db", contents)
    }
}