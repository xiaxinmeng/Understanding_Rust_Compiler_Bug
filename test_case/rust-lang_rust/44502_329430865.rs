
[01:06:59]    Compiling rustc_driver v0.0.0 (file:///checkout/src/librustc_driver)
[01:07:01] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[01:07:01]    --> /checkout/src/librustc_driver/test.rs:108:40
[01:07:01]     |
[01:07:01] 108 |     let sess = session::build_session_(options,
[01:07:01]     |                                        ^^^^^^^ expected 4 parameters
[01:07:01]
[01:07:02] error: aborting due to previous error
[01:07:02]
[01:07:02] error: Could not compile `rustc_driver`.
[01:07:02] warning: build failed, waiting for other jobs to finish...
[01:08:24] error: build failed
