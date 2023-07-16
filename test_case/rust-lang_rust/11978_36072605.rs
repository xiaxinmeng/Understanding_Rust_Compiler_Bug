 rust
extern {
    #[weak] static FOO: *T;
    #[weak] static mut FOO_MUT: *T;
}
