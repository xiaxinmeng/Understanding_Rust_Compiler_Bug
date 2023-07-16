plain
error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a"
--- stdout -------------------------------
printed: box box x
printed: box x()
printed: box x.x(x)
printed: box x.x(x)
printed: box (x + x)
printed: box (x + x)
printed: box (x * x)
printed: box (x * x)
printed: box (x << x)
printed: box (x << x)
printed: box (x && x)
printed: box (x && x)
printed: box (x || x)
printed: box (x || x)
printed: box (x < x)
printed: box (x < x)
printed: box *x
printed: box if x {}
printed: box (move || x)
printed: box (x = x)
printed: box (x = x)
printed: box x.f
printed: box (x..x)
printed: box (x..x)
printed: box &x
printed: box (return)
printed: box (return x)
printed: box S { ..x }
printed: box x?
printed: box let _ = x
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:78:9
------------------------------------------



