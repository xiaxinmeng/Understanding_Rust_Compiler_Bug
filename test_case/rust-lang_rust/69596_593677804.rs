rust
mod mod_a {
    mod name_of_proc_macro {}

    use crate::mod_b::*;

    mod mod_c {
        fn _foobar() {
            use super::name_of_proc_macro;
        }
    }
}

mod mod_b {
    pub(crate) use std::format as name_of_proc_macro; // DELETE TO TRIGGER ICE
}
