rust
// lib.rs
use std::collections::HashMap;

type Map<'a> = HashMap<&'a str, &'a str>;

fn a(_: &()) -> Map {
    todo!()
}

fn b<'a>(_: &'a Map) {
    todo!()
}
