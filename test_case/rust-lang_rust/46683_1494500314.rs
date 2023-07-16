rust
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

let table = vec![
    vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
    vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
    vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
]
.table()
.title(vec![
    "Name".cell().bold(true),
    "Age (in years)".cell().bold(true),
])
.bold(true);

assert!(print_stdout(table).is_ok());
