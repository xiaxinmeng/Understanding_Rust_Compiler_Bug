rust
extern "C" fn bad() {
    panic!()
}

fn main() {
    bad()
}
