use anyhow::Result;
use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io;

// Storage: JSON | Memory
trait Storage {
    fn write(&mut self, value: Value) -> Result<usize>;

    fn read(&self, value: Value) -> Result<Vec<Value>>;
}

#[derive(Default)]
struct MemoryStorage {
    memdb: Vec<Value>,
}

#[derive(Deserialize)]
struct FindAll {
    all: bool,
}

#[derive(Deserialize, Debug)]
struct Args {
    #[serde(deserialize_with = "deserialize_to_hashmap")]
    map_args: HashMap<String, Value>,
}

fn deserialize_to_hashmap<'de, D>(deserializer: D) -> Result<HashMap<String, Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let m = HashMap::<String, Value>::deserialize(deserializer)?;
    Ok(m)
}

impl Storage for MemoryStorage {
    fn write(&mut self, value: Value) -> Result<usize> {
        self.memdb.push(value);
        Ok(self.memdb.len() - 1)
    }

    fn read(&self, value: Value) -> Result<Vec<Value>> {
        let find_all: FindAll = match serde_json::from_value(value.clone()) {
            Ok(r) => r,
            Err(_) => FindAll { all: false },
        };

        if find_all.all {
            return Ok(self.memdb.clone());
        };

        let query: HashMap<String, Value> = serde_json::from_value(value)?;
        for (key, value) in &query {
            println!("Key: {:?}\nValue: {:?}", key, value)
        }

        Ok(vec![])
    }
}

struct ZenoOptions {
    table: String,
}

impl Default for ZenoOptions {
    fn default() -> Self {
        ZenoOptions {
            table: "_default".to_owned(),
        }
    }
}

// entry db
struct Zeno {
    options: ZenoOptions,
    storage: MemoryStorage,
}

impl Zeno {
    fn new(options: ZenoOptions) -> Self {
        Self {
            options,
            storage: MemoryStorage::default(),
        }
    }

    fn show_table_name(&self) {
        println!("table name: {:?}", self.options.table)
    }

    fn all(&self) -> Result<Vec<Value>> {
        Ok(self.storage.read(json!({"all": true}))?)
    }

    fn insert(&mut self, obj: Value) -> Result<usize> {
        let id = self.storage.write(obj)?;
        Ok(id)
    }

    fn find(&self, query: Value) -> Result<Vec<Value>> {
        Ok(self.storage.read(query)?)
    }

    fn delete(&self, _query: Value) {}

    fn truncate(&self) {}

    fn update(&self, _query: Value, _fields: Value) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() -> Result<()> {
        let options = ZenoOptions::default();
        let mut db = Zeno::new(options);
        let id = db.insert(json!({"name": "mario", "age": 28}))?;
        Ok(assert_eq!(id, 0))
    }

    #[test]
    fn test_two_insert() -> Result<()> {
        let options = ZenoOptions::default();
        let mut db = Zeno::new(options);
        let idone = db.insert(json!({"name": "mario", "age": 28}))?;
        if idone != 0 {
            panic!(String::from("is not 0"));
        }

        let idtwo = db.insert(json!({"name": "mario", "age": 28}))?;
        if idtwo != 1 {
            panic!(String::from("is not 1"));
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let options = ZenoOptions::default();
    let mut db = Zeno::new(options);
    let id = db.insert(json!({"name": "mario", "age": 28}))?;
    let _x = db.find(json!({"name": "mario", "info": {"phone": 123}}))?;

    Ok(())
}
