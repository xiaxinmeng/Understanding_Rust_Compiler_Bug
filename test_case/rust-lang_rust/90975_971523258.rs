plain
test net::udp::tests::test_timeout_zero_duration ... ok
test net::udp::tests::timeouts ... ok
test net::udp::tests::ttl ... ok
test net::tcp::tests::write_close ... ok
fatal runtime error: assertion failed: thread_info.is_none()
test num::tests::test_checked_add ... ok
test net::udp::tests::udp_clone_two_read ... ok
test net::udp::tests::socket_smoke_test_ip4 ... ok
test num::tests::test_checked_mul ... ok
test num::tests::test_checked_mul ... ok
test num::tests::test_checked_next_power_of_two_u16 ... ok
test num::tests::test_checked_next_power_of_two_u64 ... ok
test num::tests::test_checked_next_power_of_two_uint ... ok
test num::tests::test_checked_next_power_of_two_u32 ... ok
test num::tests::test_checked_sub ... ok
test net::udp::tests::udp_clone_two_write ... ok
test num::tests::test_is_power_of_two_u32 ... ok
error: test failed, to rerun pass '-p std --lib'
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/aarch64-unknown-linux-gnu/stage1-std/aarch64-unknown-linux-gnu/release/deps/std-00fa684f0f9f00a2` (signal: 6, SIGABRT: process abort signal)


command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "14" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


Build completed unsuccessfully in 0:32:52
