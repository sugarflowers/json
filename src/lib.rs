use std::fs;
use serde_json::{Value, json};

#[derive(Debug)]
pub struct JsonData {
    pub json_data: Value,
}

impl JsonData {
    pub fn open(filepath: &str) -> Self {
    let data = fs::read_to_string(filepath).expect("failed to read json file");
        let json_data:Value = serde_json::from_str(&data).expect("failed to parse JSON");
        Self {
            json_data: json_data,
        }
    }


    pub fn get_string(&self, key: &str) -> String {
        self.json_data.get(key).and_then(Value::as_str).unwrap_or("").to_string()
    }

    pub fn get_i64(&self, key: &str) -> i64 {
        self.json_data.get(key).and_then(Value::as_i64).unwrap_or(0) 
    }

    pub fn get_f64(&self, key: &str) -> f64 {
        self.json_data.get(key).and_then(Value::as_f64).unwrap_or(0.0)
    }

    pub fn get_bool(&self, key: &str) -> bool {
        self.json_data.get(key).and_then(Value::as_bool).unwrap_or(false)
    }


    pub fn set(&mut self, key: &str, value: Value) {
        if let Some(obj) = self.json_data.as_object_mut() {
            obj.insert(key.to_string(), value);
        }
    }


    pub fn save(&self, filename: &str) {
        let json_str = serde_json::to_string_pretty(&self.json_data)
            .expect("failed to serialize json");
        fs::write(filename, json_str).expect("failed to save json file");
    }
}


/*

test.json

{
    "name": "hanako",
    "age": 23,
    "single": true
}
*/

#[test]
fn json_test() {
    let mut jd = JsonData::open("test.json");
    println!("{:?}", jd);

    let name = jd.get_string("name");
    println!("name: {}", name);
    let single = jd.get_bool("single");
    println!("single: {:?}", single);


    let age:i64 = 24;
    jd.set("age", Value::Number(age.into()));
    jd.set("place", Value::String("東京".to_string()));
    
    println!("{:?}", jd);

    jd.save("test2.json");
}

