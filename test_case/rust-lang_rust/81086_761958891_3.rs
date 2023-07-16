rust
macro_rules! run_function {
    ($func: ident) => {{
        $func();
    }};
}
