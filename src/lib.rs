use std::fs;
#[allow(unused_imports)]
use serde_json::{Value, json, Map};
use anyhow::{Result, Context};

#[derive(Default, Debug)]
pub struct JsonData {
    pub json_data: Value,
}

impl JsonData {
    pub fn new() -> JsonData {
        JsonData::default()
    }

    pub fn open(filepath: &str) -> Result<JsonData> {
    let data = fs::read_to_string(filepath).context("failed to read json file")?;
        let json_data:Value = serde_json::from_str(&data).context("failed to parse JSON")?;
        Ok(Self {
            json_data: json_data,
        })
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

    pub fn save(&self, filename: &str) -> Result<()> {
        let json_str = serde_json::to_string_pretty(&self.json_data)
            .context("failed to serialize json")?;
        fs::write(filename, json_str).context("failed to save json file")?;
        Ok(())
    }
}

#[allow(dead_code)]
trait OptionValueExt {
    fn as_str(&self) -> Result<String>;
    fn as_i64(&self) -> Result<i64>;
    fn as_f64(&self) -> Result<f64>;
    fn as_bool(&self) -> Result<bool>;
    fn as_obj(&self) -> Result<JsonData>;
    fn as_array(&self) -> Result<Vec<Value>>;
}

impl OptionValueExt for Option<&Value> {
    fn as_str(&self) -> Result<String> {
        let buf = self.and_then(Value::as_str).context("is not string")?;
        Ok(buf.to_string())
    }
    fn as_i64(&self) -> Result<i64> {
        let buf = self.and_then(Value::as_i64).context("is not number")?;
        Ok(buf)
    }
    fn as_f64(&self) -> Result<f64> {
        let buf = self.and_then(Value::as_f64).context("is not number")?;
        Ok(buf)
    }
    fn as_bool(&self) -> Result<bool> {
        let buf = self.and_then(Value::as_bool).context("is not bool")?;
        Ok(buf)
    }
    fn as_obj(&self) -> Result<JsonData> {
        let buf = JsonData::object(json!(self.and_then(Value::as_object)
            .context("is not object")?));
        Ok(buf)
    }
    fn as_array(&self) -> Result<Vec<Value>> {
        let buf = self.and_then(Value::as_array).context("is not array")?.to_vec();
        Ok(buf)

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
    let mut jd = JsonData::open("test.json").unwrap();
    println!("{:?}", jd);
    
    let ret = jd.get("age").as_str();
    println!("{:?}", ret);

    println!("{:?}", jd.get("name").as_str().unwrap());
    println!("{:?}", jd.get("age").as_i64()); 
    println!("{:?}", jd.get("single").as_bool());

    println!("{:?}", jd.get("sub_data").as_obj().unwrap().get("address").as_str().unwrap());



    let mut sd = jd.get("sub_data").as_obj().unwrap();

    println!("{:?}", sd.get("group").as_array().unwrap()[0].as_str().unwrap());

    let g0_w = sd.get("group").as_array().unwrap();
    let g0 = g0_w[0].as_str().unwrap();
    println!("{:?}", g0);

    sd.set("address", json!("osaka".to_string()));
    sd.set("place", json!("midousuji".to_string()));
    jd.set("sub_data", json!(sd.values()));
    println!("{:?}", jd);

    jd.save("test2.json").unwrap();

}

