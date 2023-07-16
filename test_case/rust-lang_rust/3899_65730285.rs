 rust
#[link(name = "somelib", kind = "static")]
extern {
    fn some_c_function();
}

fn main() {
    unsafe {
        some_c_function();
    }
}
