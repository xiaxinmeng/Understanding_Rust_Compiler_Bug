plain
2019-08-21T17:06:34.6906271Z [RUSTC-TIMING] proc_macro test:false 17.622
2019-08-21T17:06:34.6919510Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-08-21T17:06:34.7189451Z warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-redox`
2019-08-21T17:06:34.7190269Z 
2019-08-21T17:06:34.7807743Z error[E0425]: cannot find value `_SC_NPROCESSORS_ONLN` in crate `libc`
2019-08-21T17:06:34.7809963Z     --> src/libtest/lib.rs:1221:38
2019-08-21T17:06:34.7810726Z      |
2019-08-21T17:06:34.7811711Z 1221 |         unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) as usize }
2019-08-21T17:06:34.7812213Z      |                                      ^^^^^^^^^^^^^^^^^^^^ not found in `libc`
2019-08-21T17:06:36.2765979Z error: aborting due to previous error
2019-08-21T17:06:36.2766167Z 
2019-08-21T17:06:36.2766481Z For more information about this error, try `rustc --explain E0425`.
2019-08-21T17:06:36.2766755Z [RUSTC-TIMING] test test:false 0.476
---
2019-08-21T17:06:36.2770128Z == clock drift check ==
2019-08-21T17:06:36.2770201Z   local time: Wed Aug 21 17:06:35 UTC 2019
2019-08-21T17:06:36.2770290Z   network time: Wed, 21 Aug 2019 17:06:35 GMT
2019-08-21T17:06:36.2770360Z == end clock drift check ==
2019-08-21T17:06:36.6064357Z ##[error]Bash exited with code '1'.
2019-08-21T17:06:36.6116931Z ##[section]Starting: Upload CPU usage statistics
2019-08-21T17:06:36.6123474Z ==============================================================================
2019-08-21T17:06:36.6123584Z Task         : Bash
2019-08-21T17:06:36.6123656Z Description  : Run a Bash script on macOS, Linux, or Windows
