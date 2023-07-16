rust
let mut h: HashMap<String, String> = HashMap::new();
h.insert(String::from("a"), String::from("a"));
let a = h.get("a").unwrap();
h.insert(String::from("b"), format!("{}", a));
