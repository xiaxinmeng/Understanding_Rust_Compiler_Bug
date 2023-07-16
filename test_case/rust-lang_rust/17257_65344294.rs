 rust
extern "C" fn foo(_: Blah) { }

struct Blah {
    _ptr: extern "C" fn(Blah)
}

fn main() {
    Blah { _ptr: foo };
}
