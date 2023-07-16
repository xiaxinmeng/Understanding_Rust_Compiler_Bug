plain
    Checking expect-test v1.0.1
    Checking sha-1 v0.10.0
    Checking md-5 v0.10.0
    Checking sha2 v0.10.1
error[E0063]: missing field `position_span` in initializer of `Argument<'_>`
   --> compiler/rustc_parse_format/src/tests.rs:129:24
    |
129 |         &[NextArgument(Argument {
    |                        ^^^^^^^^ missing `position_span`
For more information about this error, try `rustc --explain E0063`.
error: could not compile `rustc_parse_format` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:50
