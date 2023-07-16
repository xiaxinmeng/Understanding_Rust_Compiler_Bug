plain
   Compiling core v0.0.0 (/checkout/library/core)
error: field is never read: `0`
   --> library/core/benches/slice.rs:108:12
    |
108 | struct Rgb(u8, u8, u8);
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: field is never read: `1`
   --> library/core/benches/slice.rs:108:16
    |
    |
108 | struct Rgb(u8, u8, u8);

error: field is never read: `2`
   --> library/core/benches/slice.rs:108:20
    |
    |
108 | struct Rgb(u8, u8, u8);

error: field is never read: `0`
   --> library/core/benches/slice.rs:121:20
    |
---
warning: build failed, waiting for other jobs to finish...
error: field is never read: `0`
   --> library/core/tests/any.rs:122:21
    |
122 |     struct Velocity(f32, f32);
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: field is never read: `1`
   --> library/core/tests/any.rs:122:26
    |
    |
122 |     struct Velocity(f32, f32);

error: field is never read: `0`
   --> library/core/tests/array.rs:271:17
    |
    |
271 |     struct Bomb(usize);

error: field is never read: `0`
 --> library/core/tests/intrinsics.rs:7:14
  |
  |
7 |     struct Y(u32);

error: field is never read: `0`
  --> library/core/tests/intrinsics.rs:17:14
   |
   |
17 |     struct X(str);
   |              ^^^

error: field is never read: `0`
  --> library/core/tests/intrinsics.rs:18:14
   |
18 |     struct Y(dyn Z + 'static);

error: field is never read: `0`
   --> library/core/tests/ptr.rs:357:15
    |
---

error: field is never read: `0`
   --> library/core/tests/ptr.rs:364:15
    |
364 |     struct A7(u32, u16, u8);

error: field is never read: `1`
   --> library/core/tests/ptr.rs:364:20
    |
    |
364 |     struct A7(u32, u16, u8);

error: field is never read: `2`
   --> library/core/tests/ptr.rs:364:25
    |
    |
364 |     struct A7(u32, u16, u8);

error: field is never read: `0`
   --> library/core/tests/ptr.rs:366:15
    |
---

error: field is never read: `0`
   --> library/core/tests/ptr.rs:368:15
    |
368 |     struct A9(u32, u32, u8);

error: field is never read: `1`
   --> library/core/tests/ptr.rs:368:20
    |
    |
368 |     struct A9(u32, u32, u8);

error: field is never read: `2`
   --> library/core/tests/ptr.rs:368:25
    |
    |
368 |     struct A9(u32, u32, u8);

error: field is never read: `0`
   --> library/core/tests/ptr.rs:370:16
    |
    |
370 |     struct A10(u32, u32, u16);

error: field is never read: `1`
   --> library/core/tests/ptr.rs:370:21
    |
    |
370 |     struct A10(u32, u32, u16);

error: field is never read: `2`
   --> library/core/tests/ptr.rs:370:26
    |
    |
370 |     struct A10(u32, u32, u16);

error: field is never read: `0`
   --> library/core/tests/ptr.rs:437:31
    |
    |
437 |     struct Pair<A, B: ?Sized>(A, B);

error: field is never read: `0`
   --> library/core/tests/ptr.rs:527:22
    |
    |
527 |     struct Something([u8; 47]);

error: field is never read: `0`
    --> library/core/tests/slice.rs:1945:16
     |
     |
1945 |     struct U64(u64, u64);

error: field is never read: `1`
    --> library/core/tests/slice.rs:1945:21
     |
     |
1945 |     struct U64(u64, u64);

error: field is never read: `0`
    --> library/core/tests/slice.rs:1947:22
     |
     |
1947 |     struct U64U64U32(u64, u64, u32);

error: field is never read: `1`
    --> library/core/tests/slice.rs:1947:27
     |
     |
1947 |     struct U64U64U32(u64, u64, u32);

error: field is never read: `2`
    --> library/core/tests/slice.rs:1947:32
     |
     |
1947 |     struct U64U64U32(u64, u64, u32);

error: field is never read: `1`
    --> library/core/tests/slice.rs:2032:21
     |
     |
2032 |     struct Foo(i32, i32);
     |                     ^^^

error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:20:54
