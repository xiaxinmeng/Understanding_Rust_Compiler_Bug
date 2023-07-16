
use std::collections::hash_map::HashMap;

fn foo(parameters: &HashMap<String, String>) -> bool {
    parameters
        .get("key")
        .and_then(|_found: &String| {
            Some(false)
        }).unwrap_or(false)
}
