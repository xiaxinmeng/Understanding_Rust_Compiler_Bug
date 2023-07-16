plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `to_fluent_args` found for mutable reference `&mut BufferEmitter` in the current scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:196:32
    |
196 |         let fluent_args = self.to_fluent_args(diag.args());
    |                                ^^^^^^^^^^^^^^ method not found in `&mut BufferEmitter`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:09
