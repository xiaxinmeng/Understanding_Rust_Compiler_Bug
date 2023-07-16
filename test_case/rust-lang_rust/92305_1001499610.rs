
error[E0107]: missing generics for struct `Vec`
   --> src/main.rs:3:45
    |
3   | fn f<T>(data: &[T]) -> impl Iterator<Item = Vec> {
    |                                             ^^^ expected at least 1 generic argument
    |
note: struct defined here, with at least 1 generic parameter: `T`
   --> /home/jman/.rustup/toolchains/1.56.1-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:398:12
    |
398 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    |            ^^^ -
help: add missing generic argument
    |
3   | fn f<T>(data: &[T]) -> impl Iterator<Item = Vec<T>> {
    |                                             ~~~~~~

error[E0282]: type annotations needed
 --> src/main.rs:4:5
  |
4 |     iter::empty()
  |     ^^^^^^^^^^^ cannot infer type for type parameter `T` declared on the function `empty`

error[E0720]: cannot resolve opaque type
 --> src/main.rs:7:35
  |
3 | fn f<T>(data: &[T]) -> impl Iterator<Item = Vec> {
  |                        ------------------------- returning this opaque type `Filter<impl Iterator, [closure@src/main.rs:8:20: 8:35]>`
...
7 | fn g<T>(data: &[T], target: T) -> impl Iterator<Item = Vec<T>> {
  |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
8 |     f(data).filter(|x| x == target)
  |     ------------------------------- returning here with type `Filter<impl Iterator, [closure@src/main.rs:8:20: 8:35]>`

Some errors have detailed explanations: E0107, E0282, E0720.
For more information about an error, try `rustc --explain E0107`.
error: could not compile `issue-92305` due to 3 previous errors
