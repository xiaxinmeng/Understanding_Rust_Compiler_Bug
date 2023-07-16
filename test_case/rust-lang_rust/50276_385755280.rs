plain
[00:30:24]    Compiling getopts v0.2.17
[00:30:27]    Compiling rand v0.4.2
[00:30:32]    Compiling tempdir v0.3.7
[00:30:33]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:30:36] error[E0277]: the trait bound `for<'r> F: std::ops::Fn<(&'r mut getopts::Options,)>` is not satisfied
[00:30:36]     |
[00:30:36]     |
[00:30:36] 127 |     RustcOptGroup::stable(name, f)
[00:30:36]     |     ^^^^^^^^^^^^^^^^^^^^^ the trait `for<'r> std::ops::Fn<(&'r mut getopts::Options,)>` is not implemented for `F`
[00:30:36]     |
[00:30:36]     = help: consider adding a `where for<'r> F: std::ops::Fn<(&'r mut getopts::Options,)>` bound
[00:30:36]     = note: required by `rustc::session::config::RustcOptGroup::stable`
[00:30:36] 
[00:30:36] error[E0277]: the trait bound `for<'r> F: std::ops::Fn<(&'r mut getopts::Options,)>` is not satisfied
[00:30:36]     |
[00:30:36]     |
[00:30:36] 133 |     RustcOptGroup::unstable(name, f)
[00:30:36]     |     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'r> std::ops::Fn<(&'r mut getopts::Options,)>` is not implemented for `F`
[00:30:36]     |
[00:30:36]     = help: consider adding a `where for<'r> F: std::ops::Fn<(&'r mut getopts::Options,)>` bound
[00:30:36]     = note: required by `rustc::session::config::RustcOptGroup::unstable`
[00:30:36] error[E0308]: mismatched types
[00:30:36]    --> librustdoc/lib.rs:306:24
[00:30:36]     |
[00:30:36]     |
[00:30:36] 306 |         (option.apply)(&mut options);
[00:30:36]     |                        ^^^^^^^^^^^^ expected struct `getopts::Options`, found a different struct `getopts::Options`
[00:30:36]     |
[00:30:36]     = note: expected type `&mut getopts::Options` (struct `getopts::Options`)
[00:30:36]                found type `&mut getopts::Options` (struct `getopts::Options`)
[00:30:36] note: Perhaps two different versions of crate `getopts` are being used?
[00:30:36]     |
[00:30:36]     |
[00:30:36] 306 |         (option.apply)(&mut options);
[00:30:36] 
[00:30:37] error[E0308]: mismatched types
[00:30:37]    --> librustdoc/lib.rs:314:24
[00:30:37]     |
[00:30:37]     |
[00:30:37] 314 |         (option.apply)(&mut options);
[00:30:37]     |                        ^^^^^^^^^^^^ expected struct `getopts::Options`, found a different struct `getopts::Options`
[00:30:37]     |
[00:30:37]     = note: expected type `&mut getopts::Options` (struct `getopts::Options`)
[00:30:37]                found type `&mut getopts::Options` (struct `getopts::Options`)
[00:30:37] note: Perhaps two different versions of crate `getopts` are being used?
[00:30:37]     |
[00:30:37]     |
[00:30:37] 314 |         (option.apply)(&mut options);
[00:30:37] 
[00:30:37] error[E0308]: mismatched types
[00:30:37]    --> librustdoc/lib.rs:324:44
[00:30:37]     |
[00:30:37]     |
[00:30:37] 324 |     nightly_options::check_nightly_options(&matches, &opts());
[00:30:37]     |                                            ^^^^^^^^ expected struct `getopts::Matches`, found a different struct `getopts::Matches`
[00:30:37]     |
[00:30:37]     = note: expected type `&getopts::Matches` (struct `getopts::Matches`)
[00:30:37]                found type `&getopts::Matches` (struct `getopts::Matches`)
[00:30:37] note: Perhaps two different versions of crate `getopts` are being used?
[00:30:37]     |
[00:30:37]     |
[00:30:37] 324 |     nightly_options::check_nightly_options(&matches, &opts());
[00:30:37] 
[00:30:38] error[E0308]: mismatched types
[00:30:38]    --> librustdoc/lib.rs:333:42
[00:30:38]     |
[00:30:38]     |
[00:30:38] 333 |         rustc_driver::version("rustdoc", &matches);
[00:30:38]     |                                          ^^^^^^^^ expected struct `getopts::Matches`, found a different struct `getopts::Matches`
[00:30:38]     |
[00:30:38]     = note: expected type `&getopts::Matches` (struct `getopts::Matches`)
[00:30:38]                found type `&getopts::Matches` (struct `getopts::Matches`)
[00:30:38] note: Perhaps two different versions of crate `getopts` are being used?
[00:30:38]     |
[00:30:38]     |
[00:30:38] 333 |         rustc_driver::version("rustdoc", &matches);
[00:30:38] 
[00:30:38] error[E0308]: mismatched types
[00:30:38]    --> librustdoc/lib.rs:491:36
[00:30:38]     |
[00:30:38]     |
[00:30:38] 491 |     let cg = build_codegen_options(&matches, ErrorOutputType::default());
[00:30:38]     |                                    ^^^^^^^^ expected struct `getopts::Matches`, found a different struct `getopts::Matches`
[00:30:38]     |
[00:30:38]     = note: expected type `&getopts::Matches` (struct `getopts::Matches`)
[00:30:38]                found type `&getopts::Matches` (struct `getopts::Matches`)
[00:30:38] note: Perhaps two different versions of crate `getopts` are being used?
[00:30:38]     |
[00:30:38]     |
[00:30:38] 491 |     let cg = build_codegen_options(&matches, ErrorOutputType::default());
[00:30:38] 
travis_time:start:3bef11c8
