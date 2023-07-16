plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
error: field is never read: `1`
   --> library/alloc/src/collections/btree/set/tests.rs:472:30
    |
472 |     struct Foo(&'static str, i32);
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: field is never read: `0`
   --> library/alloc/src/collections/vec_deque/tests.rs:633:17
    |
633 |     struct Elem(i32);
---
    |
495 |     struct Elem(i32);
    |                 ^^^
    |
    = note: `-D dead-code` implied by `-D warnings`
error: field is never read: `0`
    --> library/alloc/tests/vec.rs:1080:22
     |
     |
1080 |         DroppedTwice(Box<i32>),

error: field is never read: `1`
    --> library/alloc/tests/vec.rs:2254:21
     |
     |
2254 |     struct Foo(i32, i32);
     |                     ^^^

error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:18:08
