plain
Successfully built e895be00fcb8
Successfully tagged rust-ci:latest
Built container sha256:e895be00fcb8d555c9c7d7f64bac9b4e5685e6f7bd1e59c38dfd0c992a4f1f96
Uploading finished image to https://ci-caches.rust-lang.org/docker/c165c0dbe07f979c65839814f65d46fc3b20640fec5c6162fa0d12eccd662a5501f008e9d9c1953dc5ee940f259fa67db4f7e378e34b63c10ab0f0e1d88ab56a
upload failed: - to s3://rust-lang-ci-sccache2/docker/c165c0dbe07f979c65839814f65d46fc3b20640fec5c6162fa0d12eccd662a5501f008e9d9c1953dc5ee940f259fa67db4f7e378e34b63c10ab0f0e1d88ab56a Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    |
122 |     #[allow(unused_tuple_struct_fields)]
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D unknown-lints` implied by `-D warnings`
error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/tests/array.rs:271:25
    |
    |
271 |     struct Bomb(#[allow(unused_tuple_struct_fields)] usize);

error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/benches/slice.rs:156:28
    |
    |
156 |     struct NewType(#[allow(unused_tuple_struct_fields)] u8);
    |
    |
    = note: `-D unknown-lints` implied by `-D warnings`
error: unknown lint: `unused_tuple_struct_fields`
 --> library/core/tests/intrinsics.rs:7:22
  |
  |
7 |     struct Y(#[allow(unused_tuple_struct_fields)] u32);

error: unknown lint: `unused_tuple_struct_fields`
  --> library/core/tests/intrinsics.rs:17:22
   |
   |
17 |     struct X(#[allow(unused_tuple_struct_fields)] str);

error: unknown lint: `unused_tuple_struct_fields`
  --> library/core/tests/intrinsics.rs:18:22
   |
   |
18 |     struct Y(#[allow(unused_tuple_struct_fields)] dyn Z + 'static);

error: unknown lint: `unused_tuple_struct_fields`
   --> library/std/src/collections/hash/set/tests.rs:354:38
    |
    |
354 |     struct Foo(&'static str, #[allow(unused_tuple_struct_fields)] i32);
    |
    |
    = note: `-D unknown-lints` implied by `-D warnings`
error: unknown lint: `unused_tuple_struct_fields`
   --> library/alloc/tests/vec.rs:498:25
    |
    |
498 |     struct Elem(#[allow(unused_tuple_struct_fields)] i32);
    |
    |
    = note: `-D unknown-lints` implied by `-D warnings`
error: unknown lint: `unused_tuple_struct_fields`
    --> library/alloc/tests/vec.rs:1108:30
     |
     |
1108 |         DroppedTwice(#[allow(unused_tuple_struct_fields)] Box<i32>),

error: unknown lint: `unused_tuple_struct_fields`
    --> library/alloc/tests/vec.rs:2282:29
     |
     |
2282 |     struct Foo(i32, #[allow(unused_tuple_struct_fields)] i32);

error: unknown lint: `unused_tuple_struct_fields`
   --> library/alloc/src/collections/btree/set/tests.rs:471:38
    |
    |
471 |     struct Foo(&'static str, #[allow(unused_tuple_struct_fields)] i32);
    |
    |
    = note: `-D unknown-lints` implied by `-D warnings`
error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/tests/ptr.rs:374:13
    |
374 |     #[allow(unused_tuple_struct_fields)]
---

error: unknown lint: `unused_tuple_struct_fields`
   --> library/alloc/src/collections/vec_deque/tests.rs:633:25
    |
633 |     struct Elem(#[allow(unused_tuple_struct_fields)] i32);

error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/tests/ptr.rs:394:13
    |
    |
394 |     #[allow(unused_tuple_struct_fields)]
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/tests/ptr.rs:462:39
    |
462 |     struct Pair<A, B: ?Sized>(#[allow(unused_tuple_struct_fields)] A, B);

error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/tests/ptr.rs:552:30
    |
    |
552 |     struct Something(#[allow(unused_tuple_struct_fields)] [u8; 47]);

error: unknown lint: `unused_tuple_struct_fields`
    --> library/core/tests/slice.rs:2048:13
     |
---

error: unknown lint: `unused_tuple_struct_fields`
    --> library/core/tests/slice.rs:2137:29
     |
2137 |     struct Foo(i32, #[allow(unused_tuple_struct_fields)] i32);

error: unknown lint: `unused_tuple_struct_fields`
   --> library/std/src/io/cursor/tests.rs:509:36
    |
    |
509 |     struct AssertEq<T: Eq>(#[allow(unused_tuple_struct_fields)] pub T);

error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `alloc` due to 2 previous errors
