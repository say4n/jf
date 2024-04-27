use serde_json::{json, Map, Value};

pub fn flatten(root: Value) -> Value {
    let _flattened = _flatten_impl(root, &String::from(""));
    let mut flattened = Map::new();

    match _flattened {
        Value::Object(items) => {
            for (k, v) in items {
                flattened.insert(k[1..].to_string(), v);
            }
        }
        _ => {}
    }

    return Value::Object(flattened);
}

fn _flatten_impl(root: Value, key: &String) -> Value {
    // println!("key: {:?}", key);

    match root {
        Value::Null => return root,
        Value::Bool(value) => return json!({key: value}),
        Value::Number(value) => return json!({key: value}),
        Value::String(value) => return json!({key: value}),
        Value::Array(items) => {
            let mut flattened_array = Map::new();

            for (index, item) in items.iter().enumerate() {
                let subkey = format!("{}.{}", key, index);
                let parsed_subtree = _flatten_impl(item.clone(), &subkey);

                match parsed_subtree {
                    Value::Bool(value) => flattened_array.insert(subkey, Value::Bool(value)),
                    Value::Number(value) => flattened_array.insert(subkey, Value::Number(value)),
                    Value::String(value) => flattened_array.insert(subkey, Value::String(value)),
                    Value::Object(parsed_subtree_items) => {
                        for (k, v) in parsed_subtree_items {
                            flattened_array.insert(k, v);
                        }
                        Some(Value::Null)
                    }
                    Value::Null | Value::Array(_) => Some(Value::Null),
                };
            }

            // println!("key: {:?}, flattened_array: {:#}\n\n", key, Value::Object(flattened_array.clone()));
            return Value::Object(flattened_array);
        }
        Value::Object(items) => {
            let mut flattened_dict = Map::new();

            for (nested_key, nested_value) in items {
                let subkey = format!("{}.{}", key, nested_key);
                let parsed_subtree = _flatten_impl(nested_value.clone(), &subkey);

                match parsed_subtree {
                    Value::Bool(value) => flattened_dict.insert(subkey, Value::Bool(value)),
                    Value::Number(value) => flattened_dict.insert(subkey, Value::Number(value)),
                    Value::String(value) => flattened_dict.insert(subkey, Value::String(value)),
                    Value::Object(parsed_subtree_items) => {
                        for (k, v) in parsed_subtree_items {
                            flattened_dict.insert(k, v);
                        }
                        Some(Value::Null)
                    }
                    Value::Null | Value::Array(_) => Some(Value::Null),
                };
            }

            // println!("key: {:?}, flattened_dict: {:#}\n\n", key, Value::Object(flattened_dict.clone()));
            return Value::Object(flattened_dict);
        }
    }
}
