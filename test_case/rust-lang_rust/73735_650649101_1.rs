rust
mod c {
    extern "C" { fn foo() -> NonZeroUsize; }
}
mod d {
    extern "C" { fn foo() -> usize; }
}
