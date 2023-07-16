
[snip]
3 | fn f(x: Cell<(&i32, &i32)>) {
  |               -     - let's call the lifetime of this reference `'1`
  |               |
  |               let's call the lifetime of this reference `'2`
4 |     g(x)
  |     ^^^^ argument requires that `'1` must outlive `'2`
