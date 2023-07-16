 rust
#![allow(dead_code)]

enum Value {
    A(String)
}

struct Table {
    rows: [[Value]]
}

impl Table {
    fn next(&self) {
        &self.rows[0];
    }
}

fn main() {}
