rust
error[E0282]: type annotations needed
  --> $DIR/xxx.rs:xx:xx
   |
14 |     let x: i32 = foo();
   |                  ^^^ cannot infer type for `U` in `fn foo<T, U>() -> T`
   |                  |
   |                  specify `U` using `foo::<_, U>()`
   |
