rust
error[E0382]: capture of moved value: `f`
   --> src/internal/context.rs:162:53
    |
162 |     CONTEXT.try_with(|cx| f(cx)).unwrap_or_else(|_| f(&Context::new()))
    |                      ----                           ^ value captured here after move
    |                      |
    |                      value moved (into closure) here
    |
    = note: move occurs because `f` has type `F`, which does not implement the `Copy` trait
