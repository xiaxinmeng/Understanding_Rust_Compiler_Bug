 rust
macro_rules! if_true { () => { if true {} } }

// try to merge the `else` into the `if
if_true!() else {}
