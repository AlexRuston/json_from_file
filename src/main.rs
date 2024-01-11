use std::fs;
use serde_json::Value;

fn main() {
    let data = fs::read_to_string("src/people.json")
        .expect("Unable to read file");

    let v: Vec<Value> = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");

    for item in &v {
        println!(
            "Name: {} - Age: {} - Gender: {} - Company: {} - Balance: {}\n",
            item["name"], item["age"], item["gender"], item["company"], item["balance"]
        );
    }

    // dbg!(json);
}
