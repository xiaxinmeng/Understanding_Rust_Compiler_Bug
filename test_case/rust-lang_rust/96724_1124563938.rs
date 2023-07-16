plain
    Checking structopt v0.3.25
error[E0532]: expected unit struct, unit variant or constant, found tuple variant `Delimiter::Invisible`
   --> src/tools/rustfmt/src/macros.rs:565:9
    |
565 |         Delimiter::Invisible => unreachable!(),
    |         ^^^^^^^^^^^^^^^^^^^^ help: use the tuple variant pattern syntax instead: `Delimiter::Invisible(/* fields */)`
   ::: /checkout/compiler/rustc_ast/src/token.rs:60:5
    |
    |
60  |     Invisible(InvisibleSource),
    |     -------------------------- `Delimiter::Invisible` defined here
For more information about this error, try `rustc --explain E0532`.
error: could not compile `rustfmt-nightly` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustfmt-nightly` due to previous error
