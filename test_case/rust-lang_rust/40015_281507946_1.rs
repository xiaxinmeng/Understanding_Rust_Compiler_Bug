rust
error[E0282]: type annotations needed
  --> $DIR/xxx.rs:xx:xx
   |
14 |     let x: i32 = foo();
   |                  ^^^ cannot infer type for `U`
   |
   = note: type annotations or generic parameter binding required
   = note: generic parameter `U` needs to be specified for foo::<T, U>()
