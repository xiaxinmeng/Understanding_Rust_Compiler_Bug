shell
   Compiling playground v0.0.1 (file:///playground)
error[E0564]: only named lifetimes are allowed in `impl Trait`, but `` was found in the type `[generator@src/main.rs:13:9: 13:86 self:&mut Stack ((), std::ops::Range<i32>, i32, fn(std::ops::Range<i32>) -> <std::ops::Range<i32> as std::iter::IntoIterator>::IntoIter {<std::ops::Range<i32> as std::iter::IntoIterator>::into_iter}, Stack)]`
  --> src/main.rs:11:27
   |
11 |     fn gen(& mut self) -> impl Generator<Yield=i32,Return=()> { 
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
