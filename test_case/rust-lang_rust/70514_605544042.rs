
   Compiling playground v0.0.1 (/playground)
error[E0282]: type annotations needed
  --> src/lib.rs:31:5
   |
31 |     C::join(ctx,a);
   |     ^^^^^^^ cannot infer type for type parameter `P` declared on the trait `B`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.
