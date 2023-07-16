rust
error[E0282]: type annotations needed
  --> $DIR/xxx.rs:xx:xx
   |
14 |     let x: i32 = foo();
   |                  ^^^ cannot infer type for `U`
   |                  |
   |                  you can specify the generic parameter here using `foo::<_, U>()`
   |
   = note: generic parameter `U` needs to be specified for `fn foo::<T, U>() -> T`
