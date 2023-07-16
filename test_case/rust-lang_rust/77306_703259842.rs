rust
pub fn write() {
    create()()
}

pub fn create() -> impl FnOnce() {
   || ()
}
