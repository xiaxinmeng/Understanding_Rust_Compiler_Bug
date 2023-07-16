rust
mod a {
    extern "C" { fn test() -> usize; }
}
mod b {
    extern "C" { fn test(); }
}

unsafe {
    b:: test();
    let s = a::test();
}
