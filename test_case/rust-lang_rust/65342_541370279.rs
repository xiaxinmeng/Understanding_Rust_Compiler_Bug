plain
2019-10-12T23:45:30.8764045Z test num::u16::tests::test_unsigned_checked_div ... ok
2019-10-12T23:45:30.8764153Z test num::u32::tests::test_be ... ok
2019-10-12T23:45:30.8764235Z test num::u32::tests::test_bitwise_operators ... ok
2019-10-12T23:45:30.8764350Z test num::u32::tests::test_count_ones ... ok
2019-10-12T23:45:30.9949497Z error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/deps/coretests-986702cfb203be05` (signal: 11, SIGSEGV: invalid memory reference)
2019-10-12T23:45:30.9966468Z 
2019-10-12T23:45:30.9967199Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "i686-unknown-linux-musl" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "core" "--"
2019-10-12T23:45:30.9967374Z expected success, got: exit code: 101
2019-10-12T23:45:30.9967420Z 
2019-10-12T23:45:30.9967420Z 
2019-10-12T23:45:30.9967450Z 
2019-10-12T23:45:30.9975390Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-10-12T23:45:30.9975946Z Build completed unsuccessfully in 1:36:02
2019-10-12T23:45:31.0027542Z == clock drift check ==
2019-10-12T23:45:31.0042847Z   local time: Sat Oct 12 23:45:31 UTC 2019
2019-10-12T23:45:31.1659663Z   network time: Sat, 12 Oct 2019 23:45:31 GMT
2019-10-12T23:45:31.1665399Z == end clock drift check ==
2019-10-12T23:45:31.6136817Z ##[error]Bash exited with code '1'.
2019-10-12T23:45:31.6185154Z ##[section]Starting: Upload CPU usage statistics
2019-10-12T23:45:31.6216776Z ==============================================================================
2019-10-12T23:45:31.6216877Z Task         : Bash
2019-10-12T23:45:31.6216937Z Description  : Run a Bash script on macOS, Linux, or Windows
