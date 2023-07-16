 rust
enum E { V }

fn f<E>(_: E) {
    fn g() {
        let _ = E::V; // is the type parameter `E` in scope here? (currently, it is)
    }
}
