rust
macro_rules! vec {
    () => (
        vec!(@force_expr_internal_do_not_use@: $crate::vec::Vec::new())
    );
    ($elem:expr; $n:expr) => (
        vec!(@force_expr_internal_do_not_use@: $crate::vec::from_elem($elem, $n))
    );
    ($($x:expr),+ $(,)?) => (
        vec!(@force_expr_internal_do_not_use@: <[_]>::into_vec(box [$($x),+]))
    );
    (@force_expr_internal_do_not_use@: $e:expr) => ($e);
}
