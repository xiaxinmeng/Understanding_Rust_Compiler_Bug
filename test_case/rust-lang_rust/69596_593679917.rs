rust
mod mod_a {
    mod name_of_proc_macro {}

    use crate::mod_b::*;

    fn _foobar() {
        use name_of_proc_macro;
    }
}

mod mod_b {
    pub(crate) use std::format as name_of_proc_macro;
}
