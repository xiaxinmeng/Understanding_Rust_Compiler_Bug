
error[[E0720]](https://doc.rust-lang.org/nightly/error-index.html#E0720): cannot resolve opaque type
  --> src/main.rs:5:13
   |
5  |   fn foo() -> impl Generator<Yield = (), Return = ()> {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
6  | /     || {
7  | |         let mut gen = Box::pin(foo());
8  | |         let mut r = gen.as_mut().resume(());
9  | |         while let GeneratorState::Yielded(v) = r {
...  |
12 | |         }
13 | |     }
   | |_____- returning here with type `[generator@src/main.rs:6:5: 6:7]`
