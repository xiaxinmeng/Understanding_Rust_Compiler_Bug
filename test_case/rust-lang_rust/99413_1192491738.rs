rust
use ::core::mem::ManuallyDrop as MD;

let mut path_content_map = BTreeMap::<&str, Box<MD<dyn Read>>>::new();
let s = format!("");
let as_bytes = s.as_bytes();
path_content_map.insert("foo", Box::new(MD::new(as_bytes)));
