 rust
struct Field {
    field: [u8; 512]
}

struct Row {
    name: Field,
    email: Field
}

struct Database {
    rows: [Row; 60]
}

struct Connection {
    database: Database
}

const EMPTY_FIELD: Field = Field { field: [0; 512] };
const EMPTY_ROW: Row = Row {name: EMPTY_FIELD, email: EMPTY_FIELD};
const EMPTY_DATABASE: Database = Database {rows: [EMPTY_ROW; 60]};

fn main() {
    Connection { database: EMPTY_DATABASE };
}
