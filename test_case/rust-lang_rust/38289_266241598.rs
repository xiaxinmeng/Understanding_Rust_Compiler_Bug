rust
failures: 

---- [mir-opt] mir-opt/deaggregator_test.rs stdout ----
        thread '[mir-opt] mir-opt/deaggregator_test.rs' panicked at 'ran out of mir dump output to match against.
Did not find expected line: "    _0 = Baz { x: _3, y: const F32(0), z: const false };"
Expected: 
bb0: {
    _2 = _1;
    _3 = _2;
    _0 = Baz { x: _3, y: const F32(0), z: const false };
    return;
}
Actual:   
fn bar(_1: usize) -> Baz {
    let mut _0: Baz;
    scope 1 {
        let _2: usize;
    }
    let mut _3: usize;
    bb0: {
        StorageLive(_2);
        _2 = _1;
        StorageLive(_3);
        _3 = _2;
        (_0.0: usize) = _3;
        (_0.1: f32) = const F32(0);
        (_0.2: bool) = const false;
        StorageDead(_3);
        StorageDead(_2);
        return;
    }
}', src/tools/compiletest/src/runtest.rs:2302
note: Run with `RUST_BACKTRACE=1` for a backtrace.
