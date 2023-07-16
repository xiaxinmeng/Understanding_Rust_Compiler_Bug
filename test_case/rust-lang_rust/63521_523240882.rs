plain
2019-08-20T22:18:07.4130995Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T22:18:07.4131350Z 
2019-08-20T22:18:07.4131912Z   git checkout -b <new-branch-name>
2019-08-20T22:18:07.4132227Z 
2019-08-20T22:18:07.4132884Z HEAD is now at b7aa38b04 Auto merge of #63521 - newpavlov:redox_builder, r=pietroalbini
2019-08-20T22:18:07.4287410Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T22:18:07.4290999Z ==============================================================================
2019-08-20T22:18:07.4291097Z Task         : Bash
2019-08-20T22:18:07.4291192Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T23:59:17.3561875Z [RUSTC-TIMING] proc_macro test:false 17.615
2019-08-20T23:59:17.3571916Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-08-20T23:59:17.3854472Z warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-redox`
2019-08-20T23:59:17.3855356Z 
2019-08-20T23:59:17.4031108Z error[E0428]: the name `stdout_isatty` is defined multiple times
2019-08-20T23:59:17.4032151Z    --> src/libtest/lib.rs:977:1
2019-08-20T23:59:17.4032921Z     |
2019-08-20T23:59:17.4033476Z 972 | fn stdout_isatty() -> bool {
2019-08-20T23:59:17.4034091Z     | -------------------------- previous definition of the value `stdout_isatty` here
2019-08-20T23:59:17.4034589Z ...
2019-08-20T23:59:17.4035110Z 977 | fn stdout_isatty() -> bool {
2019-08-20T23:59:17.4035791Z     | ^^^^^^^^^^^^^^^^^^^^^^^^^^ `stdout_isatty` redefined here
2019-08-20T23:59:17.4036869Z     |
2019-08-20T23:59:17.4037615Z     = note: `stdout_isatty` must be defined only once in the value namespace of this module
2019-08-20T23:59:17.8193248Z error: aborting due to previous error
2019-08-20T23:59:17.8193380Z 
2019-08-20T23:59:17.8193770Z For more information about this error, try `rustc --explain E0428`.
2019-08-20T23:59:17.8310172Z [RUSTC-TIMING] test test:false 0.470
---
2019-08-20T23:59:17.8458584Z == clock drift check ==
2019-08-20T23:59:17.8486256Z   local time: Tue Aug 20 23:59:17 UTC 2019
2019-08-20T23:59:17.9018557Z   network time: Tue, 20 Aug 2019 23:59:17 GMT
2019-08-20T23:59:17.9023415Z == end clock drift check ==
2019-08-20T23:59:19.4146673Z ##[error]Bash exited with code '1'.
2019-08-20T23:59:19.4190269Z ##[section]Starting: Upload CPU usage statistics
2019-08-20T23:59:19.4198641Z ==============================================================================
2019-08-20T23:59:19.4198758Z Task         : Bash
2019-08-20T23:59:19.4198854Z Description  : Run a Bash script on macOS, Linux, or Windows
