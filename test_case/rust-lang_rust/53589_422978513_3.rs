
error[E0392]: parameter `U` is never used
 --> src/main.rs:2:18
  |
2 | struct Cacher<T, U: Copy>
  |                  ^ unused type parameter
  |
  = help: you might have meant to use `U` here
  |
6 |     value: Option<U>,
  |                   -
