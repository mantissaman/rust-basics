use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn main() {
    let mut props:HashMap<String, Value> = HashMap::new();
    props.insert("address".to_string(), json!("10 Ayrshire Crescent"));
    props.insert("age".to_string(), json!(44));
    let user:User = User {
        id: "001".to_string(),
        username: "mantissaman".to_string(),
        extra: props,
    };

    let serialized = serde_json::to_string(&user).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
