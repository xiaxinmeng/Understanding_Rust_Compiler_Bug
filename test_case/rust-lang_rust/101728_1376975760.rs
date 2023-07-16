rust
macro_rules! type_ascribe {
    ($e:expr, $t:ty) => {{
        let val: $t = $e;
        val
    }};
}
