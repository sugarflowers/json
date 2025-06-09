use std::fs;
#[allow(unused_imports)]
use serde_json::{Value, json, Map};


#[derive(Default, Debug)]
pub struct JsonData {
    pub json_data: Value,
}

impl JsonData {
    pub fn new() -> JsonData {
        JsonData::default()
    }

    pub fn open(filepath: &str) -> JsonData {
    let data = fs::read_to_string(filepath).expect("failed to read json file");
        let json_data:Value = serde_json::from_str(&data).expect("failed to parse JSON");
        Self {
            json_data: json_data,
        }
    }

    pub fn object(value:Value) -> Self {
        Self {
            json_data: value,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.json_data.get(key)
    }

    pub fn values(&self) -> &Value {
        &self.json_data
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

trait OptionValueExt {
    fn as_str(&self) -> String;
    fn as_i64(&self) -> i64;
    fn as_f64(&self) -> f64;
    fn as_bool(&self) -> bool;
    fn as_obj(&self) -> JsonData;
    fn as_array(&self) -> Vec<Value>;
}

impl OptionValueExt for Option<&Value> {
    fn as_str(&self) -> String {
        self.and_then(Value::as_str).expect("is not string").to_string()
    }
    fn as_i64(&self) -> i64 {
        self.and_then(Value::as_i64).expect("is not number")
    }
    fn as_f64(&self) -> f64 {
        self.and_then(Value::as_f64).expect("is not number")
    }
    fn as_bool(&self) -> bool {
        self.and_then(Value::as_bool).expect("is not bool")
    }
    fn as_obj(&self) -> JsonData {
        JsonData::object(json!(self.and_then(Value::as_object).expect("is not object")))
    }
    fn as_array(&self) -> Vec<Value> {
        self.and_then(Value::as_array).expect("is not array").to_vec()
    }
}

/*

test.json

{
    "name": "hanako",
    "age": 23,
    "single": true,
    "sub_data": {
        "address": "tokyo",
        "group": ["a", "b"]
    }
}

$ cargo test -- --nocapture

*/

#[test]
fn json_test() {
    let mut jd = JsonData::open("test.json");
    println!("{:?}", jd);
    
    println!("{}", jd.get("name").as_str());
    println!("{}", jd.get("age").as_i64()); 
    println!("{}", jd.get("single").as_bool());

    println!("{}", jd.get("sub_data").as_obj().get("address").as_str());



    let mut sd = jd.get("sub_data").as_obj();

    println!("{}", sd.get("group").as_array()[0].as_str().unwrap());

    let g0_w = sd.get("group").as_array();
    let g0 = g0_w[0].as_str().unwrap();
    println!("{}", g0);

    sd.set("address", json!("osaka".to_string()));
    sd.set("place", json!("midousuji".to_string()));
    jd.set("sub_data", json!(sd.values()));
    println!("{:?}", jd);

    jd.save("test2.json");

}

