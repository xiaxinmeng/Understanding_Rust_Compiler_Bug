plain
    Checking thorin-dwp v0.2.0
error: cannot find macro `newtype_index` in this scope
 --> compiler/rustc_index/src/vec/tests.rs:2:1
  |
2 | newtype_index!(struct MyIdx { MAX = 0xFFFF_FFFA });
  |
  = note: consider importing one of these items:
          crate::newtype_index
          rustc_macros::newtype_index
          rustc_macros::newtype_index

error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:25:17
   |
25 |     let range = MyIdx::from_u32(1)..MyIdx::from_u32(4);
   |                 ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:25:37
   |
   |
25 |     let range = MyIdx::from_u32(1)..MyIdx::from_u32(4);
   |                                     ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:28:10
   |
   |
28 |         [MyIdx::from_u32(1), MyIdx::from_u32(2), MyIdx::from_u32(3)]
   |          ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:28:30
   |
   |
28 |         [MyIdx::from_u32(1), MyIdx::from_u32(2), MyIdx::from_u32(3)]
   |                              ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:28:50
   |
   |
28 |         [MyIdx::from_u32(1), MyIdx::from_u32(2), MyIdx::from_u32(3)]
   |                                                  ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:34:17
   |
   |
34 |     let range = MyIdx::from_u32(1)..MyIdx::from_u32(4);
   |                 ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:34:37
   |
   |
34 |     let range = MyIdx::from_u32(1)..MyIdx::from_u32(4);
   |                                     ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:37:10
   |
   |
37 |         [MyIdx::from_u32(3), MyIdx::from_u32(2), MyIdx::from_u32(1)]
   |          ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:37:30
   |
   |
37 |         [MyIdx::from_u32(3), MyIdx::from_u32(2), MyIdx::from_u32(1)]
   |                              ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:37:50
   |
   |
37 |         [MyIdx::from_u32(3), MyIdx::from_u32(2), MyIdx::from_u32(1)]
   |                                                  ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:43:17
   |
   |
43 |     let range = MyIdx::from_u32(1)..MyIdx::from_u32(4);
   |                 ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:43:37
   |
   |
43 |     let range = MyIdx::from_u32(1)..MyIdx::from_u32(4);
   |                                     ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:49:17
   |
   |
49 |     let range = MyIdx::from_u32(1)..MyIdx::from_u32(4);
   |                 ^^^^^ use of undeclared type `MyIdx`
error[E0433]: failed to resolve: use of undeclared type `MyIdx`
  --> compiler/rustc_index/src/vec/tests.rs:49:37
   |
   |
49 |     let range = MyIdx::from_u32(1)..MyIdx::from_u32(4);
   |                                     ^^^^^ use of undeclared type `MyIdx`

error[E0412]: cannot find type `MyIdx` in this scope
 --> compiler/rustc_index/src/vec/tests.rs:8:26
5 | fn index_size_is_optimized() {
5 | fn index_size_is_optimized() {
  |                           - help: you might be missing a type parameter: `<MyIdx>`
...
8 |     assert_eq!(size_of::<MyIdx>(), 4);


error[E0412]: cannot find type `MyIdx` in this scope
  --> compiler/rustc_index/src/vec/tests.rs:10:33
5  | fn index_size_is_optimized() {
5  | fn index_size_is_optimized() {
   |                           - help: you might be missing a type parameter: `<MyIdx>`
...
10 |     assert_eq!(size_of::<Option<MyIdx>>(), 4);


error[E0412]: cannot find type `MyIdx` in this scope
  --> compiler/rustc_index/src/vec/tests.rs:12:40
5  | fn index_size_is_optimized() {
5  | fn index_size_is_optimized() {
   |                           - help: you might be missing a type parameter: `<MyIdx>`
...
12 |     assert_eq!(size_of::<Option<Option<MyIdx>>>(), 4);


error[E0412]: cannot find type `MyIdx` in this scope
  --> compiler/rustc_index/src/vec/tests.rs:14:47
5  | fn index_size_is_optimized() {
5  | fn index_size_is_optimized() {
   |                           - help: you might be missing a type parameter: `<MyIdx>`
...
14 |     assert_eq!(size_of::<Option<Option<Option<MyIdx>>>>(), 4);


error[E0412]: cannot find type `MyIdx` in this scope
  --> compiler/rustc_index/src/vec/tests.rs:16:54
5  | fn index_size_is_optimized() {
5  | fn index_size_is_optimized() {
   |                           - help: you might be missing a type parameter: `<MyIdx>`
...
16 |     assert_eq!(size_of::<Option<Option<Option<Option<MyIdx>>>>>(), 4);


error[E0412]: cannot find type `MyIdx` in this scope
  --> compiler/rustc_index/src/vec/tests.rs:18:61
5  | fn index_size_is_optimized() {
5  | fn index_size_is_optimized() {
   |                           - help: you might be missing a type parameter: `<MyIdx>`
...
18 |     assert_eq!(size_of::<Option<Option<Option<Option<Option<MyIdx>>>>>>(), 4);


error[E0412]: cannot find type `MyIdx` in this scope
  --> compiler/rustc_index/src/vec/tests.rs:20:68
5  | fn index_size_is_optimized() {
5  | fn index_size_is_optimized() {
   |                           - help: you might be missing a type parameter: `<MyIdx>`
...
20 |     assert_eq!(size_of::<Option<Option<Option<Option<Option<Option<MyIdx>>>>>>>(), 8);

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rustc_index` due to 22 previous errors
