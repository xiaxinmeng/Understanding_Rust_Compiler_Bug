rust
let is_valid_stmt: bool = syn::parse_str::<syn::Stmt>("pub let some = 3;")
    .is_ok(); // false
