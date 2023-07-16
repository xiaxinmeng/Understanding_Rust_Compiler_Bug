 rust
#![allow(dead_code)]

enum Value {
    String(String)
}

type Row = [Value];

struct Table {
    rows: [Row]
}

impl Table {
    fn next(&self) -> &Row {
        &self.rows[0]
    }
}

fn main() {
}
