 rust
macro_rules! nested_expr {
    () => (fake)
}

macro_rules! call_nested_expr_sum { //~ NOTE in expansion of
    () => { 1 + nested_expr!(); } //~ ERROR unresolved name
}

fn main() {
    call_nested_expr_sum!(); //~ NOTE expansion site
}
