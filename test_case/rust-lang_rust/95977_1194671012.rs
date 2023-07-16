plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/benches/slice.rs:156:28
    |
156 |     struct NewType(#[allow(unused_tuple_struct_fields)] u8);
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
    --> library/alloc/tests/vec.rs:2229:29
     |
     |
2229 |     struct Foo(i32, #[allow(unused_tuple_struct_fields)] i32);

error: unknown lint: `unused_tuple_struct_fields`
   --> library/alloc/src/collections/btree/set/tests.rs:528:38
    |
    |
528 |     struct Foo(&'static str, #[allow(unused_tuple_struct_fields)] i32);
    |
    |
    = note: `-D unknown-lints` implied by `-D warnings`
error: unknown lint: `unused_tuple_struct_fields`
    --> library/alloc/src/collections/vec_deque/tests.rs:1024:25
     |
     |
1024 |     struct Elem(#[allow(unused_tuple_struct_fields)] i32);

error: unknown lint: `unused_tuple_struct_fields`
   --> library/std/src/collections/hash/set/tests.rs:354:38
    |
    |
354 |     struct Foo(&'static str, #[allow(unused_tuple_struct_fields)] i32);
    |
    |
    = note: `-D unknown-lints` implied by `-D warnings`
error: unknown lint: `unused_tuple_struct_fields`
   --> library/std/src/io/cursor/tests.rs:521:36
    |
    |
521 |     struct AssertEq<T: Eq>(#[allow(unused_tuple_struct_fields)] pub T);

error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
error: unknown lint: `unused_tuple_struct_fields`
error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/tests/any.rs:125:13
    |
125 |     #[allow(unused_tuple_struct_fields)]
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D unknown-lints` implied by `-D warnings`
error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/tests/array.rs:271:25
    |
    |
271 |     struct Bomb(#[allow(unused_tuple_struct_fields)] usize);

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
   --> library/core/tests/ptr.rs:474:39
    |
    |
474 |     struct Pair<A, B: ?Sized>(#[allow(unused_tuple_struct_fields)] A, B);

error: unknown lint: `unused_tuple_struct_fields`
   --> library/core/tests/ptr.rs:564:30
    |
    |
564 |     struct Something(#[allow(unused_tuple_struct_fields)] [u8; 47]);

error: unknown lint: `unused_tuple_struct_fields`
    --> library/core/tests/slice.rs:2048:13
     |
---

error: unknown lint: `unused_tuple_struct_fields`
    --> library/core/tests/slice.rs:2137:29
     |
2137 |     struct Foo(i32, #[allow(unused_tuple_struct_fields)] i32);

error: could not compile `alloc` due to 3 previous errors
error: could not compile `alloc` due to 2 previous errors
error: could not compile `std` due to 2 previous errors
