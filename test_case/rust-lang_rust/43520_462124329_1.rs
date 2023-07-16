
error: reached the recursion limit while instantiating `foo::<[closure@src/main.rs:3:20: 3:25]>`
 --> src/main.rs:1:1
  |
1 | / fn foo<F: Fn()>(x: bool, _: F) {
2 | |     if x {
3 | |         foo(false, || {})
4 | |     }
5 | | }
  | |_^
