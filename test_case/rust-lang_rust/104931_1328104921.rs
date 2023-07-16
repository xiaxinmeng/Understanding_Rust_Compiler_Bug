
error[E0308]: mismatched types
[…]
   = note:     expected struct `Pin<Box<dyn Future<Output = i32> + Send>>`
           found `async` block `[async block@$DIR/expected-boxed-future-isnt-pinned.rs:28:5: 30:6]`
