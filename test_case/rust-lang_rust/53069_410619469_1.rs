rust
fn foo(x: bool) {  // SSE2
    avx(); // WARNING
    if detect("avx") {
        avx(); // OK (NO WARNING)
    }
    let b = detect("avx");
    if b { avx(); } // OK (NO WARNING)
    if x { avx() } // ??? MIGHT BE OK
}
