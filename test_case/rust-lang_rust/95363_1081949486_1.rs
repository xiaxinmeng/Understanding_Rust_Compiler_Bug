rust
extern "C" {
    fn bar();
}

fn main() {
    unsafe { bar() };
}
