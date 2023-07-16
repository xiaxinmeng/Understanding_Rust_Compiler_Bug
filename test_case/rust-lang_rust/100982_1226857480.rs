plain
    Checking unic-char-property v0.9.0
    Checking miniz_oxide v0.4.0
    Checking unicode-normalization v0.1.13
    Checking expect-test v1.0.1
error[E0277]: the trait bound `Mutex<Runtime>: Default` is not satisfied
    |
    |
309 | static RT: Lazy<Mutex<Runtime>> = Lazy::new(Default::default);
    |                                   --------- ^^^^^^^^^^^^^^^^ the trait `~const Default` is not implemented for `Mutex<Runtime>`
    |                                   required by a bound introduced by this call
    |
    |
note: the trait `Default` is implemented for `Mutex<Runtime>`, but that implementation is not `const`
    |
    |
309 | static RT: Lazy<Mutex<Runtime>> = Lazy::new(Default::default);

For more information about this error, try `rustc --explain E0277`.
error: could not compile `expect-test` due to previous error
warning: build failed, waiting for other jobs to finish...
