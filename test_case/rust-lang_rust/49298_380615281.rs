rust
let x: Option<(String, !)>;
let y = vec!["foo".to_string()];
x = Some((String::new("bar"), panic!("{:?}", y)));
