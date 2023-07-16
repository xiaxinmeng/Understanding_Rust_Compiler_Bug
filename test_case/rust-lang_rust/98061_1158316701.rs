rust
let a: &Option<i32> = &Some(1);
let b: Option<i32> = a.to_owned();
