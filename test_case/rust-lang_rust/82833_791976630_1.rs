rust
// compile-flags: -lrepro -L.
extern "C" {
    fn banana() -> String;
}

fn main() {
    unsafe {
        assert_eq!(banana(), "😃");
    }
}
