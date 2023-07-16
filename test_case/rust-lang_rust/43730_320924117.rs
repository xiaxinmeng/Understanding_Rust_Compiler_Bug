
[00:31:18]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:31:23] error[E0061]: this function takes 3 parameters but 2 parameters were supplied
[00:31:23]    --> /checkout/src/librustdoc/core.rs:158:55
[00:31:23]     |
[00:31:23] 158 |     let krate = panictry!(driver::phase_1_parse_input(&sess, &input));
[00:31:23]     |                                                       ^^^^^^^^^^^^^ expected 3 parameters
[00:31:23] 
[00:31:25] error[E0061]: this function takes 3 parameters but 2 parameters were supplied
[00:31:25]   --> /checkout/src/librustdoc/test.rs:94:55
[00:31:25]    |
[00:31:25] 94 |     let krate = panictry!(driver::phase_1_parse_input(&sess, &input));
[00:31:25]    |                                                       ^^^^^^^^^^^^^ expected 3 parameters
[00:31:25] 
[00:31:25] error: aborting due to 2 previous errors
