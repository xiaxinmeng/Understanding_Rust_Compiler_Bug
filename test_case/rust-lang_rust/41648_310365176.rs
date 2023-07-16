rust
struct Inner { }

mod private_mod {
    use super::Inner;

    pub(crate) fn use_me(inner: Inner) {
        println!("function from private module");
    }
}

fn main() {
    private_mod::use_me(Inner { });
}
