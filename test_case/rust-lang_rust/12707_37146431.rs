 Rust
mod a {
    extern {
        fn func(); // declare void @func() unnamed_addr
    }
}

mod b {
    struct S {
        a: u64,
        b: u64,
        c: u64,
    }

    extern {
        fn func(s: S); // declare void @func(%"struct.b::S"* byval) unnamed_addr
    }
}
