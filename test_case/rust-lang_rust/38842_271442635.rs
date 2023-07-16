rust
macro_rules! m {
    ($i:ident) => {
        let x = 0;
        x; // This resolves correctly
        my_identity_proc_macro!(x); // but this doesn't (if implemented with today's API)
        //^ Assume this is a proc macro that just returns its input
    }
}
