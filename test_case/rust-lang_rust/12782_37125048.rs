 rust
macro_rules! count_expr {
    () => { 0 };
    ($_e: expr $(, $rest: expr)*) => { 1 + count_expr!($($rest),*) }
}
