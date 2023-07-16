rust
impl Thingy {
    fn foo_inner(&self) -> impl FnOnce() + '_ {
        move || {
            println!("{}", self.0);
        }
    }
}
