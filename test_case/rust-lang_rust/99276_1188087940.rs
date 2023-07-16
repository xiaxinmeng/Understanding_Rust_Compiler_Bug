rust
pub struct Record<'a> {
    pub args: &'a [(usize, &'a str)],
}

mod a {
    extern "Rust" {
        fn foo<'a, 'b>(record: &'a super::Record<'b>);
    }
}

mod b {
    extern "Rust" {
        fn foo<'a, 'b>(record: &'a super::Record<'b>);
    }
}
