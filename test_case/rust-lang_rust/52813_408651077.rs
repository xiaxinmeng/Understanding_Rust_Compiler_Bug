plain
[00:03:58]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:59]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:00]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:04:01]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:02] error[E0599]: no method named `is_sign_negative` found for type `time::Duration` in the current scope
[00:04:02]     |
[00:04:02]     |
[00:04:02] 65  | pub struct Duration {
[00:04:02]     | ------------------- method `is_sign_negative` not found for this
[00:04:02] ...
[00:04:02] 544 |         if rhs.is_sign_negative() {
[00:04:02] 
[00:04:03] error: aborting due to previous error
[00:04:03] 
[00:04:03] For more information about this error, try `rustc --explain E0599`.
---
156608 ./.git/modules/src
149124 ./src/llvm-emscripten/test
145504 ./obj/build/bootstrap/debug/incremental
130636 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs
130632 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs/s-f3cg5qdtg9-1vl24hh-11rkeiafedqcr
97532 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
77624 ./.git/modules/src/tools
71508 ./src/llvm/lib
70304 ./obj/build/x86_64-unknown-linux-gnu/native
