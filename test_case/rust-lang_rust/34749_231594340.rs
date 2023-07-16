

failures:

---- any::Any + Send::downcast_mut_0 stdout ----
    <anon>:9:1: 9:2 error: unexpected close delimiter: `}`
<anon>:9 }
         ^
thread 'any::Any + Send::downcast_mut_0' panicked at 'Box<Any>', src/libsyntax/parse/mod.rs:258
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread 'any::Any + Send::downcast_mut_0' panicked at 'couldn't compile the test', src/librustdoc/test.rs:288

---- any::Any::downcast_mut_0 stdout ----
    <anon>:9:1: 9:2 error: unexpected close delimiter: `}`
<anon>:9 }
