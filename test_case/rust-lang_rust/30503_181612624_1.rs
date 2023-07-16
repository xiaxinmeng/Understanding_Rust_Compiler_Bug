 rust
alias Return = (usize, isize);

mod a {
    fn foo() -> super::Return;
}

mod b {
    fn foo() -> super::Return;
}
