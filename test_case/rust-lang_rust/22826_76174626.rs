 rust
use std::collections::HashMap;
fn main(){
    let mut map = HashMap::new();
    map.insert("foo".to_string(), "bar".to_string());

    // Ok, maybe I can deref to cancel out the autoref?
    let key = "foo";
    println!("{}", map[*key]);
}
