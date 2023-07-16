rust
macro_rules! run_function {
    ($func: ident) => {
        #[allow(unused_results)]
        {
            &$func();
        }
    };
}
