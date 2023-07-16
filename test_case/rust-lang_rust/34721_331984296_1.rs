
error[E0382]: use of moved value: `x`
  --> src/main.rs:25:3
   |
   | pub fn baz<T: Foo>(x: T) -> T {
   |            ------ help: consider adding a `Copy` constraint here: `T: Foo + Copy`
   |   if 0 == 1 {
21 |     bar::bar(x.zero())
   |              - value moved here
...
25 |   x.zero()
   |   ^ value used here after move
   |
   = note: move occurs because `x` has type `T`, which does not implement the `Copy` trait

