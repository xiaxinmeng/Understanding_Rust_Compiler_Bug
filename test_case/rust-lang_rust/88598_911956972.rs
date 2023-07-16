plain
    Checking ignore v0.4.17
   Compiling structopt-derive v0.4.9
    Checking thiserror v1.0.20
    Checking structopt v0.3.16
error[E0063]: missing field `could_be_bare_literal` in initializer of `rustc_ast::Block`
    |
    |
149 |     let block = ast::Block {
    |                 ^^^^^^^^^^ missing `could_be_bare_literal`
For more information about this error, try `rustc --explain E0063`.
error: could not compile `rustfmt-nightly` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
