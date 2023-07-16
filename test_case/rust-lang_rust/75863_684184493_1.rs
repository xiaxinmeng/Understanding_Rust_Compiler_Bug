rust
let xs = vec![Some("he"), Some("llo")];
assert_eq!(xs.into_iter().collect::<Option<String>>(), Some(String::from("hello")));

let xs = vec![Some("he"), None, Some("llo")];
assert_eq!(xs.into_iter().collect::<Option<String>>(), None);
