rust
mod foo {
    #[linkage="private"]
    static mut FOO: usize = 0;

    fn inc_non_generic() {
        unsafe { FOO += 1; }
    }

    fn inc_generic<T>(stuff: &Vec<T>) {
        unsafe { FOO += stuff.len(); }
    }
}
