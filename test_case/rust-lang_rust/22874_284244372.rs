rust
struct Table {
    rows: [[String]]
}

fn f(table: &Table) -> &[String] {
    &table.rows[0]
}

fn main() {}
